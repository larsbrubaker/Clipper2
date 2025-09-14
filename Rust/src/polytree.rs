//! PolyTree hierarchical polygon structures
//!
//! This module provides hierarchical representations of polygons that maintain
//! parent-child relationships for holes and islands. Unlike flat path lists,
//! PolyTrees preserve the nesting structure of complex polygons.

use crate::path::{Path64, PathD, Paths64, PathsD};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// A node in the PolyTree hierarchy for 64-bit integer coordinates
#[derive(Debug, Clone)]
pub struct PolyNode64 {
    /// The polygon path for this node
    pub polygon: Path64,
    /// Reference to parent node (weak to avoid cycles)
    pub parent: Option<Weak<RefCell<PolyNode64>>>,
    /// Child nodes (holes if this is an outer polygon, islands if this is a hole)
    pub children: Vec<Rc<RefCell<PolyNode64>>>,
    /// Whether this node represents a hole (true) or solid polygon (false)
    pub is_hole: bool,
}

/// A node in the PolyTree hierarchy for floating-point coordinates  
#[derive(Debug, Clone)]
pub struct PolyNodeD {
    /// The polygon path for this node
    pub polygon: PathD,
    /// Reference to parent node (weak to avoid cycles)
    pub parent: Option<Weak<RefCell<PolyNodeD>>>,
    /// Child nodes (holes if this is an outer polygon, islands if this is a hole)  
    pub children: Vec<Rc<RefCell<PolyNodeD>>>,
    /// Whether this node represents a hole (true) or solid polygon (false)
    pub is_hole: bool,
}

/// Hierarchical polygon tree for 64-bit integer coordinates
#[derive(Debug, Clone)]
pub struct PolyTree64 {
    /// Root nodes (top-level polygons with no parents)
    pub children: Vec<Rc<RefCell<PolyNode64>>>,
}

/// Hierarchical polygon tree for floating-point coordinates
#[derive(Debug, Clone)]
pub struct PolyTreeD {
    /// Root nodes (top-level polygons with no parents)  
    pub children: Vec<Rc<RefCell<PolyNodeD>>>,
}

impl PolyNode64 {
    /// Creates a new PolyNode64
    pub fn new(polygon: Path64, is_hole: bool) -> Self {
        Self {
            polygon,
            parent: None,
            children: Vec::new(),
            is_hole,
        }
    }

    /// Adds a child node to this node
    pub fn add_child(&mut self, child: Rc<RefCell<PolyNode64>>) {
        self.children.push(child);
    }

    /// Gets the depth/nesting level of this node (root = 0)
    pub fn depth(&self) -> usize {
        match &self.parent {
            Some(parent_weak) => {
                if let Some(parent_rc) = parent_weak.upgrade() {
                    parent_rc.borrow().depth() + 1
                } else {
                    0 // Parent was dropped, treat as root
                }
            }
            None => 0,
        }
    }

    /// Returns true if this node is a top-level outer polygon
    pub fn is_outer(&self) -> bool {
        !self.is_hole
    }

    /// Counts total number of descendant nodes
    pub fn descendant_count(&self) -> usize {
        let mut count = 0;
        for child in &self.children {
            count += 1 + child.borrow().descendant_count();
        }
        count
    }
}

impl PolyNodeD {
    /// Creates a new PolyNodeD
    pub fn new(polygon: PathD, is_hole: bool) -> Self {
        Self {
            polygon,
            parent: None,  
            children: Vec::new(),
            is_hole,
        }
    }

    /// Adds a child node to this node
    pub fn add_child(&mut self, child: Rc<RefCell<PolyNodeD>>) {
        self.children.push(child);
    }

    /// Gets the depth/nesting level of this node (root = 0)  
    pub fn depth(&self) -> usize {
        match &self.parent {
            Some(parent_weak) => {
                if let Some(parent_rc) = parent_weak.upgrade() {
                    parent_rc.borrow().depth() + 1
                } else {
                    0 // Parent was dropped, treat as root
                }
            }
            None => 0,
        }
    }

    /// Returns true if this node is a top-level outer polygon
    pub fn is_outer(&self) -> bool {
        !self.is_hole
    }

    /// Counts total number of descendant nodes
    pub fn descendant_count(&self) -> usize {
        let mut count = 0;
        for child in &self.children {
            count += 1 + child.borrow().descendant_count();
        }
        count
    }
}

impl PolyTree64 {
    /// Creates a new empty PolyTree64
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    /// Adds a root-level polygon to the tree
    pub fn add_root(&mut self, polygon: Path64, is_hole: bool) -> Rc<RefCell<PolyNode64>> {
        let node = Rc::new(RefCell::new(PolyNode64::new(polygon, is_hole)));
        self.children.push(node.clone());
        node
    }

    /// Counts total number of nodes in the tree
    pub fn total_count(&self) -> usize {
        let mut count = 0;
        for child in &self.children {
            count += 1 + child.borrow().descendant_count();
        }
        count
    }

    /// Gets all outer polygons (non-holes) from the tree
    pub fn get_outers(&self) -> Vec<Rc<RefCell<PolyNode64>>> {
        let mut outers = Vec::new();
        self.collect_outers(&self.children, &mut outers);
        outers
    }

    /// Recursively collects outer polygons
    fn collect_outers(&self, nodes: &[Rc<RefCell<PolyNode64>>], outers: &mut Vec<Rc<RefCell<PolyNode64>>>) {
        for node_rc in nodes {
            let node = node_rc.borrow();
            if node.is_outer() {
                outers.push(node_rc.clone());
            }
            self.collect_outers(&node.children, outers);
        }
    }

    /// Clears all nodes from the tree
    pub fn clear(&mut self) {
        self.children.clear();
    }
}

impl PolyTreeD {
    /// Creates a new empty PolyTreeD  
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    /// Adds a root-level polygon to the tree
    pub fn add_root(&mut self, polygon: PathD, is_hole: bool) -> Rc<RefCell<PolyNodeD>> {
        let node = Rc::new(RefCell::new(PolyNodeD::new(polygon, is_hole)));
        self.children.push(node.clone());
        node
    }

    /// Counts total number of nodes in the tree  
    pub fn total_count(&self) -> usize {
        let mut count = 0;
        for child in &self.children {
            count += 1 + child.borrow().descendant_count();
        }
        count
    }

    /// Gets all outer polygons (non-holes) from the tree
    pub fn get_outers(&self) -> Vec<Rc<RefCell<PolyNodeD>>> {
        let mut outers = Vec::new();
        self.collect_outers(&self.children, &mut outers);
        outers
    }

    /// Recursively collects outer polygons
    fn collect_outers(&self, nodes: &[Rc<RefCell<PolyNodeD>>], outers: &mut Vec<Rc<RefCell<PolyNodeD>>>) {
        for node_rc in nodes {
            let node = node_rc.borrow();
            if node.is_outer() {
                outers.push(node_rc.clone());
            }
            self.collect_outers(&node.children, outers);
        }
    }

    /// Clears all nodes from the tree
    pub fn clear(&mut self) {
        self.children.clear();
    }
}

/// Converts a PolyTree64 to a flat list of paths
/// 
/// # Arguments
/// * `polytree` - The PolyTree64 to convert
/// 
/// # Returns
/// A `Paths64` containing all polygon paths from the tree in depth-first order
pub fn polytree_to_paths64(polytree: &PolyTree64) -> Paths64 {
    let mut paths = Paths64::new();
    for child in &polytree.children {
        collect_paths64(&child.borrow(), &mut paths);
    }
    paths
}

/// Recursively collects paths from PolyNode64 hierarchy  
fn collect_paths64(node: &PolyNode64, paths: &mut Paths64) {
    paths.push(node.polygon.clone());
    for child in &node.children {
        collect_paths64(&child.borrow(), paths);
    }
}

/// Converts a PolyTreeD to a flat list of paths
///
/// # Arguments  
/// * `polytree` - The PolyTreeD to convert
///
/// # Returns
/// A `PathsD` containing all polygon paths from the tree in depth-first order
pub fn polytree_to_paths_d(polytree: &PolyTreeD) -> PathsD {
    let mut paths = PathsD::new();
    for child in &polytree.children {
        collect_paths_d(&child.borrow(), &mut paths);
    }
    paths
}

/// Recursively collects paths from PolyNodeD hierarchy
fn collect_paths_d(node: &PolyNodeD, paths: &mut PathsD) {
    paths.push(node.polygon.clone());
    for child in &node.children {
        collect_paths_d(&child.borrow(), paths);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point64;

    #[test]
    fn test_polytree_basic() {
        let mut tree = PolyTree64::new();
        
        // Create a simple square outer polygon
        let outer = vec![
            Point64::new(0, 0),
            Point64::new(10, 0), 
            Point64::new(10, 10),
            Point64::new(0, 10),
        ];
        
        let outer_node = tree.add_root(outer.clone(), false);
        
        // Add a hole inside the square
        let hole = vec![
            Point64::new(2, 2),
            Point64::new(8, 2),
            Point64::new(8, 8), 
            Point64::new(2, 8),
        ];
        
        let hole_node = Rc::new(RefCell::new(PolyNode64::new(hole.clone(), true)));
        hole_node.borrow_mut().parent = Some(Rc::downgrade(&outer_node));
        outer_node.borrow_mut().add_child(hole_node);
        
        assert_eq!(tree.total_count(), 2);
        assert_eq!(tree.get_outers().len(), 1);
        
        let paths = polytree_to_paths64(&tree);
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], outer);
        assert_eq!(paths[1], hole);
    }

    #[test] 
    fn test_node_depth() {
        let mut tree = PolyTree64::new();
        
        let outer = vec![Point64::new(0, 0), Point64::new(10, 0), Point64::new(10, 10), Point64::new(0, 10)];
        let outer_node = tree.add_root(outer, false);
        
        assert_eq!(outer_node.borrow().depth(), 0);
        
        let hole = vec![Point64::new(2, 2), Point64::new(8, 2), Point64::new(8, 8), Point64::new(2, 8)];
        let hole_node = Rc::new(RefCell::new(PolyNode64::new(hole, true)));
        hole_node.borrow_mut().parent = Some(Rc::downgrade(&outer_node));
        outer_node.borrow_mut().add_child(hole_node.clone());
        
        assert_eq!(hole_node.borrow().depth(), 1);
        
        // Add an island inside the hole
        let island = vec![Point64::new(4, 4), Point64::new(6, 4), Point64::new(6, 6), Point64::new(4, 6)];
        let island_node = Rc::new(RefCell::new(PolyNode64::new(island, false)));
        island_node.borrow_mut().parent = Some(Rc::downgrade(&hole_node));
        hole_node.borrow_mut().add_child(island_node.clone());
        
        assert_eq!(island_node.borrow().depth(), 2);
    }
}