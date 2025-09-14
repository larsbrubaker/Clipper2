//! Polygon orientation and area calculations

use crate::path::Path64;

/// Fill rule for polygon operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FillRule {
    EvenOdd,
    NonZero,
    Positive,
    Negative,
}

/// Clip operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClipType {
    Intersection,
    Union,
    Difference,
    Xor,
}

/// Calculate the signed area of a polygon using the shoelace formula
/// 
/// # Arguments
/// * `poly` - The polygon path
/// 
/// # Returns
/// Positive area if clockwise, negative if counter-clockwise
/// This follows the C++ implementation from Clipper2
pub fn area(poly: &Path64) -> f64 {
    let cnt = poly.len();
    if cnt < 3 {
        return 0.0;
    }

    let mut a = 0.0;
    let mut i = 0;
    let mut j = cnt - 1;
    
    // Exact port of C++ Area calculation using shoelace formula
    // C++: result += (prev->pt.y + pt.y) * (prev->pt.x - pt.x)
    // This means: (y_prev + y_curr) * (x_prev - x_curr)
    while i < cnt {
        a += (poly[j].y as f64 + poly[i].y as f64) * (poly[j].x as f64 - poly[i].x as f64);
        j = i;
        i += 1;
    }
    
    a * 0.5
}

/// Determine if a polygon has positive (clockwise) orientation
/// 
/// A curve has positive orientation [and area] if a region 'R'
/// is on the left when traveling around the outside of 'R'.
/// This is the opposite of the usual mathematical convention but
/// consistent with many graphics libraries.
/// 
/// # Arguments
/// * `poly` - The polygon path to check
/// 
/// # Returns
/// `true` if the polygon is oriented clockwise (positive area)
/// 
/// # Example
/// ```
/// use clipper2::{MakePath, is_positive};
/// 
/// // Clockwise square  
/// let clockwise = MakePath(&[0, 0, 10, 0, 10, 10, 0, 10]);
/// assert_eq!(is_positive(&clockwise), true);
/// 
/// // Counter-clockwise square
/// let counter_clockwise = MakePath(&[0, 0, 0, 10, 10, 10, 10, 0]);  
/// assert_eq!(is_positive(&counter_clockwise), false);
/// ```
pub fn is_positive(poly: &Path64) -> bool {
    area(poly) >= 0.0
}

/// C++ API compatibility function
#[allow(non_snake_case)]
pub fn IsPositive(poly: &Path64) -> bool {
    is_positive(poly)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MakePath;

    #[test]
    fn test_area_positive() {
        // Clockwise square: (0,0) -> (10,0) -> (10,10) -> (0,10) -> (0,0)
        let square = MakePath(&[0, 0, 10, 0, 10, 10, 0, 10]);
        let area_val = area(&square);
        assert!(area_val > 0.0);
        assert_eq!(area_val, 100.0);
    }

    #[test] 
    fn test_area_negative() {
        // Counter-clockwise square: (0,0) -> (0,10) -> (10,10) -> (10,0) -> (0,0)
        let square = MakePath(&[0, 0, 0, 10, 10, 10, 10, 0]);
        let area_val = area(&square);
        assert!(area_val < 0.0);
        assert_eq!(area_val, -100.0);
    }

    #[test]
    fn test_is_positive_clockwise() {
        let square = MakePath(&[0, 0, 10, 0, 10, 10, 0, 10]);
        assert_eq!(is_positive(&square), true);
        assert_eq!(IsPositive(&square), true); // C++ API
    }

    #[test]
    fn test_is_positive_counter_clockwise() {
        let square = MakePath(&[0, 0, 0, 10, 10, 10, 10, 0]);
        assert_eq!(is_positive(&square), false);
        assert_eq!(IsPositive(&square), false); // C++ API
    }

    #[test] 
    fn test_orientation_cpp_test_case() {
        // Test the exact case from C++ TestOrientation
        // subjects.push_back(MakePath({ 0,0, 0,100, 100,100, 100,0 }));
        // EXPECT_FALSE(IsPositive(subjects[0]));
        
        let path1 = MakePath(&[0, 0, 0, 100, 100, 100, 100, 0]);
        assert_eq!(IsPositive(&path1), false);
        
        let path2 = MakePath(&[10, 10, 10, 110, 110, 110, 110, 10]);
        assert_eq!(IsPositive(&path2), false);
        
        let path3 = MakePath(&[50, 50, 50, 150, 150, 150, 150, 50]);
        assert_eq!(IsPositive(&path3), false);
    }
}