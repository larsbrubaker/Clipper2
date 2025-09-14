//! Complete Clipper64 implementation using the new engine

use crate::path::Paths64;
use crate::orientation::{ClipType, FillRule};
use crate::engine_complete::{ClipperEngine, PathType};

/// Complete Clipper64 implementation using the faithful C++ port
pub struct Clipper64Complete {
    engine: ClipperEngine,
    preserve_collinear: bool,
    reverse_solution: bool,
}

impl Clipper64Complete {
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

        // Execute the complete algorithm
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

    pub fn clear(&mut self) {
        self.engine.clear();
    }
}

/// Complete Union function using the new engine
pub fn union_complete(subjects: &Paths64, clips: &Paths64, fill_rule: FillRule) -> Paths64 {
    let mut clipper = Clipper64Complete::new();

    if !clipper.add_subject(subjects) || !clipper.add_clip(clips) {
        return Vec::new();
    }

    let mut solution = Paths64::new();
    let mut solution_open = Paths64::new();

    if clipper.execute(ClipType::Union, fill_rule, &mut solution, &mut solution_open) {
        solution
    } else {
        Vec::new()
    }
}