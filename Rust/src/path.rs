use crate::point::{Point64, PointD};

/// A path is a sequence of points forming a polygon or polyline
pub type Path64 = Vec<Point64>;
pub type PathD = Vec<PointD>;

/// Multiple paths
pub type Paths64 = Vec<Path64>;
pub type PathsD = Vec<PathD>;

/// Creates a Path64 from a slice of coordinate pairs
/// Each pair of values represents (x, y) coordinates
pub fn make_path(coords: &[i64]) -> Path64 {
    let mut path = Vec::new();
    
    // Ensure we have an even number of coordinates
    let len = coords.len() - (coords.len() % 2);
    
    for i in (0..len).step_by(2) {
        path.push(Point64::new(coords[i], coords[i + 1]));
    }
    
    path
}

/// Creates a PathD from a slice of coordinate pairs
pub fn make_path_d(coords: &[f64]) -> PathD {
    let mut path = Vec::new();
    
    // Ensure we have an even number of coordinates
    let len = coords.len() - (coords.len() % 2);
    
    for i in (0..len).step_by(2) {
        path.push(PointD::new(coords[i], coords[i + 1]));
    }
    
    path
}

/// Calculate the perimeter length of a path
pub fn length(path: &Path64, is_closed: bool) -> f64 {
    if path.len() < 2 {
        return 0.0;
    }
    
    let mut result = 0.0;
    
    // Calculate distance between consecutive points
    for i in 0..(path.len() - 1) {
        result += crate::point::distance_64(&path[i], &path[i + 1]);
    }
    
    // If closed, add distance from last point back to first
    if is_closed && path.len() > 2 {
        result += crate::point::distance_64(&path[path.len() - 1], &path[0]);
    }
    
    result
}

/// Calculate the perimeter length of a PathD
pub fn length_d(path: &PathD, is_closed: bool) -> f64 {
    if path.len() < 2 {
        return 0.0;
    }
    
    let mut result = 0.0;
    
    // Calculate distance between consecutive points
    for i in 0..(path.len() - 1) {
        result += crate::point::distance_d(&path[i], &path[i + 1]);
    }
    
    // If closed, add distance from last point back to first
    if is_closed && path.len() > 2 {
        result += crate::point::distance_d(&path[path.len() - 1], &path[0]);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point64;

    #[test]
    fn test_make_path() {
        let path = make_path(&[0, 0, 10, 20, 30, 40]);
        
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], Point64::new(0, 0));
        assert_eq!(path[1], Point64::new(10, 20));
        assert_eq!(path[2], Point64::new(30, 40));
    }

    #[test]
    fn test_make_path_odd_coords() {
        // Should ignore the last odd coordinate
        let path = make_path(&[0, 0, 10, 20, 30]);
        
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], Point64::new(0, 0));
        assert_eq!(path[1], Point64::new(10, 20));
    }

    #[test]
    fn test_length_open() {
        let path = make_path(&[0, 0, 3, 4]); // 3-4-5 triangle leg
        let len = length(&path, false);
        
        assert_eq!(len, 5.0);
    }

    #[test]
    fn test_length_closed() {
        let path = make_path(&[0, 0, 3, 0, 3, 4, 0, 4]); // Rectangle
        let len = length(&path, true);
        
        // Perimeter: 3 + 4 + 3 + 4 = 14
        assert_eq!(len, 14.0);
    }

    #[test]
    fn test_length_single_point() {
        let path = make_path(&[5, 10]);
        let len = length(&path, false);
        
        assert_eq!(len, 0.0);
    }
}