//! Clipper64 implementation using the complete Vatti algorithm

use crate::path::Paths64;
use crate::orientation::{ClipType, FillRule};
use crate::engine::PathType;
use crate::engine::ClipperEngine;

/// Main clipping class using the complete Vatti algorithm
pub struct Clipper64 {
    engine: ClipperEngine,
    preserve_collinear: bool,
    reverse_solution: bool,
}

impl Clipper64 {
    pub fn new() -> Self {
        Self {
            engine: ClipperEngine::new(),
            preserve_collinear: true,
            reverse_solution: false,
        }
    }
    
    pub fn add_subject(&mut self, paths: &Paths64) -> bool {
        self.engine.add_paths(paths, PathType::Subject, false)
    }
    
    pub fn add_clip(&mut self, paths: &Paths64) -> bool {
        self.engine.add_paths(paths, PathType::Clip, false)
    }
    
    pub fn add_open_subject(&mut self, paths: &Paths64) -> bool {
        self.engine.add_paths(paths, PathType::Subject, true)
    }
    
    pub fn preserve_collinear(&mut self, value: bool) {
        self.preserve_collinear = value;
    }
    
    pub fn reverse_solution(&mut self, value: bool) {
        self.reverse_solution = value;
    }
    
    pub fn execute(
        &mut self,
        clip_type: ClipType,
        fill_rule: FillRule,
        solution: &mut Paths64,
        solution_open: &mut Paths64,
    ) -> bool {
        solution.clear();
        solution_open.clear();
        
        // Execute the full Vatti algorithm
        let result = self.engine.execute(clip_type, fill_rule, solution, solution_open);
        
        // Apply solution reversal if needed
        if self.reverse_solution && result {
            for path in solution.iter_mut() {
                path.reverse();
            }
            for path in solution_open.iter_mut() {
                path.reverse();
            }
        }
        
        result
    }
}