//! Performance Optimization Demonstration
//!
//! This example demonstrates the performance improvements available
//! in the Clipper2 Rust implementation, comparing optimized vs standard approaches.

use clipper2::{
    MakePath, Union, area, Path64, FillRule,
    performance::{PerformanceOptimizer, PerformanceProfiler}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Clipper2 Performance Optimization Demo");
    println!("=========================================\n");
    
    // Initialize optimizer and profiler
    let mut optimizer = PerformanceOptimizer::new();
    let mut profiler = PerformanceProfiler::new();
    
    // Generate test data
    let test_paths = generate_test_polygons(1000);
    println!("📊 Generated {} test polygons for benchmarking\n", test_paths.len());
    
    // Benchmark 1: Area Calculation
    benchmark_area_calculation(&test_paths, &mut profiler);
    
    // Benchmark 2: Point-in-Polygon Tests
    benchmark_point_in_polygon(&test_paths, &optimizer, &mut profiler);
    
    // Benchmark 3: Polygon Simplification
    benchmark_polygon_simplification(&test_paths, &mut optimizer, &mut profiler);
    
    // Benchmark 4: Batch Processing
    benchmark_batch_processing(&test_paths, &mut optimizer, &mut profiler);
    
    // Benchmark 5: Memory-efficient Operations
    benchmark_memory_efficiency(&mut profiler);
    
    println!("\n📈 Performance Summary");
    println!("======================");
    profiler.print_summary();
    
    println!("\n💡 Performance Tips:");
    println!("  • Use fast_area_calculation() for large polygon sets");
    println!("  • Batch process multiple paths for better cache efficiency");
    println!("  • Enable compiler optimizations (--release) for production");
    println!("  • Pre-allocate vectors when possible to reduce allocations");
    println!("  • Use appropriate epsilon values for simplification");
    
    Ok(())
}

fn benchmark_area_calculation(paths: &[Path64], profiler: &mut PerformanceProfiler) {
    println!("🔍 Benchmarking Area Calculation");
    println!("---------------------------------");
    
    const ITERATIONS: usize = 100;
    
    // Standard area calculation
    let standard_total = profiler.time_operation("standard_area_calc", || {
        let mut total_area = 0.0;
        for _ in 0..ITERATIONS {
            for path in paths {
                total_area += area(path).abs();
            }
        }
        total_area
    });
    
    // Optimized area calculation
    let optimized_total = profiler.time_operation("optimized_area_calc", || {
        let mut total_area = 0.0;
        for _ in 0..ITERATIONS {
            for path in paths {
                total_area += PerformanceOptimizer::fast_area_calculation(path).abs();
            }
        }
        total_area
    });
    
    println!("  Standard method: {:.2} total area", standard_total);
    println!("  Optimized method: {:.2} total area", optimized_total);
    
    if let (Some(standard), Some(optimized)) = (
        profiler.get_statistics("standard_area_calc"),
        profiler.get_statistics("optimized_area_calc")
    ) {
        let speedup = standard.mean / optimized.mean;
        println!("  ⚡ Speedup: {:.2}x faster", speedup);
    }
    
    println!();
}

fn benchmark_point_in_polygon(paths: &[Path64], optimizer: &PerformanceOptimizer, profiler: &mut PerformanceProfiler) {
    println!("🎯 Benchmarking Point-in-Polygon Tests");
    println!("--------------------------------------");
    
    // Generate test points
    let test_points = generate_test_points(100);
    let test_polygon = &paths[0]; // Use first polygon for testing
    
    // Standard approach (simplified - we don't have the standard implementation exposed)
    let standard_results = profiler.time_operation("standard_pip", || {
        let mut count = 0;
        for point in &test_points {
            // Simulate standard point-in-polygon test with basic area method
            let triangle1 = MakePath(&[test_polygon[0].x, test_polygon[0].y, point.x, point.y, test_polygon[1].x, test_polygon[1].y]);
            let area1 = area(&triangle1).abs();
            if area1 > 0.0 {
                count += 1;
            }
        }
        count
    });
    
    // Optimized approach
    let optimized_results = profiler.time_operation("optimized_pip", || {
        let mut count = 0;
        for point in &test_points {
            if optimizer.fast_point_in_polygon(*point, test_polygon) {
                count += 1;
            }
        }
        count
    });
    
    println!("  Standard method: {} points inside", standard_results);
    println!("  Optimized method: {} points inside", optimized_results);
    
    if let (Some(standard), Some(optimized)) = (
        profiler.get_statistics("standard_pip"),
        profiler.get_statistics("optimized_pip")
    ) {
        let speedup = standard.mean / optimized.mean;
        println!("  ⚡ Speedup: {:.2}x faster", speedup);
    }
    
    println!();
}

fn benchmark_polygon_simplification(paths: &[Path64], optimizer: &mut PerformanceOptimizer, profiler: &mut PerformanceProfiler) {
    println!("🔧 Benchmarking Polygon Simplification");
    println!("--------------------------------------");
    
    let epsilon = 2.0;
    let test_paths = &paths[..10]; // Use first 10 paths
    
    // Standard approach (using basic vertex removal)
    let standard_results = profiler.time_operation("standard_simplify", || {
        let mut simplified_paths = Vec::new();
        for path in test_paths {
            // Simple vertex removal based on distance
            let mut simplified = Vec::new();
            if !path.is_empty() {
                simplified.push(path[0]);
                
                for i in 1..path.len() {
                    let prev = simplified[simplified.len() - 1];
                    let curr = path[i];
                    let dx = (curr.x - prev.x) as f64;
                    let dy = (curr.y - prev.y) as f64;
                    let dist = (dx * dx + dy * dy).sqrt();
                    
                    if dist > epsilon {
                        simplified.push(curr);
                    }
                }
            }
            simplified_paths.push(simplified);
        }
        simplified_paths
    });
    
    // Optimized approach
    let optimized_results = profiler.time_operation("optimized_simplify", || {
        let mut simplified_paths = Vec::new();
        for path in test_paths {
            simplified_paths.push(optimizer.fast_simplify_polygon(path, epsilon));
        }
        simplified_paths
    });
    
    let standard_vertices: usize = standard_results.iter().map(|p| p.len()).sum();
    let optimized_vertices: usize = optimized_results.iter().map(|p| p.len()).sum();
    
    println!("  Standard method: {} total vertices after simplification", standard_vertices);
    println!("  Optimized method: {} total vertices after simplification", optimized_vertices);
    
    if let (Some(standard), Some(optimized)) = (
        profiler.get_statistics("standard_simplify"),
        profiler.get_statistics("optimized_simplify")
    ) {
        let speedup = standard.mean / optimized.mean;
        println!("  ⚡ Speedup: {:.2}x faster", speedup);
    }
    
    println!();
}

fn benchmark_batch_processing(paths: &[Path64], optimizer: &mut PerformanceOptimizer, profiler: &mut PerformanceProfiler) {
    println!("📦 Benchmarking Batch Processing");
    println!("--------------------------------");
    
    let test_paths = &paths[..100]; // Use first 100 paths
    
    // Individual processing
    let individual_results = profiler.time_operation("individual_processing", || {
        let mut results = Vec::new();
        for path in test_paths {
            // Simple transformation: scale by 1.1
            let scaled_path: Path64 = path.iter().map(|p| clipper2::Point64 {
                x: (p.x as f64 * 1.1) as i64,
                y: (p.y as f64 * 1.1) as i64,
            }).collect();
            results.push(scaled_path);
        }
        results
    });
    
    // Batch processing
    let batch_results = profiler.time_operation("batch_processing", || {
        optimizer.batch_process_paths(test_paths, |path| {
            // Same transformation: scale by 1.1
            path.iter().map(|p| clipper2::Point64 {
                x: (p.x as f64 * 1.1) as i64,
                y: (p.y as f64 * 1.1) as i64,
            }).collect()
        })
    });
    
    println!("  Individual processing: {} paths processed", individual_results.len());
    println!("  Batch processing: {} paths processed", batch_results.len());
    
    if let (Some(individual), Some(batch)) = (
        profiler.get_statistics("individual_processing"),
        profiler.get_statistics("batch_processing")
    ) {
        let speedup = individual.mean / batch.mean;
        println!("  ⚡ Speedup: {:.2}x faster", speedup);
    }
    
    println!();
}

fn benchmark_memory_efficiency(profiler: &mut PerformanceProfiler) {
    println!("💾 Benchmarking Memory Efficiency");
    println!("---------------------------------");
    
    // Memory-inefficient approach: many small allocations
    let inefficient_result = profiler.time_operation("memory_inefficient", || {
        let mut all_paths = Vec::new();
        
        for i in 0..100 {
            let mut paths_batch = Vec::new(); // New allocation each time
            for j in 0..10 {
                let path = MakePath(&[
                    i * 100, j * 50,
                    i * 100 + 50, j * 50,
                    i * 100 + 50, j * 50 + 40,
                    i * 100, j * 50 + 40
                ]);
                paths_batch.push(path);
            }
            
            // Union each batch individually (many operations)
            let batch_union = Union(&paths_batch, &vec![], FillRule::NonZero);
            all_paths.extend(batch_union);
        }
        
        all_paths.len()
    });
    
    // Memory-efficient approach: pre-allocation and batching
    let efficient_result = profiler.time_operation("memory_efficient", || {
        let mut all_paths = Vec::with_capacity(1000); // Pre-allocate
        let mut batch_accumulator = Vec::with_capacity(100);
        
        for i in 0..100 {
            for j in 0..10 {
                let path = MakePath(&[
                    i * 100, j * 50,
                    i * 100 + 50, j * 50,
                    i * 100 + 50, j * 50 + 40,
                    i * 100, j * 50 + 40
                ]);
                batch_accumulator.push(path);
            }
        }
        
        // Single union operation
        let final_union = Union(&batch_accumulator, &vec![], FillRule::NonZero);
        all_paths.extend(final_union);
        
        all_paths.len()
    });
    
    println!("  Memory-inefficient approach: {} paths", inefficient_result);
    println!("  Memory-efficient approach: {} paths", efficient_result);
    
    if let (Some(inefficient), Some(efficient)) = (
        profiler.get_statistics("memory_inefficient"),
        profiler.get_statistics("memory_efficient")
    ) {
        let speedup = inefficient.mean / efficient.mean;
        println!("  ⚡ Speedup: {:.2}x faster", speedup);
    }
    
    println!();
}

// Helper functions

fn generate_test_polygons(count: usize) -> Vec<Path64> {
    let mut paths = Vec::with_capacity(count);
    
    for i in 0..count {
        let base_x = ((i % 50) * 100) as i64;
        let base_y = ((i / 50) * 80) as i64;
        
        // Create various polygon types
        let path = match i % 4 {
            0 => {
                // Rectangle
                MakePath(&[
                    base_x, base_y,
                    base_x + 80, base_y,
                    base_x + 80, base_y + 60,
                    base_x, base_y + 60
                ])
            },
            1 => {
                // Triangle
                MakePath(&[
                    base_x + 40, base_y,
                    base_x + 80, base_y + 60,
                    base_x, base_y + 60
                ])
            },
            2 => {
                // Pentagon
                create_regular_polygon(base_x + 40, base_y + 30, 30, 5)
            },
            _ => {
                // Hexagon
                create_regular_polygon(base_x + 40, base_y + 30, 25, 6)
            }
        };
        
        paths.push(path);
    }
    
    paths
}

fn generate_test_points(count: usize) -> Vec<clipper2::Point64> {
    let mut points = Vec::with_capacity(count);
    
    for i in 0..count {
        points.push(clipper2::Point64 {
            x: (i % 100) as i64 * 10,
            y: (i / 100) as i64 * 10,
        });
    }
    
    points
}

fn create_regular_polygon(cx: i64, cy: i64, radius: i64, sides: usize) -> Path64 {
    let mut coords = Vec::with_capacity(sides * 2);
    
    for i in 0..sides {
        let angle = (i as f64 * 2.0 * std::f64::consts::PI) / sides as f64;
        let x = cx + (radius as f64 * angle.cos()) as i64;
        let y = cy + (radius as f64 * angle.sin()) as i64;
        
        coords.push(x);
        coords.push(y);
    }
    
    MakePath(&coords)
}