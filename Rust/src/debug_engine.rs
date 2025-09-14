//! Debug version of the test to trace what's happening in the engine

use crate::*;
use crate::orientation::FillRule;
use crate::clipper::Clipper64;
use crate::engine::ClipperEngine;

pub fn debug_negative_orientation_case() {
    println!("=== Debugging TestNegativeOrientation Case ===");

    let subjects = vec![
        MakePath(&[0, 0, 0, 100, 100, 100, 100, 0]),
        MakePath(&[10, 10, 10, 110, 110, 110, 110, 10])
    ];

    let clips = vec![
        MakePath(&[50, 50, 50, 150, 150, 150, 150, 50])
    ];

    println!("\nInput verification:");
    println!("subjects[0] area: {}", area(&subjects[0]));
    println!("subjects[1] area: {}", area(&subjects[1]));
    println!("clips[0] area: {}", area(&clips[0]));
    println!("subjects[0] is_positive: {}", IsPositive(&subjects[0]));
    println!("subjects[1] is_positive: {}", IsPositive(&subjects[1]));
    println!("clips[0] is_positive: {}", IsPositive(&clips[0]));

    // Enable debugging in engine by creating it directly
    let mut engine = ClipperEngine::new();

    // Add paths manually so we can trace
    println!("\nAdding subjects...");
    let subjects_ok = engine.add_paths(&subjects, crate::engine::PathType::Subject, false);
    println!("Add subjects result: {}", subjects_ok);

    println!("Adding clips...");
    let clips_ok = engine.add_paths(&clips, crate::engine::PathType::Clip, false);
    println!("Add clips result: {}", clips_ok);

    // Execute with tracing
    println!("\nExecuting with FillRule::Negative...");
    let mut solution = Vec::new();
    let mut solution_open = Vec::new();

    let result = engine.execute(
        crate::orientation::ClipType::Union,
        FillRule::Negative,
        &mut solution,
        &mut solution_open
    );

    println!("\nResults:");
    println!("Execution successful: {}", result);
    println!("Solution paths: {}", solution.len());
    println!("Open solution paths: {}", solution_open.len());

    if !solution.is_empty() {
        println!("First solution path: {} vertices", solution[0].len());
        println!("Vertices:");
        for (i, pt) in solution[0].iter().enumerate() {
            println!("  {}: ({}, {})", i, pt.x, pt.y);
        }
    }

    // Compare with different fill rules
    println!("\n=== Testing other fill rules ===");

    for fill_rule in [FillRule::NonZero, FillRule::EvenOdd, FillRule::Positive].iter() {
        let result = Union(&subjects, &clips, *fill_rule);
        println!("{:?}: {} paths, {} vertices in first", fill_rule, result.len(),
                if result.is_empty() { 0 } else { result[0].len() });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_negative_orientation() {
        debug_negative_orientation_case();
    }
}