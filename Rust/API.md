# Clipper2 Rust API Documentation

This document provides comprehensive API documentation with examples for all public functions and types in the Clipper2 Rust library.

## Table of Contents

1. [Core Types](#core-types)
2. [Path Creation](#path-creation)
3. [Boolean Operations](#boolean-operations)
4. [Path Operations](#path-operations)
5. [Geometric Queries](#geometric-queries)
6. [Advanced Usage](#advanced-usage)
7. [Performance Module](#performance-module)
8. [Error Handling](#error-handling)

## Core Types

### Point64

Represents a 2D point with 64-bit integer coordinates.

```rust
use clipper2::Point64;

let point = Point64 { x: 100, y: 200 };
println!("Point: ({}, {})", point.x, point.y);
```

### Path64

A polygon path represented as a vector of Point64.

```rust
use clipper2::{Path64, Point64};

let path: Path64 = vec![
    Point64 { x: 0, y: 0 },
    Point64 { x: 100, y: 0 },
    Point64 { x: 100, y: 100 },
    Point64 { x: 0, y: 100 },
];
```

### Paths64

A collection of polygon paths.

```rust
use clipper2::{Paths64, Path64, MakePath};

let mut paths: Paths64 = Vec::new();
paths.push(MakePath(&[0, 0, 100, 0, 100, 100, 0, 100]));
paths.push(MakePath(&[50, 50, 150, 50, 150, 150, 50, 150]));
```

### Enumerations

#### FillRule

Determines how overlapping polygons are filled.

```rust
use clipper2::FillRule;

// Available fill rules:
let rules = [
    FillRule::EvenOdd,   // Standard even-odd rule
    FillRule::NonZero,   // Non-zero winding rule
    FillRule::Positive,  // Only positive windings
    FillRule::Negative,  // Only negative windings
];
```

#### ClipType

Specifies the type of boolean operation.

```rust
use clipper2::ClipType;

let operations = [
    ClipType::Union,        // A ∪ B
    ClipType::Intersection, // A ∩ B  
    ClipType::Difference,   // A - B
    ClipType::Xor,         // A ⊕ B
];
```

#### JoinType

Controls how path joints are rendered during offsetting.

```rust
use clipper2::JoinType;

let join_types = [
    JoinType::Round,  // Rounded joints
    JoinType::Square, // Square joints
    JoinType::Miter,  // Mitered joints
];
```

#### EndType

Controls how path ends are rendered during offsetting.

```rust
use clipper2::EndType;

let end_types = [
    EndType::Polygon,     // Closed polygon (joins end to start)
    EndType::Joined,      // Joins end to start  
    EndType::Butt,       // Flat end
    EndType::Square,     // Square end
    EndType::Round,      // Rounded end
];
```

## Path Creation

### MakePath

Creates a Path64 from coordinate pairs.

```rust
use clipper2::MakePath;

// Create a rectangle
let rect = MakePath(&[0, 0, 100, 0, 100, 100, 0, 100]);

// Create a triangle  
let triangle = MakePath(&[0, 0, 50, 0, 25, 50]);

// Create a complex polygon
let complex = MakePath(&[
    10, 10,   // Point 1
    100, 10,  // Point 2
    100, 50,  // Point 3
    150, 50,  // Point 4
    150, 100, // Point 5
    10, 100,  // Point 6
]);

println!("Rectangle has {} vertices", rect.len());
```

### Creating Shapes Programmatically

```rust
use clipper2::{MakePath, Path64, Point64};
use std::f64::consts::PI;

// Create a regular polygon
fn create_regular_polygon(cx: i64, cy: i64, radius: i64, sides: usize) -> Path64 {
    let mut coords = Vec::with_capacity(sides * 2);
    
    for i in 0..sides {
        let angle = (i as f64 * 2.0 * PI) / sides as f64;
        let x = cx + (radius as f64 * angle.cos()) as i64;
        let y = cy + (radius as f64 * angle.sin()) as i64;
        coords.extend_from_slice(&[x, y]);
    }
    
    MakePath(&coords)
}

// Create a star shape
fn create_star(cx: i64, cy: i64, outer: i64, inner: i64, points: usize) -> Path64 {
    let mut coords = Vec::with_capacity(points * 4);
    
    for i in 0..(points * 2) {
        let angle = (i as f64 * PI) / points as f64;
        let radius = if i % 2 == 0 { outer } else { inner };
        let x = cx + (radius as f64 * angle.cos()) as i64;
        let y = cy + (radius as f64 * angle.sin()) as i64;
        coords.extend_from_slice(&[x, y]);
    }
    
    MakePath(&coords)
}

// Usage
let hexagon = create_regular_polygon(100, 100, 50, 6);
let star = create_star(200, 200, 80, 40, 8);
```

## Boolean Operations

### Global Functions (C++ Compatible API)

These functions provide a simple interface for common operations.

#### Union

Combines all polygons into a single result.

```rust
use clipper2::{Union, MakePath, FillRule};

let rect1 = MakePath(&[0, 0, 100, 0, 100, 100, 0, 100]);
let rect2 = MakePath(&[50, 50, 150, 50, 150, 150, 50, 150]);

let subjects = vec![rect1];
let clips = vec![rect2];

let result = Union(&subjects, &clips, FillRule::NonZero);
println!("Union produced {} paths", result.len());

// Self-union (merge overlapping paths in a single collection)
let overlapping_rects = vec![rect1, rect2];
let merged = Union(&overlapping_rects, &vec![], FillRule::NonZero);
```

#### Intersect

Finds the intersection of subject and clip polygons.

```rust
use clipper2::{Intersect, MakePath, FillRule, area};

let circle = create_regular_polygon(0, 0, 100, 32);
let square = MakePath(&[-50, -50, 50, -50, 50, 50, -50, 50]);

let intersection = Intersect(&vec![circle], &vec![square], FillRule::EvenOdd);

for (i, path) in intersection.iter().enumerate() {
    println!("Intersection part {}: area = {:.1}", i, area(path).abs());
}
```

#### Difference

Subtracts clip polygons from subject polygons.

```rust
use clipper2::{Difference, MakePath, FillRule};

// Cut a hole in a rectangle
let outer_rect = MakePath(&[0, 0, 200, 0, 200, 200, 0, 200]);
let inner_rect = MakePath(&[50, 50, 150, 50, 150, 150, 50, 150]);

let result = Difference(&vec![outer_rect], &vec![inner_rect], FillRule::NonZero);
println!("Rectangle with hole has {} paths", result.len());
```

#### Xor

Finds the exclusive OR (symmetric difference) of polygons.

```rust
use clipper2::{Xor, MakePath, FillRule};

let rect1 = MakePath(&[0, 0, 100, 0, 100, 100, 0, 100]);
let rect2 = MakePath(&[50, 50, 150, 50, 150, 150, 50, 150]);

let xor_result = Xor(&vec![rect1], &vec![rect2], FillRule::NonZero);
println!("XOR produced {} paths", xor_result.len());
```

### Engine API (Advanced Usage)

For more control over the clipping process, use the Clipper64 engine directly.

```rust
use clipper2::{Clipper64, ClipType, FillRule, MakePath, Paths64};

let mut clipper = Clipper64::new();

// Add subject paths (the primary polygons)
let subjects = vec![MakePath(&[0, 0, 100, 0, 100, 100, 0, 100])];
clipper.add_subject(&subjects);

// Add clip paths (the cutting/intersecting polygons)
let clips = vec![MakePath(&[50, 50, 150, 50, 150, 150, 50, 150])];
clipper.add_clip(&clips);

// Execute the operation
let mut solution = Paths64::new();
let mut open_paths = Paths64::new();

let success = clipper.execute(
    ClipType::Intersection,
    FillRule::NonZero,
    &mut solution,
    &mut open_paths
);

if success {
    println!("Clipping succeeded: {} closed paths, {} open paths", 
             solution.len(), open_paths.len());
} else {
    println!("Clipping failed");
}
```

### Fill Rule Examples

```rust
use clipper2::{Union, MakePath, FillRule};

// Create a self-intersecting polygon (figure-8)
let figure_eight = MakePath(&[
    0, 0,       // Bottom left
    100, 100,   // Top right
    200, 0,     // Bottom right
    200, 200,   // Top right
    100, 100,   // Center (repeated)
    0, 200,     // Top left
]);

// Different fill rules produce different results
let even_odd = Union(&vec![figure_eight.clone()], &vec![], FillRule::EvenOdd);
let non_zero = Union(&vec![figure_eight.clone()], &vec![], FillRule::NonZero);

println!("EvenOdd fill: {} paths", even_odd.len());
println!("NonZero fill: {} paths", non_zero.len());
```

## Path Operations

### InflatePaths

Expands or contracts polygons by a specified distance.

```rust
use clipper2::{InflatePaths, MakePath, JoinType, EndType, area};

let original = MakePath(&[0, 0, 100, 0, 100, 100, 0, 100]);

// Expand by 10 units
let expanded = InflatePaths(&vec![original.clone()], 10.0, JoinType::Round, EndType::Polygon);

// Contract by 5 units  
let contracted = InflatePaths(&vec![original.clone()], -5.0, JoinType::Round, EndType::Polygon);

println!("Original area: {:.1}", area(&original).abs());
if !expanded.is_empty() {
    println!("Expanded area: {:.1}", area(&expanded[0]).abs());
}
if !contracted.is_empty() {
    println!("Contracted area: {:.1}", area(&contracted[0]).abs());
}
```

### Different Join Types

```rust
use clipper2::{InflatePaths, MakePath, JoinType, EndType};

let star = create_star(0, 0, 100, 50, 8);
let paths = vec![star];

// Round joins (smooth curves)
let round_result = InflatePaths(&paths, 20.0, JoinType::Round, EndType::Polygon);

// Square joins (extended corners)
let square_result = InflatePaths(&paths, 20.0, JoinType::Square, EndType::Polygon);

// Miter joins (sharp corners, may be clipped at acute angles)
let miter_result = InflatePaths(&paths, 20.0, JoinType::Miter, EndType::Polygon);

println!("Round: {} paths, Square: {} paths, Miter: {} paths",
         round_result.len(), square_result.len(), miter_result.len());
```

### SimplifyPath

Removes near-collinear points to reduce polygon complexity.

```rust
use clipper2::{SimplifyPath, MakePath};

// Create a path with many near-collinear points
let complex_path = MakePath(&[
    0, 0,     // Start
    10, 1,    // Nearly collinear
    20, 2,    // Nearly collinear  
    30, 0,    // Corner
    100, 0,   // End of edge
    100, 100, // Corner
    0, 100,   // Corner
]);

// Simplify with 2.0 unit tolerance
let simplified = SimplifyPath(&complex_path, 2.0, true);

println!("Original: {} vertices, Simplified: {} vertices", 
         complex_path.len(), simplified.len());
```

## Geometric Queries

### area

Calculates the signed area of a polygon.

```rust
use clipper2::{area, MakePath};

let square = MakePath(&[0, 0, 100, 0, 100, 100, 0, 100]);
let area_value = area(&square);

println!("Area: {:.1} (absolute: {:.1})", area_value, area_value.abs());

// Positive area = counter-clockwise orientation
// Negative area = clockwise orientation
if area_value > 0.0 {
    println!("Counter-clockwise orientation");
} else if area_value < 0.0 {
    println!("Clockwise orientation");  
} else {
    println!("Degenerate polygon (zero area)");
}
```

### IsPositive

Determines if a polygon has positive (counter-clockwise) orientation.

```rust
use clipper2::{IsPositive, MakePath};

let ccw_square = MakePath(&[0, 0, 100, 0, 100, 100, 0, 100]);
let cw_square = MakePath(&[0, 0, 0, 100, 100, 100, 100, 0]);

println!("CCW square is positive: {}", IsPositive(&ccw_square));
println!("CW square is positive: {}", IsPositive(&cw_square));
```

### Length

Calculates the perimeter length of a path.

```rust
use clipper2::{Length, MakePath};

let square = MakePath(&[0, 0, 100, 0, 100, 100, 0, 100]);
let length = Length(&square);

println!("Square perimeter: {:.1}", length); // Should be 400.0
```

## Advanced Usage

### Complex Multi-Step Operations

```rust
use clipper2::{Clipper64, ClipType, FillRule, InflatePaths, JoinType, EndType, MakePath};

// Multi-step machining simulation
fn generate_toolpath(part: &clipper2::Path64, tool_diameter: f64) -> clipper2::Paths64 {
    let tool_radius = tool_diameter / 2.0;
    let step_over = tool_diameter * 0.6; // 60% stepover
    
    let mut toolpaths = Vec::new();
    let mut current_boundary = vec![part.clone()];
    
    // Generate concentric toolpaths
    loop {
        // Offset inward by tool radius
        let offset = InflatePaths(&current_boundary, -tool_radius, JoinType::Round, EndType::Polygon);
        
        if offset.is_empty() {
            break; // No more material to cut
        }
        
        toolpaths.extend(offset.clone());
        
        // Next pass - offset by stepover distance
        current_boundary = InflatePaths(&offset, -step_over, JoinType::Round, EndType::Polygon);
        
        if current_boundary.is_empty() {
            break;
        }
    }
    
    toolpaths
}

// Usage
let part = create_star(0, 0, 100, 60, 8);
let toolpath = generate_toolpath(&part, 10.0);
println!("Generated {} toolpath passes", toolpath.len());
```

### Batch Processing

```rust
use clipper2::{Union, FillRule, MakePath, Paths64};

fn process_many_polygons(polygons: &[clipper2::Path64]) -> Paths64 {
    const BATCH_SIZE: usize = 100;
    let mut final_result = Vec::new();
    
    // Process in batches to avoid memory pressure
    for batch in polygons.chunks(BATCH_SIZE) {
        let batch_result = Union(&batch.to_vec(), &vec![], FillRule::NonZero);
        final_result.extend(batch_result);
    }
    
    // Final merge
    Union(&final_result, &vec![], FillRule::NonZero)
}

// Generate test polygons
let mut polygons = Vec::new();
for i in 0..1000 {
    let x = (i % 50) * 20;
    let y = (i / 50) * 20;
    polygons.push(MakePath(&[x, y, x+15, y, x+15, y+15, x, y+15]));
}

let merged = process_many_polygons(&polygons);
println!("Merged {} polygons into {} result paths", polygons.len(), merged.len());
```

## Performance Module

The performance module provides optimized algorithms and profiling tools.

```rust
use clipper2::performance::{PerformanceOptimizer, PerformanceProfiler};

let mut optimizer = PerformanceOptimizer::new();
let mut profiler = PerformanceProfiler::new();

// Fast area calculation
let path = clipper2::MakePath(&[0, 0, 100, 0, 100, 100, 0, 100]);
let fast_area = PerformanceOptimizer::fast_area_calculation(&path);

// Fast point-in-polygon test
let point = clipper2::Point64 { x: 50, y: 50 };
let is_inside = optimizer.fast_point_in_polygon(point, &path);

// Performance profiling
let result = profiler.time_operation("my_operation", || {
    // Some expensive computation
    std::thread::sleep(std::time::Duration::from_millis(1));
    42
});

// Print performance statistics
if let Some(stats) = profiler.get_statistics("my_operation") {
    println!("Operation stats: {}", stats);
}
```

## Error Handling

Most operations in Clipper2 Rust use return values rather than exceptions:

```rust
use clipper2::{Clipper64, ClipType, FillRule, MakePath, Paths64};

let mut clipper = Clipper64::new();

// Check if paths were added successfully
let subjects = vec![MakePath(&[0, 0, 100, 0, 100, 100, 0, 100])];
if !clipper.add_subject(&subjects) {
    eprintln!("Failed to add subject paths");
    return;
}

let clips = vec![MakePath(&[50, 50, 150, 50, 150, 150, 50, 150])];
if !clipper.add_clip(&clips) {
    eprintln!("Failed to add clip paths");
    return;
}

// Check if execution succeeded
let mut solution = Paths64::new();
let mut open_paths = Paths64::new();

match clipper.execute(ClipType::Union, FillRule::NonZero, &mut solution, &mut open_paths) {
    true => {
        println!("Success: {} paths generated", solution.len());
        
        // Validate results
        for (i, path) in solution.iter().enumerate() {
            if path.len() < 3 {
                eprintln!("Warning: Path {} has only {} vertices", i, path.len());
            }
        }
    },
    false => {
        eprintln!("Clipping operation failed");
        // Handle failure gracefully
    }
}
```

### Validation Functions

```rust
use clipper2::{area, MakePath};

fn validate_polygon(path: &clipper2::Path64) -> Result<(), String> {
    if path.len() < 3 {
        return Err(format!("Polygon has only {} vertices (minimum 3 required)", path.len()));
    }
    
    let area_val = area(path).abs();
    if area_val < 1e-10 {
        return Err("Polygon has zero or near-zero area".to_string());
    }
    
    // Check for duplicate consecutive points
    for i in 0..path.len() {
        let current = &path[i];
        let next = &path[(i + 1) % path.len()];
        if current.x == next.x && current.y == next.y {
            return Err(format!("Duplicate consecutive points at index {}", i));
        }
    }
    
    Ok(())
}

// Usage
let polygon = MakePath(&[0, 0, 100, 0, 100, 100, 0, 100]);
match validate_polygon(&polygon) {
    Ok(()) => println!("Polygon is valid"),
    Err(e) => eprintln!("Polygon validation failed: {}", e),
}
```

## Best Practices

1. **Memory Management**: Pre-allocate vectors when processing many polygons
2. **Performance**: Use `--release` flag for production builds
3. **Precision**: Be aware of integer coordinate precision limits
4. **Fill Rules**: Choose appropriate fill rules for your use case
5. **Validation**: Validate input polygons before processing
6. **Batch Processing**: Process large datasets in chunks
7. **Error Handling**: Always check return values for success/failure

For more examples and advanced usage patterns, see the comprehensive examples in the `/examples` directory.