//! Boolean operations on polygons (Union, Intersection, Difference, XOR)

use crate::path::{Paths64};
use crate::orientation::{ClipType, FillRule};
use crate::clipper::Clipper64;

/// Perform a Union operation on two sets of paths
/// 
/// # Arguments  
/// * `subjects` - Subject paths (typically the main polygons)
/// * `clips` - Clip paths (paths to union with subjects)  
/// * `fill_rule` - Fill rule to determine inside/outside
/// 
/// # Returns
/// Result paths from the union operation
pub fn union(subjects: &Paths64, clips: &Paths64, fill_rule: FillRule) -> Paths64 {
    let mut clipper = Clipper64::new();
    
    // eprintln!("DEBUG: union() called with {} subjects, {} clips", subjects.len(), clips.len());
    // for (i, subject) in subjects.iter().enumerate() {
    //     eprintln!("  Subject {}: {} vertices", i, subject.len());
    //     for (j, pt) in subject.iter().enumerate() {
    //         eprintln!("    Vertex {}: ({}, {})", j, pt.x, pt.y);
    //     }
    // }
    
    // Add paths to the clipper engine
    // eprintln!("DEBUG: Adding subjects to clipper");
    let subjects_ok = clipper.add_subject(subjects);
    // eprintln!("DEBUG: add_subject returned: {}", subjects_ok);
    
    // eprintln!("DEBUG: Adding clips to clipper");
    let clips_ok = clipper.add_clip(clips);
    // eprintln!("DEBUG: add_clip returned: {}", clips_ok);
    
    if !subjects_ok || !clips_ok {
        // If adding paths failed, return empty result
        // eprintln!("DEBUG: Adding paths failed!");
        return Vec::new();
    }
    
    // Execute the union operation
    let mut solution = Paths64::new();
    let mut solution_open = Paths64::new();
    
    if clipper.execute(ClipType::Union, fill_rule, &mut solution, &mut solution_open) {
        // eprintln!("DEBUG: Engine execute succeeded, {} solutions, {} open", solution.len(), solution_open.len());
        solution
    } else {
        // Engine failed - return empty result
        // eprintln!("DEBUG: Engine execute FAILED");
        Vec::new()
    }
}

/// C++ API compatibility function
#[allow(non_snake_case)]
pub fn Union(subjects: &Paths64, clips: &Paths64, fill_rule: FillRule) -> Paths64 {
    union(subjects, clips, fill_rule)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MakePath, IsPositive};

    #[test]
    fn test_union_negative_fill_rule() {
        // This mirrors the C++ test: TestNegativeOrientation
        let mut subjects = Paths64::new();
        subjects.push(MakePath(&[0, 0, 0, 100, 100, 100, 100, 0]));
        subjects.push(MakePath(&[10, 10, 10, 110, 110, 110, 110, 10]));
        
        // Verify orientations match C++ test expectations  
        assert_eq!(IsPositive(&subjects[0]), false);
        assert_eq!(IsPositive(&subjects[1]), false);
        
        let mut clips = Paths64::new();
        clips.push(MakePath(&[50, 50, 50, 150, 150, 150, 150, 50]));
        assert_eq!(IsPositive(&clips[0]), false);
        
        let solution = Union(&subjects, &clips, FillRule::Negative);
        
        // The C++ test expects 1 result path with 12 points
        assert_eq!(solution.len(), 1);
        assert_eq!(solution[0].len(), 12);
    }

    #[test]
    fn test_union_basic() {
        // Test basic union functionality
        let subjects = vec![MakePath(&[0, 0, 10, 0, 10, 10, 0, 10])];
        let clips = vec![MakePath(&[5, 5, 15, 5, 15, 15, 5, 15])];
        
        let result = union(&subjects, &clips, FillRule::NonZero);
        
        // Should return union result from the engine
        assert!(result.len() >= 1, "Union should produce at least one result path");
    }
}