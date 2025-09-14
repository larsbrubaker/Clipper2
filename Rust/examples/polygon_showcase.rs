//! Comprehensive Clipper2 Polygon Operations Showcase
//! 
//! This example demonstrates all the major features of the Clipper2 Rust library:
//! - Boolean operations (Union, Intersection, Difference, XOR)
//! - Various fill rules (EvenOdd, NonZero, Positive, Negative)
//! - Path offsetting with different join types
//! - Complex polygon handling
//! - Self-intersecting polygons
//! - Performance metrics

use clipper2::{
    Clipper64, ClipType, FillRule, JoinType, EndType, 
    MakePath, SimplifyPath, InflatePaths, area, IsPositive,
    Path64, Paths64
};
use std::time::Instant;

fn main() {
    println!("🎨 Clipper2 Rust - Polygon Operations Showcase");
    println!("=============================================\n");

    // Create test polygons
    let rectangle1 = create_rectangle(0, 0, 100, 80);
    let rectangle2 = create_rectangle(50, 40, 150, 120);
    let star = create_star(75, 75, 40, 20, 8);
    let circle = create_circle(100, 60, 35, 24);

    println!("📊 Test Polygons Created:");
    println!("  • Rectangle 1: {}×{} at (0,0)", 100, 80);
    println!("  • Rectangle 2: {}×{} at (50,40)", 100, 80);
    println!("  • Star: 8-point star at (75,75)");
    println!("  • Circle: 24-sided polygon at (100,60)\n");

    // Showcase 1: Basic Boolean Operations
    showcase_boolean_operations(&rectangle1, &rectangle2);

    // Showcase 2: Complex Polygon Clipping
    showcase_complex_clipping(&star, &circle);

    // Showcase 3: Fill Rule Demonstrations
    showcase_fill_rules(&create_self_intersecting_polygon());

    // Showcase 4: Path Offsetting
    showcase_offsetting(&rectangle1, &star);

    // Showcase 5: Performance Metrics
    showcase_performance();

    println!("✅ Showcase Complete! All operations successful.");
}

/// Demonstrate basic boolean operations
fn showcase_boolean_operations(rect1: &Path64, rect2: &Path64) {
    println!("🔧 Boolean Operations Showcase");
    println!("------------------------------");

    let subjects = vec![rect1.clone()];
    let clips = vec![rect2.clone()];

    // Union
    let start = Instant::now();
    let union_result = clipper2::Union(&subjects, &clips, FillRule::NonZero);
    let union_time = start.elapsed();
    
    // Intersection  
    let start = Instant::now();
    let intersection_result = clipper2::Intersect(&subjects, &clips, FillRule::NonZero);
    let intersection_time = start.elapsed();

    // Difference
    let start = Instant::now();
    let difference_result = clipper2::Difference(&subjects, &clips, FillRule::NonZero);
    let difference_time = start.elapsed();

    // XOR
    let start = Instant::now();
    let xor_result = clipper2::Xor(&subjects, &clips, FillRule::NonZero);
    let xor_time = start.elapsed();

    println!("  Union:        {} paths, area: {:.1}, time: {:?}",
        union_result.len(), 
        union_result.iter().map(|p| area(p).abs()).sum::<f64>(),
        union_time
    );
    
    println!("  Intersection: {} paths, area: {:.1}, time: {:?}",
        intersection_result.len(),
        intersection_result.iter().map(|p| area(p).abs()).sum::<f64>(),
        intersection_time
    );
    
    println!("  Difference:   {} paths, area: {:.1}, time: {:?}",
        difference_result.len(),
        difference_result.iter().map(|p| area(p).abs()).sum::<f64>(),
        difference_time
    );
    
    println!("  XOR:          {} paths, area: {:.1}, time: {:?}",
        xor_result.len(),
        xor_result.iter().map(|p| area(p).abs()).sum::<f64>(),
        xor_time
    );

    println!();
}

/// Demonstrate complex polygon clipping
fn showcase_complex_clipping(star: &Path64, circle: &Path64) {
    println!("⭐ Complex Polygon Clipping");
    println!("---------------------------");

    let subjects = vec![star.clone()];
    let clips = vec![circle.clone()];

    let mut clipper = Clipper64::new();
    clipper.add_subject(&subjects);
    clipper.add_clip(&clips);

    let mut solution = Paths64::new();
    let mut open_paths = Paths64::new();

    let start = Instant::now();
    let success = clipper.execute(ClipType::Intersection, FillRule::EvenOdd, &mut solution, &mut open_paths);
    let elapsed = start.elapsed();

    if success {
        let total_area = solution.iter().map(|p| area(p).abs()).sum::<f64>();
        println!("  Star ∩ Circle: {} result paths", solution.len());
        println!("  Total area: {:.1}", total_area);
        println!("  Processing time: {:?}", elapsed);
        
        // Analyze result complexity
        let total_vertices: usize = solution.iter().map(|p| p.len()).sum();
        println!("  Total vertices: {}", total_vertices);
        println!("  Average vertices per path: {:.1}", total_vertices as f64 / solution.len() as f64);
    } else {
        println!("  Complex clipping failed");
    }

    println!();
}

/// Demonstrate different fill rules
fn showcase_fill_rules(self_intersecting: &Path64) {
    println!("📐 Fill Rule Demonstrations");
    println!("---------------------------");

    let subjects = vec![self_intersecting.clone()];
    let fill_rules = [
        (FillRule::EvenOdd, "EvenOdd"),
        (FillRule::NonZero, "NonZero"), 
        (FillRule::Positive, "Positive"),
        (FillRule::Negative, "Negative"),
    ];

    for (fill_rule, name) in &fill_rules {
        let mut clipper = Clipper64::new();
        clipper.add_subject(&subjects);

        let mut solution = Paths64::new();
        let mut open_paths = Paths64::new();

        let start = Instant::now();
        let success = clipper.execute(ClipType::Union, *fill_rule, &mut solution, &mut open_paths);
        let elapsed = start.elapsed();

        if success {
            let total_area = solution.iter().map(|p| area(p).abs()).sum::<f64>();
            println!("  {:<8}: {} paths, area: {:.1}, time: {:?}",
                name, solution.len(), total_area, elapsed);
        } else {
            println!("  {:<8}: Failed", name);
        }
    }

    println!();
}

/// Demonstrate path offsetting
fn showcase_offsetting(rect: &Path64, star: &Path64) {
    println!("📏 Path Offsetting Showcase");
    println!("---------------------------");

    let paths = vec![rect.clone(), star.clone()];
    let deltas = [5.0, 10.0, -3.0];
    let join_types = [
        (JoinType::Round, "Round"),
        (JoinType::Square, "Square"), 
        (JoinType::Miter, "Miter"),
    ];

    for delta in &deltas {
        println!("  Delta: {}", delta);
        for (join_type, name) in &join_types {
            let start = Instant::now();
            let result = InflatePaths(&paths, *delta, *join_type, EndType::Polygon);
            let elapsed = start.elapsed();

            let total_area = result.iter().map(|p| area(p).abs()).sum::<f64>();
            println!("    {:<6}: {} paths, area: {:.1}, time: {:?}",
                name, result.len(), total_area, elapsed);
        }
    }

    println!();
}

/// Performance stress testing
fn showcase_performance() {
    println!("⚡ Performance Stress Testing");
    println!("-----------------------------");

    // Test 1: Many small rectangles
    let mut many_rectangles = Vec::new();
    for i in 0..100 {
        let x = (i % 10) * 15;
        let y = (i / 10) * 15;
        many_rectangles.push(create_rectangle(x as i64, y as i64, 10, 10));
    }

    let start = Instant::now();
    let union_result = clipper2::Union(&many_rectangles, &vec![], FillRule::NonZero);
    let elapsed = start.elapsed();

    println!("  100 rectangles union: {} result paths in {:?}",
        union_result.len(), elapsed);

    // Test 2: Complex star operations
    let complex_star = create_star(0, 0, 100, 50, 16);
    let clip_circle = create_circle(0, 0, 75, 32);

    let start = Instant::now();
    let complex_result = clipper2::Intersect(&vec![complex_star], &vec![clip_circle], FillRule::EvenOdd);
    let elapsed = start.elapsed();

    println!("  16-point star ∩ 32-point circle: {} paths in {:?}",
        complex_result.len(), elapsed);

    // Test 3: Large coordinate values
    let large_rect1 = create_rectangle(1000000, 1000000, 500000, 300000);
    let large_rect2 = create_rectangle(1200000, 1100000, 400000, 400000);

    let start = Instant::now();
    let large_result = clipper2::Intersect(&vec![large_rect1], &vec![large_rect2], FillRule::NonZero);
    let elapsed = start.elapsed();

    println!("  Large coordinate intersection: {} paths in {:?}",
        large_result.len(), elapsed);

    println!();
}

/// Create a rectangle path
fn create_rectangle(x: i64, y: i64, width: i64, height: i64) -> Path64 {
    MakePath(&[x, y, x + width, y, x + width, y + height, x, y + height])
}

/// Create a star polygon
fn create_star(cx: i64, cy: i64, outer_radius: i64, inner_radius: i64, points: usize) -> Path64 {
    let mut coords = Vec::new();
    
    for i in 0..(points * 2) {
        let angle = (i as f64 * std::f64::consts::PI) / points as f64;
        let radius = if i % 2 == 0 { outer_radius } else { inner_radius };
        
        let x = cx + (radius as f64 * angle.cos()) as i64;
        let y = cy + (radius as f64 * angle.sin()) as i64;
        
        coords.push(x);
        coords.push(y);
    }
    
    MakePath(&coords)
}

/// Create a circular polygon (regular n-sided polygon)
fn create_circle(cx: i64, cy: i64, radius: i64, sides: usize) -> Path64 {
    let mut coords = Vec::new();
    
    for i in 0..sides {
        let angle = (i as f64 * 2.0 * std::f64::consts::PI) / sides as f64;
        let x = cx + (radius as f64 * angle.cos()) as i64;
        let y = cy + (radius as f64 * angle.sin()) as i64;
        
        coords.push(x);
        coords.push(y);
    }
    
    MakePath(&coords)
}

/// Create a self-intersecting polygon (figure-8 or bowtie shape)
fn create_self_intersecting_polygon() -> Path64 {
    MakePath(&[
        0, 0,       // bottom left
        100, 100,   // top right (creates intersection)
        200, 0,     // bottom right  
        200, 200,   // top right
        100, 100,   // intersection point again
        0, 200,     // top left
    ])
}