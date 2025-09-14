//! Clipper2 Rust Performance Benchmarks
//! 
//! Comprehensive benchmarking suite to measure the performance of polygon operations
//! and compare against various scenarios and complexity levels.

use clipper2::{
    Clipper64, ClipType, FillRule, JoinType, EndType,
    MakePath, InflatePaths, Union, Intersect, Difference, Xor,
    Path64, Paths64
};
use std::hint::black_box;

// Simple benchmarks using std::time (since criterion isn't always available)

fn main() {
    println!("🚀 Clipper2 Rust Performance Benchmarks");
    println!("========================================\n");

    // Run all benchmark categories
    benchmark_basic_operations();
    benchmark_complex_polygons();
    benchmark_many_polygons();
    benchmark_offsetting();
    benchmark_precision_handling();
    benchmark_edge_cases();

    println!("✅ All benchmarks completed successfully!");
}

fn benchmark_basic_operations() {
    println!("📊 Basic Boolean Operations");
    println!("---------------------------");

    let rect1 = create_rectangle(0, 0, 1000, 800);
    let rect2 = create_rectangle(500, 400, 1000, 800);
    let subjects = vec![rect1];
    let clips = vec![rect2];

    // Benchmark each operation multiple times
    const ITERATIONS: usize = 1000;

    let start = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _result = black_box(Union(&subjects, &clips, FillRule::NonZero));
    }
    let union_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _result = black_box(Intersect(&subjects, &clips, FillRule::NonZero));
    }
    let intersection_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _result = black_box(Difference(&subjects, &clips, FillRule::NonZero));
    }
    let difference_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _result = black_box(Xor(&subjects, &clips, FillRule::NonZero));
    }
    let xor_time = start.elapsed();

    println!("  Union:        {:>8.2}μs/op ({} iterations)", 
        union_time.as_nanos() as f64 / ITERATIONS as f64 / 1000.0, ITERATIONS);
    println!("  Intersection: {:>8.2}μs/op ({} iterations)",
        intersection_time.as_nanos() as f64 / ITERATIONS as f64 / 1000.0, ITERATIONS);
    println!("  Difference:   {:>8.2}μs/op ({} iterations)",
        difference_time.as_nanos() as f64 / ITERATIONS as f64 / 1000.0, ITERATIONS);
    println!("  XOR:          {:>8.2}μs/op ({} iterations)",
        xor_time.as_nanos() as f64 / ITERATIONS as f64 / 1000.0, ITERATIONS);

    println!();
}

fn benchmark_complex_polygons() {
    println!("🌟 Complex Polygon Operations");
    println!("-----------------------------");

    const ITERATIONS: usize = 100;

    // Complex star vs circle
    let star = create_star(0, 0, 1000, 500, 16);
    let circle = create_circle(0, 0, 750, 64);
    let subjects = vec![star];
    let clips = vec![circle];

    let start = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _result = black_box(Intersect(&subjects, &clips, FillRule::EvenOdd));
    }
    let elapsed = start.elapsed();

    println!("  16-point star ∩ 64-point circle: {:>8.2}μs/op ({} iterations)",
        elapsed.as_nanos() as f64 / ITERATIONS as f64 / 1000.0, ITERATIONS);

    // Self-intersecting polygon
    let bowtie = create_bowtie();
    let subjects = vec![bowtie];

    let start = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _result = black_box(Union(&subjects, &vec![], FillRule::EvenOdd));
    }
    let elapsed = start.elapsed();

    println!("  Self-intersecting union:      {:>8.2}μs/op ({} iterations)",
        elapsed.as_nanos() as f64 / ITERATIONS as f64 / 1000.0, ITERATIONS);

    println!();
}

fn benchmark_many_polygons() {
    println!("📦 Many Polygon Operations");
    println!("--------------------------");

    const ITERATIONS: usize = 10;

    // Many small rectangles
    let many_rects = create_many_rectangles(100);

    let start = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _result = black_box(Union(&many_rects, &vec![], FillRule::NonZero));
    }
    let elapsed = start.elapsed();

    println!("  Union of 100 rectangles:     {:>8.2}ms/op ({} iterations)",
        elapsed.as_millis() as f64 / ITERATIONS as f64, ITERATIONS);

    // Many overlapping circles
    let many_circles = create_many_circles(50);

    let start = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _result = black_box(Union(&many_circles, &vec![], FillRule::NonZero));
    }
    let elapsed = start.elapsed();

    println!("  Union of 50 circles:         {:>8.2}ms/op ({} iterations)",
        elapsed.as_millis() as f64 / ITERATIONS as f64, ITERATIONS);

    println!();
}

fn benchmark_offsetting() {
    println!("📏 Path Offsetting Operations");
    println!("-----------------------------");

    const ITERATIONS: usize = 100;

    let complex_path = create_star(0, 0, 1000, 500, 12);
    let paths = vec![complex_path];

    // Different join types
    let join_types = [
        (JoinType::Round, "Round"),
        (JoinType::Square, "Square"),
        (JoinType::Miter, "Miter"),
    ];

    for (join_type, name) in &join_types {
        let start = std::time::Instant::now();
        for _ in 0..ITERATIONS {
            let _result = black_box(InflatePaths(&paths, 50.0, *join_type, EndType::Polygon));
        }
        let elapsed = start.elapsed();

        println!("  {} join offsetting:     {:>8.2}μs/op ({} iterations)",
            name, elapsed.as_nanos() as f64 / ITERATIONS as f64 / 1000.0, ITERATIONS);
    }

    println!();
}

fn benchmark_precision_handling() {
    println!("🎯 Precision Handling");
    println!("--------------------");

    const ITERATIONS: usize = 100;

    // Large coordinate values
    let large_rect1 = create_rectangle(10_000_000, 10_000_000, 5_000_000, 3_000_000);
    let large_rect2 = create_rectangle(12_000_000, 11_000_000, 4_000_000, 4_000_000);

    let start = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _result = black_box(Intersect(&vec![large_rect1.clone()], &vec![large_rect2.clone()], FillRule::NonZero));
    }
    let elapsed = start.elapsed();

    println!("  Large coordinates:           {:>8.2}μs/op ({} iterations)",
        elapsed.as_nanos() as f64 / ITERATIONS as f64 / 1000.0, ITERATIONS);

    // Very small coordinates (near-zero precision)
    let small_rect1 = MakePath(&[0, 0, 10, 0, 10, 10, 0, 10]);
    let small_rect2 = MakePath(&[5, 5, 15, 5, 15, 15, 5, 15]);

    let start = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _result = black_box(Intersect(&vec![small_rect1.clone()], &vec![small_rect2.clone()], FillRule::NonZero));
    }
    let elapsed = start.elapsed();

    println!("  Small coordinates:           {:>8.2}μs/op ({} iterations)",
        elapsed.as_nanos() as f64 / ITERATIONS as f64 / 1000.0, ITERATIONS);

    println!();
}

fn benchmark_edge_cases() {
    println!("🔧 Edge Case Handling");
    println!("---------------------");

    const ITERATIONS: usize = 100;

    // Degenerate polygons
    let line = MakePath(&[0, 0, 100, 0, 0, 0]); // Degenerate triangle
    let point = MakePath(&[50, 50, 50, 50, 50, 50]); // Point

    let start = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _result = black_box(Union(&vec![line.clone()], &vec![point.clone()], FillRule::NonZero));
    }
    let elapsed = start.elapsed();

    println!("  Degenerate polygons:         {:>8.2}μs/op ({} iterations)",
        elapsed.as_nanos() as f64 / ITERATIONS as f64 / 1000.0, ITERATIONS);

    // Polygons with many collinear points
    let collinear = create_collinear_polygon();

    let start = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _result = black_box(Union(&vec![collinear.clone()], &vec![], FillRule::NonZero));
    }
    let elapsed = start.elapsed();

    println!("  Collinear points:            {:>8.2}μs/op ({} iterations)",
        elapsed.as_nanos() as f64 / ITERATIONS as f64 / 1000.0, ITERATIONS);

    println!();
}

// Helper functions for creating test geometry

fn create_rectangle(x: i64, y: i64, width: i64, height: i64) -> Path64 {
    MakePath(&[x, y, x + width, y, x + width, y + height, x, y + height])
}

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

fn create_bowtie() -> Path64 {
    MakePath(&[
        0, 0,       // bottom left
        100, 100,   // intersection center
        200, 0,     // bottom right
        200, 200,   // top right
        100, 100,   // intersection center again
        0, 200,     // top left
    ])
}

fn create_many_rectangles(count: usize) -> Paths64 {
    let mut paths = Vec::new();
    let grid_size = (count as f64).sqrt() as usize + 1;
    
    for i in 0..count {
        let x = (i % grid_size) as i64 * 80;
        let y = (i / grid_size) as i64 * 60;
        paths.push(create_rectangle(x, y, 70, 50));
    }
    
    paths
}

fn create_many_circles(count: usize) -> Paths64 {
    let mut paths = Vec::new();
    let grid_size = (count as f64).sqrt() as usize + 1;
    
    for i in 0..count {
        let x = (i % grid_size) as i64 * 100 + 50;
        let y = (i / grid_size) as i64 * 100 + 50;
        paths.push(create_circle(x, y, 45, 16));
    }
    
    paths
}

fn create_collinear_polygon() -> Path64 {
    MakePath(&[
        0, 0,      // corner
        25, 0,     // collinear point
        50, 0,     // collinear point  
        100, 0,    // corner
        100, 25,   // collinear point
        100, 50,   // collinear point
        100, 100,  // corner
        50, 100,   // collinear point
        0, 100,    // corner
        0, 50,     // collinear point
    ])
}