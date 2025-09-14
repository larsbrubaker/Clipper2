/// 64-bit integer precision point for exact arithmetic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point64 {
    pub x: i64,
    pub y: i64,
}

impl Point64 {
    /// Creates a new Point64
    /// 
    /// # Arguments
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    
    /// Creates a point at the origin (0, 0)
    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Default for Point64 {
    fn default() -> Self {
        Self::zero()
    }
}

/// Double precision floating point for high-precision calculations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointD {
    pub x: f64,
    pub y: f64,
}

impl PointD {
    /// Creates a new PointD
    /// 
    /// # Arguments
    /// * `x` - X coordinate  
    /// * `y` - Y coordinate
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    /// Creates a point at the origin (0.0, 0.0)
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Default for PointD {
    fn default() -> Self {
        Self::zero()
    }
}

/// Calculate squared distance between two Point64
pub fn distance_sqr_64(pt1: &Point64, pt2: &Point64) -> f64 {
    let dx = pt1.x as f64 - pt2.x as f64;
    let dy = pt1.y as f64 - pt2.y as f64;
    dx * dx + dy * dy
}

/// Calculate squared distance between two PointD
pub fn distance_sqr_d(pt1: &PointD, pt2: &PointD) -> f64 {
    let dx = pt1.x - pt2.x;
    let dy = pt1.y - pt2.y;
    dx * dx + dy * dy
}

/// Calculate distance between two Point64
pub fn distance_64(pt1: &Point64, pt2: &Point64) -> f64 {
    distance_sqr_64(pt1, pt2).sqrt()
}

/// Calculate distance between two PointD
pub fn distance_d(pt1: &PointD, pt2: &PointD) -> f64 {
    distance_sqr_d(pt1, pt2).sqrt()
}