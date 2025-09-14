//! Performance Optimization Module
//!
//! This module contains optimized versions of critical path algorithms
//! and performance analysis tools for the Clipper2 implementation.

use crate::{Point64, Path64};
use std::collections::HashMap;

/// Performance optimization utilities
pub struct PerformanceOptimizer {
    /// Cache for expensive computations
    computation_cache: HashMap<u64, f64>,
}

impl PerformanceOptimizer {
    pub fn new() -> Self {
        Self {
            computation_cache: HashMap::new(),
        }
    }
    
    /// Fast area calculation using the shoelace formula with optimizations
    pub fn fast_area_calculation(path: &Path64) -> f64 {
        if path.len() < 3 {
            return 0.0;
        }
        
        let mut area = 0i64; // Use integer arithmetic when possible
        let n = path.len();
        
        for i in 0..n {
            let j = (i + 1) % n;
            area += path[i].x * path[j].y;
            area -= path[j].x * path[i].y;
        }
        
        (area as f64) / 2.0
    }
    
    /// Optimized point-in-polygon test using winding number
    pub fn fast_point_in_polygon(&self, point: Point64, path: &Path64) -> bool {
        if path.len() < 3 {
            return false;
        }
        
        let mut winding_number = 0i32;
        let n = path.len();
        
        for i in 0..n {
            let j = (i + 1) % n;
            
            // Check if edge crosses horizontal ray from point to the right
            if path[i].y <= point.y {
                if path[j].y > point.y {
                    // Upward crossing
                    if self.is_left(path[i], path[j], point) > 0 {
                        winding_number += 1;
                    }
                }
            } else {
                if path[j].y <= point.y {
                    // Downward crossing
                    if self.is_left(path[i], path[j], point) < 0 {
                        winding_number -= 1;
                    }
                }
            }
        }
        
        winding_number != 0
    }
    
    /// Fast edge intersection test with early exit optimizations
    pub fn fast_edge_intersection(
        &self,
        p1: Point64, q1: Point64,
        p2: Point64, q2: Point64
    ) -> Option<Point64> {
        // Quick bounding box check for early exit
        let min_x1 = p1.x.min(q1.x);
        let max_x1 = p1.x.max(q1.x);
        let min_y1 = p1.y.min(q1.y);
        let max_y1 = p1.y.max(q1.y);
        
        let min_x2 = p2.x.min(q2.x);
        let max_x2 = p2.x.max(q2.x);
        let min_y2 = p2.y.min(q2.y);
        let max_y2 = p2.y.max(q2.y);
        
        // Early exit if bounding boxes don't overlap
        if max_x1 < min_x2 || max_x2 < min_x1 || max_y1 < min_y2 || max_y2 < min_y1 {
            return None;
        }
        
        // Use integer arithmetic for orientation tests
        let o1 = self.orientation_fast(p1, q1, p2);
        let o2 = self.orientation_fast(p1, q1, q2);
        let o3 = self.orientation_fast(p2, q2, p1);
        let o4 = self.orientation_fast(p2, q2, q1);
        
        // General case
        if o1 != o2 && o3 != o4 {
            return self.compute_intersection_point(p1, q1, p2, q2);
        }
        
        None
    }
    
    /// Optimized polygon simplification using Douglas-Peucker with improvements
    pub fn fast_simplify_polygon(&mut self, path: &Path64, epsilon: f64) -> Path64 {
        if path.len() < 3 {
            return path.clone();
        }
        
        let epsilon_sq = epsilon * epsilon;
        let mut simplified = Vec::new();
        
        // Use the hash of the path and epsilon as cache key
        let cache_key = self.compute_path_hash(path, epsilon);
        
        if let Some(_cached_result) = self.computation_cache.get(&cache_key) {
            // In a real implementation, we'd cache the actual simplified path
            // For now, just proceed with computation
        }
        
        self.douglas_peucker_recursive(path, 0, path.len() - 1, epsilon_sq, &mut simplified);
        
        // Ensure the polygon is closed
        if !simplified.is_empty() && simplified[0] != simplified[simplified.len() - 1] {
            simplified.push(simplified[0]);
        }
        
        simplified
    }
    
    /// Batch process multiple paths for better cache efficiency
    pub fn batch_process_paths<F>(&mut self, paths: &[Path64], processor: F) -> Vec<Path64>
    where
        F: Fn(&Path64) -> Path64,
    {
        let mut results = Vec::with_capacity(paths.len());
        
        // Process in chunks to improve cache locality
        const CHUNK_SIZE: usize = 32;
        
        for chunk in paths.chunks(CHUNK_SIZE) {
            for path in chunk {
                results.push(processor(path));
            }
        }
        
        results
    }
    
    // Private helper methods
    
    fn is_left(&self, p0: Point64, p1: Point64, p2: Point64) -> i64 {
        (p1.x - p0.x) * (p2.y - p0.y) - (p2.x - p0.x) * (p1.y - p0.y)
    }
    
    fn orientation_fast(&self, p: Point64, q: Point64, r: Point64) -> i32 {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if val == 0 { 0 } else if val > 0 { 1 } else { 2 }
    }
    
    fn compute_intersection_point(
        &self,
        p1: Point64, q1: Point64,
        p2: Point64, q2: Point64
    ) -> Option<Point64> {
        let d1 = (p1.x - q1.x, p1.y - q1.y);
        let d2 = (p2.x - q2.x, p2.y - q2.y);
        
        let denominator = d1.0 * d2.1 - d1.1 * d2.0;
        
        if denominator.abs() < 1 {
            return None; // Lines are parallel
        }
        
        let t = ((p2.x - p1.x) * d2.1 - (p2.y - p1.y) * d2.0) as f64 / denominator as f64;
        
        let intersection_x = p1.x as f64 + t * d1.0 as f64;
        let intersection_y = p1.y as f64 + t * d1.1 as f64;
        
        Some(Point64 {
            x: intersection_x.round() as i64,
            y: intersection_y.round() as i64,
        })
    }
    
    fn douglas_peucker_recursive(
        &self,
        path: &Path64,
        start: usize,
        end: usize,
        epsilon_sq: f64,
        result: &mut Path64
    ) {
        if end <= start + 1 {
            result.push(path[start]);
            return;
        }
        
        let mut max_dist_sq = 0.0;
        let mut max_index = start;
        
        // Find the point with maximum distance from line segment
        for i in (start + 1)..end {
            let dist_sq = self.point_to_line_distance_sq(path[i], path[start], path[end]);
            if dist_sq > max_dist_sq {
                max_dist_sq = dist_sq;
                max_index = i;
            }
        }
        
        if max_dist_sq > epsilon_sq {
            // Recursively simplify the two parts
            self.douglas_peucker_recursive(path, start, max_index, epsilon_sq, result);
            self.douglas_peucker_recursive(path, max_index, end, epsilon_sq, result);
        } else {
            // All points are within tolerance, just keep the endpoints
            result.push(path[start]);
        }
    }
    
    fn point_to_line_distance_sq(&self, point: Point64, line_start: Point64, line_end: Point64) -> f64 {
        let a = line_end.x - line_start.x;
        let b = line_end.y - line_start.y;
        let c = point.x - line_start.x;
        let d = point.y - line_start.y;
        
        let dot = a * c + b * d;
        let len_sq = a * a + b * b;
        
        if len_sq == 0 {
            // Line segment is actually a point
            let dx = point.x - line_start.x;
            let dy = point.y - line_start.y;
            return (dx * dx + dy * dy) as f64;
        }
        
        let param = dot as f64 / len_sq as f64;
        let param = param.max(0.0).min(1.0); // Clamp to line segment
        
        let proj_x = line_start.x as f64 + param * a as f64;
        let proj_y = line_start.y as f64 + param * b as f64;
        
        let dx = point.x as f64 - proj_x;
        let dy = point.y as f64 - proj_y;
        
        dx * dx + dy * dy
    }
    
    fn compute_path_hash(&self, path: &Path64, epsilon: f64) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        path.len().hash(&mut hasher);
        ((epsilon * 1000.0) as u64).hash(&mut hasher);
        
        // Hash first few points for quick differentiation
        for (i, point) in path.iter().enumerate() {
            if i >= 8 { break; } // Only hash first 8 points for speed
            point.x.hash(&mut hasher);
            point.y.hash(&mut hasher);
        }
        
        hasher.finish()
    }
}

/// Performance analysis and profiling utilities
pub struct PerformanceProfiler {
    operation_times: HashMap<String, Vec<f64>>,
}

impl PerformanceProfiler {
    pub fn new() -> Self {
        Self {
            operation_times: HashMap::new(),
        }
    }
    
    pub fn time_operation<F, T>(&mut self, operation_name: &str, operation: F) -> T 
    where F: FnOnce() -> T 
    {
        let start = std::time::Instant::now();
        let result = operation();
        let elapsed = start.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
        
        self.operation_times
            .entry(operation_name.to_string())
            .or_insert_with(Vec::new)
            .push(elapsed);
            
        result
    }
    
    pub fn get_statistics(&self, operation_name: &str) -> Option<OperationStats> {
        if let Some(times) = self.operation_times.get(operation_name) {
            if times.is_empty() {
                return None;
            }
            
            let sum: f64 = times.iter().sum();
            let count = times.len();
            let mean = sum / count as f64;
            
            let mut sorted_times = times.clone();
            sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let median = if count % 2 == 0 {
                (sorted_times[count / 2 - 1] + sorted_times[count / 2]) / 2.0
            } else {
                sorted_times[count / 2]
            };
            
            let min = sorted_times[0];
            let max = sorted_times[count - 1];
            
            // Calculate standard deviation
            let variance = times.iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / count as f64;
            let std_dev = variance.sqrt();
            
            Some(OperationStats {
                count,
                mean,
                median,
                min,
                max,
                std_dev,
                total: sum,
            })
        } else {
            None
        }
    }
    
    pub fn print_summary(&self) {
        println!("Performance Profile Summary");
        println!("==========================");
        
        for (operation, _) in &self.operation_times {
            if let Some(stats) = self.get_statistics(operation) {
                println!("{}: {}", operation, stats);
            }
        }
    }
}

#[derive(Debug)]
pub struct OperationStats {
    pub count: usize,
    pub mean: f64,
    pub median: f64,
    pub min: f64,
    pub max: f64,
    pub std_dev: f64,
    pub total: f64,
}

impl std::fmt::Display for OperationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "{} ops, avg: {:.3}ms, median: {:.3}ms, range: {:.3}-{:.3}ms, σ: {:.3}ms",
            self.count, self.mean, self.median, self.min, self.max, self.std_dev
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MakePath;
    
    #[test]
    fn test_fast_area_calculation() {
        let square = MakePath(&[0, 0, 100, 0, 100, 100, 0, 100]);
        let area = PerformanceOptimizer::fast_area_calculation(&square);
        assert_eq!(area.abs(), 10000.0);
    }
    
    #[test] 
    fn test_fast_point_in_polygon() {
        let optimizer = PerformanceOptimizer::new();
        let square = MakePath(&[0, 0, 100, 0, 100, 100, 0, 100]);
        
        let inside_point = Point64 { x: 50, y: 50 };
        let outside_point = Point64 { x: 150, y: 150 };
        
        assert!(optimizer.fast_point_in_polygon(inside_point, &square));
        assert!(!optimizer.fast_point_in_polygon(outside_point, &square));
    }
    
    #[test]
    fn test_performance_profiler() {
        let mut profiler = PerformanceProfiler::new();
        
        // Time a simple operation
        let _result = profiler.time_operation("test_operation", || {
            std::thread::sleep(std::time::Duration::from_millis(1));
            42
        });
        
        let stats = profiler.get_statistics("test_operation");
        assert!(stats.is_some());
        
        let stats = stats.unwrap();
        assert_eq!(stats.count, 1);
        assert!(stats.mean >= 1.0); // At least 1ms
    }
}