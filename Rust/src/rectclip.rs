//! Optimized rectangle clipping functionality  
//!
//! This module provides specialized, high-performance clipping operations 
//! for rectangular clip regions. It's much faster than the general-purpose 
//! polygon clipping engine for rectangular clipping operations.

use crate::point::Point64;
use crate::path::{Path64, Paths64};

/// Specialized rectangular clipper for fast rectangle-against-polygon clipping
pub struct RectClip64 {
    /// The clipping rectangle (defined by min/max coordinates)
    rect: (Point64, Point64), // (min, max)
}

/// Location codes for Cohen-Sutherland line clipping algorithm
#[derive(Debug, Clone, Copy, PartialEq)]
struct OutCode {
    bits: u8,
}

impl OutCode {
    const INSIDE: OutCode = OutCode { bits: 0 }; // 0000
    const LEFT: OutCode = OutCode { bits: 1 };   // 0001
    const RIGHT: OutCode = OutCode { bits: 2 };  // 0010
    const BOTTOM: OutCode = OutCode { bits: 4 }; // 0100
    const TOP: OutCode = OutCode { bits: 8 };    // 1000

    fn is_inside(self) -> bool {
        self.bits == 0
    }

    fn intersects(self, other: OutCode) -> bool {
        (self.bits & other.bits) == 0
    }
}

impl std::ops::BitOr for OutCode {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        OutCode { bits: self.bits | rhs.bits }
    }
}

impl std::ops::BitAnd for OutCode {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        OutCode { bits: self.bits & rhs.bits }
    }
}

impl RectClip64 {
    /// Creates a new RectClip64 with the specified rectangle bounds
    ///
    /// # Arguments
    /// * `left` - Left edge of clipping rectangle
    /// * `top` - Top edge of clipping rectangle  
    /// * `right` - Right edge of clipping rectangle
    /// * `bottom` - Bottom edge of clipping rectangle
    ///
    /// # Returns
    /// A new `RectClip64` instance
    ///
    /// # Example
    /// ```
    /// use clipper2::RectClip64;
    /// let rect_clipper = RectClip64::new(0, 0, 100, 100);
    /// ```
    pub fn new(left: i64, top: i64, right: i64, bottom: i64) -> Self {
        let min_x = left.min(right);
        let max_x = left.max(right);
        let min_y = top.min(bottom);
        let max_y = top.max(bottom);
        
        Self {
            rect: (Point64::new(min_x, min_y), Point64::new(max_x, max_y)),
        }
    }

    /// Creates a RectClip64 from a rectangle defined by two corner points
    ///
    /// # Arguments
    /// * `corner1` - First corner of the rectangle
    /// * `corner2` - Opposite corner of the rectangle
    ///
    /// # Returns
    /// A new `RectClip64` instance
    pub fn from_points(corner1: Point64, corner2: Point64) -> Self {
        Self::new(corner1.x, corner1.y, corner2.x, corner2.y)
    }

    /// Gets the clipping rectangle bounds
    ///
    /// # Returns
    /// A tuple containing (min_point, max_point) of the clipping rectangle
    pub fn bounds(&self) -> (Point64, Point64) {
        self.rect
    }

    /// Clips a single path against the rectangle
    ///
    /// # Arguments
    /// * `path` - The path to clip
    ///
    /// # Returns
    /// A `Paths64` containing the clipped result (may be multiple disconnected paths)
    ///
    /// # Example
    /// ```
    /// use clipper2::{RectClip64, MakePath};
    /// let rect_clipper = RectClip64::new(10, 10, 90, 90);
    /// let path = MakePath(&[0, 50, 100, 50, 50, 0, 50, 100]);
    /// let result = rect_clipper.clip_path(&path);
    /// ```
    pub fn clip_path(&self, path: &Path64) -> Paths64 {
        if path.len() < 2 {
            return Paths64::new();
        }

        // For closed paths (polygons), use Sutherland-Hodgman algorithm
        if path.len() >= 3 && path.first() == path.last() {
            return self.clip_polygon_sutherland_hodgman(path);
        }

        // For open paths (lines), use Cohen-Sutherland algorithm
        self.clip_lines_cohen_sutherland(path)
    }

    /// Clips multiple paths against the rectangle
    ///
    /// # Arguments
    /// * `paths` - The paths to clip
    ///
    /// # Returns
    /// A `Paths64` containing all clipped results
    pub fn clip_paths(&self, paths: &Paths64) -> Paths64 {
        let mut result = Paths64::new();
        
        for path in paths {
            let clipped = self.clip_path(path);
            result.extend(clipped);
        }
        
        result
    }

    /// Specialized line clipping using Cohen-Sutherland algorithm
    fn clip_lines_cohen_sutherland(&self, path: &Path64) -> Paths64 {
        let mut result = Paths64::new();
        let mut current_segment = Path64::new();
        
        for i in 0..path.len() - 1 {
            let p1 = path[i];
            let p2 = path[i + 1];
            
            if let Some((clipped_p1, clipped_p2)) = self.clip_line_segment(p1, p2) {
                // Check if this segment connects to the previous one
                if current_segment.is_empty() || current_segment.last() != Some(&clipped_p1) {
                    // Start new segment
                    if !current_segment.is_empty() {
                        result.push(current_segment);
                    }
                    current_segment = vec![clipped_p1, clipped_p2];
                } else {
                    // Continue current segment
                    current_segment.push(clipped_p2);
                }
            } else {
                // Segment is completely outside - finish current segment if any
                if !current_segment.is_empty() {
                    result.push(current_segment);
                    current_segment = Path64::new();
                }
            }
        }
        
        // Add final segment if any
        if !current_segment.is_empty() {
            result.push(current_segment);
        }
        
        result
    }

    /// Clips a line segment against the rectangle using Cohen-Sutherland algorithm
    fn clip_line_segment(&self, mut p1: Point64, mut p2: Point64) -> Option<(Point64, Point64)> {
        let mut outcode1 = self.compute_outcode(p1);
        let mut outcode2 = self.compute_outcode(p2);
        
        loop {
            if outcode1.is_inside() && outcode2.is_inside() {
                // Both points inside
                return Some((p1, p2));
            } else if !outcode1.intersects(outcode2) {
                // Both points outside same region
                return None;
            } else {
                // At least one point outside, need to clip
                let outcode_out = if !outcode1.is_inside() { outcode1 } else { outcode2 };
                let (x, y) = self.compute_intersection(p1, p2, outcode_out)?;
                
                if outcode_out.bits == outcode1.bits {
                    p1 = Point64::new(x, y);
                    outcode1 = self.compute_outcode(p1);
                } else {
                    p2 = Point64::new(x, y);
                    outcode2 = self.compute_outcode(p2);
                }
            }
        }
    }

    /// Compute Cohen-Sutherland outcode for a point
    fn compute_outcode(&self, point: Point64) -> OutCode {
        let mut code = OutCode::INSIDE;
        
        if point.x < self.rect.0.x {
            code = code | OutCode::LEFT;
        } else if point.x > self.rect.1.x {
            code = code | OutCode::RIGHT;
        }
        
        if point.y < self.rect.0.y {
            code = code | OutCode::BOTTOM;
        } else if point.y > self.rect.1.y {
            code = code | OutCode::TOP;
        }
        
        code
    }

    /// Compute intersection point with rectangle edge
    fn compute_intersection(&self, p1: Point64, p2: Point64, outcode: OutCode) -> Option<(i64, i64)> {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        
        if (outcode & OutCode::TOP).bits != 0 {
            // Intersection with top edge
            if dy == 0 { return None; }
            let x = p1.x + dx * (self.rect.1.y - p1.y) / dy;
            return Some((x, self.rect.1.y));
        } else if (outcode & OutCode::BOTTOM).bits != 0 {
            // Intersection with bottom edge  
            if dy == 0 { return None; }
            let x = p1.x + dx * (self.rect.0.y - p1.y) / dy;
            return Some((x, self.rect.0.y));
        } else if (outcode & OutCode::RIGHT).bits != 0 {
            // Intersection with right edge
            if dx == 0 { return None; }
            let y = p1.y + dy * (self.rect.1.x - p1.x) / dx;
            return Some((self.rect.1.x, y));
        } else if (outcode & OutCode::LEFT).bits != 0 {
            // Intersection with left edge
            if dx == 0 { return None; }
            let y = p1.y + dy * (self.rect.0.x - p1.x) / dx;
            return Some((self.rect.0.x, y));
        }
        
        None
    }

    /// Polygon clipping using Sutherland-Hodgman algorithm
    fn clip_polygon_sutherland_hodgman(&self, polygon: &Path64) -> Paths64 {
        if polygon.len() < 3 {
            return Paths64::new();
        }

        let mut clipped = polygon.clone();
        
        // Remove last point if it duplicates the first (closing point)
        if clipped.len() > 3 && clipped.first() == clipped.last() {
            clipped.pop();
        }
        
        // Clip against each edge of the rectangle
        clipped = self.clip_against_edge(clipped, Edge::Left);
        if clipped.is_empty() { return Paths64::new(); }
        
        clipped = self.clip_against_edge(clipped, Edge::Right);
        if clipped.is_empty() { return Paths64::new(); }
        
        clipped = self.clip_against_edge(clipped, Edge::Bottom);
        if clipped.is_empty() { return Paths64::new(); }
        
        clipped = self.clip_against_edge(clipped, Edge::Top);
        if clipped.is_empty() { return Paths64::new(); }
        
        if clipped.len() >= 3 {
            vec![clipped]
        } else {
            Paths64::new()
        }
    }

    /// Clip polygon against one edge of the rectangle
    fn clip_against_edge(&self, polygon: Path64, edge: Edge) -> Path64 {
        if polygon.is_empty() {
            return Path64::new();
        }
        
        let mut result = Path64::new();
        let mut prev_point = *polygon.last().unwrap();
        let mut prev_inside = self.point_inside_edge(prev_point, edge);
        
        for &current_point in &polygon {
            let current_inside = self.point_inside_edge(current_point, edge);
            
            if current_inside {
                if !prev_inside {
                    // Entering: add intersection point
                    if let Some(intersection) = self.intersect_edge(prev_point, current_point, edge) {
                        result.push(intersection);
                    }
                }
                // Inside: add current point
                result.push(current_point);
            } else if prev_inside {
                // Exiting: add intersection point
                if let Some(intersection) = self.intersect_edge(prev_point, current_point, edge) {
                    result.push(intersection);
                }
            }
            // Outside: don't add anything
            
            prev_point = current_point;
            prev_inside = current_inside;
        }
        
        result
    }

    /// Check if point is inside (on the correct side of) an edge
    fn point_inside_edge(&self, point: Point64, edge: Edge) -> bool {
        match edge {
            Edge::Left => point.x >= self.rect.0.x,
            Edge::Right => point.x <= self.rect.1.x,
            Edge::Bottom => point.y >= self.rect.0.y,
            Edge::Top => point.y <= self.rect.1.y,
        }
    }

    /// Compute intersection with a rectangle edge
    fn intersect_edge(&self, p1: Point64, p2: Point64, edge: Edge) -> Option<Point64> {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        
        match edge {
            Edge::Left => {
                if dx == 0 { return None; }
                let y = p1.y + dy * (self.rect.0.x - p1.x) / dx;
                Some(Point64::new(self.rect.0.x, y))
            }
            Edge::Right => {
                if dx == 0 { return None; }
                let y = p1.y + dy * (self.rect.1.x - p1.x) / dx;
                Some(Point64::new(self.rect.1.x, y))
            }
            Edge::Bottom => {
                if dy == 0 { return None; }
                let x = p1.x + dx * (self.rect.0.y - p1.y) / dy;
                Some(Point64::new(x, self.rect.0.y))
            }
            Edge::Top => {
                if dy == 0 { return None; }
                let x = p1.x + dx * (self.rect.1.y - p1.y) / dy;
                Some(Point64::new(x, self.rect.1.y))
            }
        }
    }
}

/// Rectangle edges for Sutherland-Hodgman clipping
#[derive(Debug, Clone, Copy)]
enum Edge {
    Left,
    Right, 
    Bottom,
    Top,
}

/// Specialized line clipper for clipping lines against rectangles
pub struct RectClipLines64 {
    rect_clipper: RectClip64,
}

impl RectClipLines64 {
    /// Creates a new RectClipLines64
    pub fn new(left: i64, top: i64, right: i64, bottom: i64) -> Self {
        Self {
            rect_clipper: RectClip64::new(left, top, right, bottom),
        }
    }

    /// Clips multiple line paths against the rectangle
    pub fn clip_lines(&self, paths: &Paths64) -> Paths64 {
        let mut result = Paths64::new();
        
        for path in paths {
            if path.len() >= 2 {
                let clipped = self.rect_clipper.clip_lines_cohen_sutherland(path);
                result.extend(clipped);
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point64;

    #[test]
    fn test_rectclip64_creation() {
        let clipper = RectClip64::new(0, 0, 100, 100);
        let (min, max) = clipper.bounds();
        
        assert_eq!(min, Point64::new(0, 0));
        assert_eq!(max, Point64::new(100, 100));
    }

    #[test]
    fn test_rectclip64_from_points() {
        let clipper = RectClip64::from_points(
            Point64::new(50, 25),
            Point64::new(10, 75)
        );
        let (min, max) = clipper.bounds();
        
        // Should normalize to min/max coordinates
        assert_eq!(min, Point64::new(10, 25));
        assert_eq!(max, Point64::new(50, 75));
    }

    #[test]
    fn test_outcode_computation() {
        let clipper = RectClip64::new(10, 10, 90, 90);
        
        // Inside point
        let inside_code = clipper.compute_outcode(Point64::new(50, 50));
        assert!(inside_code.is_inside());
        
        // Outside points
        let left_code = clipper.compute_outcode(Point64::new(5, 50));
        assert_eq!(left_code & OutCode::LEFT, OutCode::LEFT);
        
        let right_code = clipper.compute_outcode(Point64::new(95, 50));
        assert_eq!(right_code & OutCode::RIGHT, OutCode::RIGHT);
        
        let top_code = clipper.compute_outcode(Point64::new(50, 95));
        assert_eq!(top_code & OutCode::TOP, OutCode::TOP);
        
        let bottom_code = clipper.compute_outcode(Point64::new(50, 5));
        assert_eq!(bottom_code & OutCode::BOTTOM, OutCode::BOTTOM);
    }

    #[test]
    fn test_line_segment_clipping() {
        let clipper = RectClip64::new(10, 10, 90, 90);
        
        // Line completely inside
        let inside_line = clipper.clip_line_segment(
            Point64::new(20, 20),
            Point64::new(80, 80)
        );
        assert!(inside_line.is_some());
        assert_eq!(inside_line.unwrap(), (Point64::new(20, 20), Point64::new(80, 80)));
        
        // Line completely outside
        let outside_line = clipper.clip_line_segment(
            Point64::new(0, 0),
            Point64::new(5, 5)
        );
        assert!(outside_line.is_none());
        
        // Line crossing rectangle
        let crossing_line = clipper.clip_line_segment(
            Point64::new(0, 50),
            Point64::new(100, 50)
        );
        assert!(crossing_line.is_some());
        let (p1, p2) = crossing_line.unwrap();
        assert_eq!(p1, Point64::new(10, 50));
        assert_eq!(p2, Point64::new(90, 50));
    }

    #[test]
    fn test_simple_path_clipping() {
        let clipper = RectClip64::new(10, 10, 90, 90);
        
        // Simple line path that crosses the rectangle
        let path = vec![
            Point64::new(0, 50),
            Point64::new(50, 50),
            Point64::new(100, 50),
        ];
        
        let result = clipper.clip_path(&path);
        assert!(!result.is_empty());
        
        // Should produce at least one clipped segment
        if !result.is_empty() {
            let clipped_path = &result[0];
            assert!(clipped_path.len() >= 2);
            
            // All points should be within or on the rectangle bounds
            for point in clipped_path {
                assert!(point.x >= 10 && point.x <= 90);
                assert!(point.y >= 10 && point.y <= 90);
            }
        }
    }

    #[test]
    fn test_rectangle_polygon_clipping() {
        let clipper = RectClip64::new(25, 25, 75, 75);
        
        // Square that partially overlaps the clipping rectangle
        let square = vec![
            Point64::new(0, 0),
            Point64::new(50, 0),
            Point64::new(50, 50),
            Point64::new(0, 50),
            Point64::new(0, 0), // Closing point
        ];
        
        let result = clipper.clip_path(&square);
        
        // Should produce a result (the overlapping region)
        assert!(!result.is_empty());
        
        if !result.is_empty() {
            let clipped = &result[0];
            assert!(clipped.len() >= 3); // Should be a valid polygon
            
            // All points should be within the clipping rectangle
            for point in clipped {
                assert!(point.x >= 25 && point.x <= 75);
                assert!(point.y >= 25 && point.y <= 75);
            }
        }
    }

    #[test]
    fn test_rectcliplines64() {
        let line_clipper = RectClipLines64::new(10, 10, 90, 90);
        
        let paths = vec![
            vec![Point64::new(0, 50), Point64::new(100, 50)],  // Horizontal line
            vec![Point64::new(50, 0), Point64::new(50, 100)],  // Vertical line
        ];
        
        let result = line_clipper.clip_lines(&paths);
        
        // Should clip both lines to the rectangle bounds
        assert!(!result.is_empty());
        
        for path in &result {
            for point in path {
                assert!(point.x >= 10 && point.x <= 90);
                assert!(point.y >= 10 && point.y <= 90);
            }
        }
    }
}