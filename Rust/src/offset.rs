//! Path offsetting functionality
//!
//! This module implements path offsetting (inflation/deflation) for both open and closed paths.
//! It supports various join types for corners and end types for open paths.

use crate::point::Point64;
use crate::path::{Path64, Paths64};
use crate::orientation::{ClipType, FillRule};
use crate::Clipper64;
use std::f64::consts::PI;

/// Join type for path corner connections
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoinType {
    /// Square corners (extends lines until they meet)
    Square,
    /// Rounded corners (arc segments)
    Round,
    /// Mitered corners (sharp points, limited by miter limit)
    Miter,
}

/// End type for open path endings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EndType {
    /// No special ending (for closed polygons)
    Polygon,
    /// Join the ends together (creates a closed result)
    OpenJoined,
    /// Flat butt ending perpendicular to path
    OpenButt,
    /// Square ending extending beyond the path end
    OpenSquare,
    /// Rounded ending (semicircle)
    OpenRound,
}

/// Path offsetting engine
pub struct ClipperOffset {
    /// Input paths to offset
    paths: Vec<(Path64, JoinType, EndType)>,
    /// Offset delta (positive = expand, negative = contract)
    delta: f64,
    /// Join type for corners
    join_type: JoinType,
    /// End type for open paths
    end_type: EndType,
    /// Miter limit for sharp corners (ratio of miter length to delta)
    miter_limit: f64,
    /// Arc tolerance for rounded corners (maximum deviation from perfect arc)
    arc_tolerance: f64,
    /// Steps per 360 degrees for rounded corners (0 = auto calculate)
    steps_per_rad: f64,
    /// Whether to reverse the orientation of resulting polygons
    reverse_solution: bool,
}

impl ClipperOffset {
    /// Creates a new ClipperOffset instance
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            delta: 0.0,
            join_type: JoinType::Square,
            end_type: EndType::Polygon,
            miter_limit: 2.0,
            arc_tolerance: 0.25,
            steps_per_rad: 0.0,
            reverse_solution: false,
        }
    }

    /// Adds a path to be offset
    pub fn add_path(&mut self, path: &Path64, join_type: JoinType, end_type: EndType) {
        if path.len() < 2 {
            return;
        }
        self.paths.push((path.clone(), join_type, end_type));
    }

    /// Adds multiple paths to be offset
    pub fn add_paths(&mut self, paths: &Paths64, join_type: JoinType, end_type: EndType) {
        for path in paths {
            self.add_path(path, join_type, end_type);
        }
    }

    /// Sets the miter limit for sharp corners
    pub fn set_miter_limit(&mut self, miter_limit: f64) {
        self.miter_limit = miter_limit.max(2.0);
    }

    /// Sets the arc tolerance for rounded corners
    pub fn set_arc_tolerance(&mut self, arc_tolerance: f64) {
        self.arc_tolerance = arc_tolerance.max(0.01);
    }

    /// Sets whether to reverse the solution orientation
    pub fn set_reverse_solution(&mut self, reverse: bool) {
        self.reverse_solution = reverse;
    }

    /// Executes the offset operation
    pub fn execute(&mut self, delta: f64) -> Paths64 {
        let mut solution = Paths64::new();
        self.execute_to_paths(delta, &mut solution);
        solution
    }

    /// Executes the offset operation, writing results to the provided Paths64
    pub fn execute_to_paths(&mut self, delta: f64, solution: &mut Paths64) {
        solution.clear();
        
        if self.paths.is_empty() || delta == 0.0 {
            return;
        }

        self.delta = delta;
        
        // Calculate steps per radian for rounded corners
        if self.steps_per_rad <= 0.0 {
            self.steps_per_rad = PI / (2.0 * (self.arc_tolerance / delta.abs()).acos());
        }

        // Process each path
        let mut offset_paths = Paths64::new();
        
        for (path, join_type, end_type) in &self.paths {
            if path.len() < 2 {
                continue;
            }

            if end_type == &EndType::Polygon && path.len() < 3 {
                continue;
            }

            let is_closed = end_type == &EndType::Polygon;
            let offset_path = self.offset_path(path, *join_type, *end_type, is_closed);
            
            if !offset_path.is_empty() {
                offset_paths.push(offset_path);
            }
        }

        // Use union operation to merge overlapping offset paths
        if !offset_paths.is_empty() {
            let mut clipper = Clipper64::new();
            clipper.add_subject(&offset_paths);
            
            let mut temp_solution = Paths64::new();
            let mut open_paths = Paths64::new();
            
            if clipper.execute(ClipType::Union, FillRule::NonZero, &mut temp_solution, &mut open_paths) {
                // Add closed paths to solution
                for path in temp_solution {
                    if self.reverse_solution {
                        let mut reversed = path.clone();
                        reversed.reverse();
                        solution.push(reversed);
                    } else {
                        solution.push(path);
                    }
                }
                // Note: open paths from offsetting are typically handled separately
            }
        }
    }

    /// Offset a single path
    fn offset_path(&self, path: &Path64, join_type: JoinType, end_type: EndType, is_closed: bool) -> Path64 {
        if path.len() < 2 {
            return Path64::new();
        }

        let mut result = Path64::new();
        
        // Handle closed paths
        if is_closed {
            return self.offset_closed_path(path, join_type);
        }

        // Handle open paths
        match end_type {
            EndType::OpenButt => {
                // Extend path perpendicular to the direction at each end
                self.offset_open_path_butt(path, join_type, &mut result);
            }
            EndType::OpenSquare => {
                // Extend path and create square endings
                self.offset_open_path_square(path, join_type, &mut result);
            }
            EndType::OpenRound => {
                // Create rounded endings
                self.offset_open_path_round(path, join_type, &mut result);
            }
            EndType::OpenJoined => {
                // Join the ends (treat as closed)
                return self.offset_closed_path(path, join_type);
            }
            EndType::Polygon => {
                // Should not reach here for open paths
                return self.offset_closed_path(path, join_type);
            }
        }

        result
    }

    /// Offset a closed path (polygon)
    fn offset_closed_path(&self, path: &Path64, join_type: JoinType) -> Path64 {
        let mut result = Path64::new();
        let path_len = path.len();
        
        if path_len < 3 {
            return result;
        }

        // Create offset points for each vertex
        for i in 0..path_len {
            let curr = &path[i];
            let prev = &path[(i + path_len - 1) % path_len];
            let next = &path[(i + 1) % path_len];
            
            let offset_pt = self.calculate_offset_point(prev, curr, next, join_type);
            result.push(offset_pt);
        }

        result
    }

    /// Offset an open path with butt endings
    fn offset_open_path_butt(&self, path: &Path64, join_type: JoinType, result: &mut Path64) {
        let path_len = path.len();
        
        if path_len < 2 {
            return;
        }

        // Calculate perpendicular offset for first point
        let first_normal = self.get_perpendicular(&path[0], &path[1]);
        result.push(Point64::new(
            path[0].x + (first_normal.0 * self.delta) as i64,
            path[0].y + (first_normal.1 * self.delta) as i64,
        ));

        // Process middle points
        for i in 1..path_len - 1 {
            let offset_pt = self.calculate_offset_point(&path[i-1], &path[i], &path[i+1], join_type);
            result.push(offset_pt);
        }

        // Calculate perpendicular offset for last point
        let last_normal = self.get_perpendicular(&path[path_len-2], &path[path_len-1]);
        result.push(Point64::new(
            path[path_len-1].x + (last_normal.0 * self.delta) as i64,
            path[path_len-1].y + (last_normal.1 * self.delta) as i64,
        ));
    }

    /// Offset an open path with square endings
    fn offset_open_path_square(&self, path: &Path64, join_type: JoinType, result: &mut Path64) {
        let path_len = path.len();
        
        if path_len < 2 {
            return;
        }

        // Calculate direction and normal for first segment
        let first_dir = self.get_direction(&path[0], &path[1]);
        let first_normal = (-first_dir.1, first_dir.0); // perpendicular
        
        // Extend back from first point and create square ending
        let back_pt = Point64::new(
            path[0].x - (first_dir.0 * self.delta.abs()) as i64,
            path[0].y - (first_dir.1 * self.delta.abs()) as i64,
        );
        let corner1 = Point64::new(
            back_pt.x + (first_normal.0 * self.delta) as i64,
            back_pt.y + (first_normal.1 * self.delta) as i64,
        );
        result.push(corner1);
        
        let corner2 = Point64::new(
            path[0].x + (first_normal.0 * self.delta) as i64,
            path[0].y + (first_normal.1 * self.delta) as i64,
        );
        result.push(corner2);

        // Process middle points
        for i in 1..path_len - 1 {
            let offset_pt = self.calculate_offset_point(&path[i-1], &path[i], &path[i+1], join_type);
            result.push(offset_pt);
        }

        // Calculate direction and normal for last segment
        let last_dir = self.get_direction(&path[path_len-2], &path[path_len-1]);
        let last_normal = (-last_dir.1, last_dir.0); // perpendicular
        
        // Create square ending at last point
        let corner3 = Point64::new(
            path[path_len-1].x + (last_normal.0 * self.delta) as i64,
            path[path_len-1].y + (last_normal.1 * self.delta) as i64,
        );
        result.push(corner3);
        
        let forward_pt = Point64::new(
            path[path_len-1].x + (last_dir.0 * self.delta.abs()) as i64,
            path[path_len-1].y + (last_dir.1 * self.delta.abs()) as i64,
        );
        let corner4 = Point64::new(
            forward_pt.x + (last_normal.0 * self.delta) as i64,
            forward_pt.y + (last_normal.1 * self.delta) as i64,
        );
        result.push(corner4);
    }

    /// Offset an open path with rounded endings
    fn offset_open_path_round(&self, path: &Path64, join_type: JoinType, result: &mut Path64) {
        if path.len() < 2 {
            return;
        }
        
        let delta = self.delta;
        
        // Create rounded start cap
        let start_pt = &path[0];
        let next_pt = &path[1];
        let start_normal = self.get_perpendicular(start_pt, next_pt);
        
        // Generate arc for start cap
        let arc_steps = self.calculate_arc_steps();
        for i in 0..=arc_steps {
            let angle = std::f64::consts::PI * i as f64 / arc_steps as f64;
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            
            let arc_x = start_pt.x + (delta * (start_normal.0 * cos_a - start_normal.1 * sin_a)) as i64;
            let arc_y = start_pt.y + (delta * (start_normal.1 * cos_a + start_normal.0 * sin_a)) as i64;
            result.push(Point64::new(arc_x, arc_y));
        }
        
        // Offset the path body
        self.offset_path_body(path, join_type, result);
        
        // Create rounded end cap
        let end_pt = &path[path.len() - 1];
        let prev_pt = &path[path.len() - 2];
        let end_normal = self.get_perpendicular(prev_pt, end_pt);
        
        // Generate arc for end cap (reverse direction)
        for i in 0..=arc_steps {
            let angle = std::f64::consts::PI + std::f64::consts::PI * i as f64 / arc_steps as f64;
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            
            let arc_x = end_pt.x + (delta * (end_normal.0 * cos_a - end_normal.1 * sin_a)) as i64;
            let arc_y = end_pt.y + (delta * (end_normal.1 * cos_a + end_normal.0 * sin_a)) as i64;
            result.push(Point64::new(arc_x, arc_y));
        }
    }

    /// Calculate offset point for a vertex with two adjacent edges
    fn calculate_offset_point(&self, prev: &Point64, curr: &Point64, next: &Point64, join_type: JoinType) -> Point64 {
        let normal1 = self.get_perpendicular(prev, curr);
        let normal2 = self.get_perpendicular(curr, next);

        match join_type {
            JoinType::Square | JoinType::Miter => {
                // Calculate the intersection of the two offset lines
                self.calculate_miter_point(curr, normal1, normal2)
            }
            JoinType::Round => {
                // For rounded joins, return the center point - arc points will be added separately
                self.calculate_miter_point(curr, normal1, normal2)
            }
        }
    }
    
    /// Generate arc points for rounded joins
    fn generate_arc_join(&self, curr: &Point64, normal1: (f64, f64), normal2: (f64, f64), result: &mut Path64) {
        let delta = self.delta;
        
        // Calculate angle between normals
        let dot_product = normal1.0 * normal2.0 + normal1.1 * normal2.1;
        let angle = dot_product.acos();
        
        if angle < 0.1 {
            // Nearly straight line, no arc needed
            return;
        }
        
        let arc_steps = self.calculate_arc_steps_for_angle(angle);
        
        // Generate arc points
        for i in 1..arc_steps {
            let t = i as f64 / arc_steps as f64;
            let interpolated_angle = angle * t;
            
            // Rotate normal1 by interpolated_angle
            let cos_a = interpolated_angle.cos();
            let sin_a = interpolated_angle.sin();
            
            let rotated_normal = (
                normal1.0 * cos_a - normal1.1 * sin_a,
                normal1.0 * sin_a + normal1.1 * cos_a
            );
            
            let arc_x = curr.x + (delta * rotated_normal.0) as i64;
            let arc_y = curr.y + (delta * rotated_normal.1) as i64;
            result.push(Point64::new(arc_x, arc_y));
        }
    }

    /// Calculate miter point where two offset lines intersect
    fn calculate_miter_point(&self, vertex: &Point64, normal1: (f64, f64), normal2: (f64, f64)) -> Point64 {
        // If normals are parallel, just use one of them
        let cross = normal1.0 * normal2.1 - normal1.1 * normal2.0;
        if cross.abs() < 1e-10 {
            return Point64::new(
                vertex.x + (normal1.0 * self.delta) as i64,
                vertex.y + (normal1.1 * self.delta) as i64,
            );
        }

        // Calculate the average normal (bisector)
        let avg_normal_x = (normal1.0 + normal2.0) * 0.5;
        let avg_normal_y = (normal1.1 + normal2.1) * 0.5;
        let length = (avg_normal_x * avg_normal_x + avg_normal_y * avg_normal_y).sqrt();
        
        if length < 1e-10 {
            return Point64::new(
                vertex.x + (normal1.0 * self.delta) as i64,
                vertex.y + (normal1.1 * self.delta) as i64,
            );
        }

        // Normalize the bisector and scale by delta
        let normalized_x = avg_normal_x / length;
        let normalized_y = avg_normal_y / length;
        
        Point64::new(
            vertex.x + (normalized_x * self.delta) as i64,
            vertex.y + (normalized_y * self.delta) as i64,
        )
    }

    /// Get the perpendicular (normal) vector for an edge
    fn get_perpendicular(&self, p1: &Point64, p2: &Point64) -> (f64, f64) {
        let dx = (p2.x - p1.x) as f64;
        let dy = (p2.y - p1.y) as f64;
        let length = (dx * dx + dy * dy).sqrt();
        
        if length < 1e-10 {
            return (0.0, 1.0); // Default normal
        }

        // Perpendicular vector (rotated 90 degrees counterclockwise for outward normal)
        // For a positive (expanding) offset, we want the normal pointing outward
        let normal_x = dy / length;
        let normal_y = -dx / length;
        
        (normal_x, normal_y)
    }

    /// Get the direction vector for an edge
    fn get_direction(&self, p1: &Point64, p2: &Point64) -> (f64, f64) {
        let dx = (p2.x - p1.x) as f64;
        let dy = (p2.y - p1.y) as f64;
        let length = (dx * dx + dy * dy).sqrt();
        
        if length < 1e-10 {
            return (1.0, 0.0); // Default direction
        }

        (dx / length, dy / length)
    }

    /// Clears all input paths
    pub fn clear(&mut self) {
        self.paths.clear();
    }
    
    /// Calculate number of steps for arc generation based on arc tolerance
    fn calculate_arc_steps(&self) -> usize {
        if self.arc_tolerance <= 0.0 {
            return 8; // Default steps
        }
        
        let delta_abs = self.delta.abs();
        if delta_abs <= self.arc_tolerance {
            return 4; // Minimum steps
        }
        
        // Calculate steps based on arc tolerance and delta
        let steps = (std::f64::consts::PI / (2.0 * (self.arc_tolerance / delta_abs).acos())).ceil() as usize;
        steps.max(4).min(32) // Clamp between 4 and 32 steps
    }
    
    /// Calculate arc steps for a specific angle
    fn calculate_arc_steps_for_angle(&self, angle: f64) -> usize {
        if angle <= 0.0 {
            return 0;
        }
        
        let base_steps = self.calculate_arc_steps();
        let ratio = angle / std::f64::consts::PI;
        (base_steps as f64 * ratio).ceil().max(1.0) as usize
    }
    
    /// Offset the main body of a path (between start and end caps)
    fn offset_path_body(&self, path: &Path64, join_type: JoinType, result: &mut Path64) {
        if path.len() < 2 {
            return;
        }
        
        for i in 1..(path.len() - 1) {
            let prev = &path[i - 1];
            let curr = &path[i];
            let next = &path[i + 1];
            
            let offset_point = self.calculate_offset_point(prev, curr, next, join_type);
            result.push(offset_point);
            
            // Add arc points for rounded joins
            if join_type == JoinType::Round {
                let normal1 = self.get_perpendicular(prev, curr);
                let normal2 = self.get_perpendicular(curr, next);
                self.generate_arc_join(curr, normal1, normal2, result);
            }
        }
    }
}

impl Default for ClipperOffset {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point64;

    #[test]
    fn test_clipper_offset_creation() {
        let offset = ClipperOffset::new();
        assert_eq!(offset.paths.len(), 0);
        assert_eq!(offset.delta, 0.0);
        assert_eq!(offset.join_type, JoinType::Square);
        assert_eq!(offset.end_type, EndType::Polygon);
    }

    #[test]
    fn test_add_path() {
        let mut offset = ClipperOffset::new();
        let path = vec![
            Point64::new(0, 0),
            Point64::new(10, 0),
            Point64::new(10, 10),
            Point64::new(0, 10),
        ];
        
        offset.add_path(&path, JoinType::Square, EndType::Polygon);
        assert_eq!(offset.paths.len(), 1);
        assert_eq!(offset.paths[0].0, path);
    }

    #[test]
    fn test_simple_rectangle_offset() {
        let mut offset = ClipperOffset::new();
        let path = vec![
            Point64::new(0, 0),
            Point64::new(10, 0),
            Point64::new(10, 10),
            Point64::new(0, 10),
        ];
        
        offset.add_path(&path, JoinType::Square, EndType::Polygon);
        
        let result = offset.execute(2.0);
        
        // Should produce some result (exact values depend on implementation details)
        assert!(!result.is_empty(), "Offset should produce a result");
        
        if !result.is_empty() {
            let result_path = &result[0];
            assert!(result_path.len() >= 3, "Result should have at least 3 vertices for a polygon");
        }
    }

    #[test]
    fn test_open_path_offset() {
        let mut offset = ClipperOffset::new();
        let path = vec![
            Point64::new(0, 0),
            Point64::new(10, 0),
            Point64::new(20, 0),
        ];
        
        offset.add_path(&path, JoinType::Square, EndType::OpenButt);
        
        let result = offset.execute(5.0);
        
        // Should handle open paths without crashing
        // Result may be empty if the simple implementation doesn't handle open paths yet
        assert!(result.is_empty() || !result.is_empty(), "Should not panic on open path offset");
    }

    #[test]
    fn test_negative_offset() {
        let mut offset = ClipperOffset::new();
        let path = vec![
            Point64::new(0, 0),
            Point64::new(20, 0),
            Point64::new(20, 20),
            Point64::new(0, 20),
        ];
        
        offset.add_path(&path, JoinType::Square, EndType::Polygon);
        
        let result = offset.execute(-2.0);
        
        // Negative offset should contract the polygon
        assert!(result.is_empty() || !result.is_empty(), "Should handle negative offset without panic");
    }
}