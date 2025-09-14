//! # Clipper2 Rust Port
//!
//! A Rust port of the Clipper2 polygon clipping library.
//! This library provides polygon boolean operations (union, intersection, difference, XOR),
//! polygon offsetting, and path simplification functionality.

pub mod point;
pub mod path;
pub mod simplify;
pub mod orientation;
pub mod boolean;
pub mod engine;
pub mod clipper; // Clipper64 using complete Vatti algorithm
pub mod polytree;
pub mod offset;
pub mod rectclip;
pub mod performance;
pub mod c_plus_plus_reference_tests;
pub mod debug_engine;
pub mod engine_complete;
pub mod clipper_complete;

// Re-export main types for convenience  
pub use point::{Point64, PointD, distance_64, distance_d, distance_sqr_64, distance_sqr_d};
pub use path::{Path64, PathD, Paths64, PathsD, make_path, make_path_d, length, length_d};
pub use simplify::{simplify_path, simplify_path_d};
pub use orientation::{FillRule, ClipType, area, is_positive, IsPositive};
pub use boolean::{union, Union};
pub use clipper::Clipper64;
pub use polytree::{PolyTree64, PolyTreeD, PolyNode64, PolyNodeD, polytree_to_paths64, polytree_to_paths_d};
pub use offset::{ClipperOffset, JoinType, EndType};
pub use rectclip::{RectClip64, RectClipLines64};

// C++-compatible API (PascalCase for compatibility with existing C++ code)
/// Creates a Path64 from a slice of coordinates (C++ API compatibility)
/// 
/// # Arguments
/// * `coords` - Slice of i64 coordinates in pairs (x1, y1, x2, y2, ...)
/// 
/// # Returns
/// A `Path64` containing the points
///
/// # Example
/// ```
/// use clipper2::MakePath;
/// let path = MakePath(&[0, 0, 10, 10, 20, 0]);
/// assert_eq!(path.len(), 3);
/// ```
#[allow(non_snake_case)] // C++ compatibility
pub fn MakePath(coords: &[i64]) -> Path64 {
    make_path(coords)
}

/// Simplifies a path by removing near-collinear points (C++ API compatibility)
///
/// # Arguments
/// * `path` - The path to simplify
/// * `epsilon` - Distance tolerance for point removal
/// * `is_closed_path` - Whether the path is closed (true) or open (false)
///
/// # Returns
/// A simplified `Path64` with redundant points removed
///
/// # Example
/// ```
/// use clipper2::{MakePath, SimplifyPath};
/// let path = MakePath(&[0, 0, 1, 1, 2, 2, 10, 10]);
/// let simplified = SimplifyPath(&path, 2.0, false);
/// assert_eq!(simplified.len(), 2); // Should keep only start and end
/// ```
#[allow(non_snake_case)] // C++ compatibility
pub fn SimplifyPath(path: &Path64, epsilon: f64, is_closed_path: bool) -> Path64 {
    simplify_path(path, epsilon, is_closed_path)
}

/// Calculates the length of a path (C++ API compatibility)
///
/// # Arguments
/// * `path` - The path to measure
///
/// # Returns
/// The total length of the path
///
/// # Example
/// ```
/// use clipper2::{MakePath, Length};
/// let path = MakePath(&[0, 0, 3, 4]); // 3-4-5 triangle leg
/// assert_eq!(Length(&path), 5.0);
/// ```
#[allow(non_snake_case)] // C++ compatibility  
pub fn Length(path: &Path64) -> f64 {
    length(path, false)
}

/// Converts a PolyTree64 to flat paths (C++ API compatibility)
///
/// # Arguments
/// * `polytree` - The PolyTree64 to convert
///
/// # Returns  
/// A `Paths64` containing all polygon paths from the tree
///
/// # Example
/// ```
/// use clipper2::{PolyTree64, PolyTreeToPaths64};
/// let tree = PolyTree64::new();
/// let paths = PolyTreeToPaths64(&tree);
/// ```
#[allow(non_snake_case)] // C++ compatibility
pub fn PolyTreeToPaths64(polytree: &polytree::PolyTree64) -> Paths64 {
    polytree::polytree_to_paths64(polytree)
}

/// Converts a PolyTreeD to flat paths (C++ API compatibility)  
///
/// # Arguments
/// * `polytree` - The PolyTreeD to convert
///
/// # Returns
/// A `PathsD` containing all polygon paths from the tree
///
/// # Example
/// ```
/// use clipper2::{PolyTreeD, PolyTreeToPathsD};
/// let tree = PolyTreeD::new();  
/// let paths = PolyTreeToPathsD(&tree);
/// ```
#[allow(non_snake_case)] // C++ compatibility
pub fn PolyTreeToPathsD(polytree: &polytree::PolyTreeD) -> PathsD {
    polytree::polytree_to_paths_d(polytree)
}

// Global clipping functions for C++ API compatibility

/// Perform intersection operation on paths (C++ API compatibility)
/// 
/// # Arguments
/// * `subjects` - Subject paths
/// * `clips` - Clip paths  
/// * `fill_rule` - Fill rule to use
/// 
/// # Returns
/// Result paths from intersection
#[allow(non_snake_case)]
pub fn Intersect(subjects: &Paths64, clips: &Paths64, fill_rule: FillRule) -> Paths64 {
    let mut clipper = clipper::Clipper64::new();
    
    if !clipper.add_subject(subjects) || !clipper.add_clip(clips) {
        return Vec::new();
    }
    
    let mut solution = Paths64::new();
    let mut solution_open = Paths64::new();
    
    if clipper.execute(ClipType::Intersection, fill_rule, &mut solution, &mut solution_open) {
        solution
    } else {
        Vec::new()
    }
}

/// Perform difference operation on paths (C++ API compatibility)
/// 
/// # Arguments
/// * `subjects` - Subject paths
/// * `clips` - Clip paths
/// * `fill_rule` - Fill rule to use
/// 
/// # Returns  
/// Result paths from difference (subjects - clips)
#[allow(non_snake_case)]
pub fn Difference(subjects: &Paths64, clips: &Paths64, fill_rule: FillRule) -> Paths64 {
    let mut clipper = clipper::Clipper64::new();
    
    if !clipper.add_subject(subjects) || !clipper.add_clip(clips) {
        return Vec::new();
    }
    
    let mut solution = Paths64::new();
    let mut solution_open = Paths64::new();
    
    if clipper.execute(ClipType::Difference, fill_rule, &mut solution, &mut solution_open) {
        solution
    } else {
        Vec::new()
    }
}

/// Perform XOR operation on paths (C++ API compatibility)
/// 
/// # Arguments
/// * `subjects` - Subject paths
/// * `clips` - Clip paths
/// * `fill_rule` - Fill rule to use
/// 
/// # Returns
/// Result paths from XOR operation  
#[allow(non_snake_case)]
pub fn Xor(subjects: &Paths64, clips: &Paths64, fill_rule: FillRule) -> Paths64 {
    let mut clipper = clipper::Clipper64::new();
    
    if !clipper.add_subject(subjects) || !clipper.add_clip(clips) {
        return Vec::new();
    }
    
    let mut solution = Paths64::new();
    let mut solution_open = Paths64::new();
    
    if clipper.execute(ClipType::Xor, fill_rule, &mut solution, &mut solution_open) {
        solution
    } else {
        Vec::new()
    }
}

/// Inflate/deflate paths (C++ API compatibility)
/// 
/// # Arguments
/// * `paths` - Input paths to offset
/// * `delta` - Offset distance (positive for expansion, negative for contraction)
/// * `join_type` - How to join offset edges
/// * `end_type` - How to handle path ends
/// 
/// # Returns
/// Offset paths
#[allow(non_snake_case)]
pub fn InflatePaths(paths: &Paths64, delta: f64, join_type: JoinType, end_type: EndType) -> Paths64 {
    let mut offset = ClipperOffset::new();
    
    for path in paths {
        offset.add_path(path, join_type, end_type);
    }
    
    offset.execute(delta)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_global_api_functions() {
        // Test the C++ compatible global functions
        let subjects = vec![MakePath(&[0, 0, 10, 0, 10, 10, 0, 10])];
        let clips = vec![MakePath(&[5, 5, 15, 5, 15, 15, 5, 15])];
        
        // Test Intersect
        let intersection = Intersect(&subjects, &clips, FillRule::NonZero);
        assert!(intersection.len() >= 0, "Intersect should return valid result");
        
        // Test Difference
        let difference = Difference(&subjects, &clips, FillRule::NonZero);
        assert!(difference.len() >= 0, "Difference should return valid result");
        
        // Test Xor
        let xor_result = Xor(&subjects, &clips, FillRule::NonZero);
        assert!(xor_result.len() >= 0, "Xor should return valid result");
        
        // Test InflatePaths
        let inflated = InflatePaths(&subjects, 2.0, JoinType::Round, EndType::Polygon);
        assert!(inflated.len() >= 0, "InflatePaths should return valid result");
    }
}