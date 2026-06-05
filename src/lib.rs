//! # ternary-core
//!
//! Core traits and types shared across the ternary fleet.
//! Every ternary crate should depend on this for common abstractions:
//! `TernaryValue`, `TernaryGrid`, `TernaryGraph`, and Z₃ arithmetic.

#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;
use alloc::{vec, vec::Vec};

// ============================================================================
// CORE ARITHMETIC
// ============================================================================

/// Add two values modulo 3, returning {-1, 0, 1}
pub fn tadd(a: i8, b: i8) -> i8 {
    match ((a + b) % 3 + 3) % 3 {
        0 => 0, 1 => 1, _ => -1,
    }
}

/// Subtract b from a modulo 3
pub fn tsub(a: i8, b: i8) -> i8 {
    tadd(a, -b)
}

/// Multiply two values modulo 3
pub fn tmul(a: i8, b: i8) -> i8 {
    match ((a * b) % 3 + 3) % 3 {
        0 => 0, 1 => 1, _ => -1,
    }
}

/// Negate a value modulo 3
pub fn tneg(a: i8) -> i8 {
    match a { 1 => -1, -1 => 1, _ => 0 }
}

/// Inverse of a value modulo 3 (only 1 and -1 have inverses)
pub fn tinv(a: i8) -> Option<i8> {
    match a {
        1 => Some(1),
        -1 => Some(-1),
        _ => None,
    }
}

/// Clamp any integer to ternary range {-1, 0, 1}
pub fn tclamp(v: i8) -> i8 {
    v.clamp(-1, 1)
}

/// Modular distance: shortest path on Z₃ circle
pub fn tdist(a: i8, b: i8) -> i8 {
    let d = ((a - b) % 3 + 3) % 3;
    if d <= 1 { d } else { 3 - d }
}

/// Inner product of two ternary vectors mod 3
pub fn tdot(a: &[i8], b: &[i8]) -> i8 {
    a.iter().zip(b.iter()).map(|(&x, &y)| tmul(x, y)).fold(0i8, |acc, v| tadd(acc, v))
}

// ============================================================================
// TERNARY VALUE TRAIT
// ============================================================================

/// A type that can be represented as a ternary value {-1, 0, 1}
pub trait TernaryValue: Copy + PartialEq + Eq + core::fmt::Debug {
    fn to_ternary(self) -> i8;
    fn from_ternary(v: i8) -> Self;
    fn is_positive(self) -> bool { self.to_ternary() > 0 }
    fn is_negative(self) -> bool { self.to_ternary() < 0 }
    fn is_zero(self) -> bool { self.to_ternary() == 0 }
}

impl TernaryValue for i8 {
    fn to_ternary(self) -> i8 { self.clamp(-1, 1) }
    fn from_ternary(v: i8) -> Self { v.clamp(-1, 1) }
}

impl TernaryValue for bool {
    fn to_ternary(self) -> i8 { if self { 1 } else { 0 } }
    fn from_ternary(v: i8) -> Self { v > 0 }
}

// ============================================================================
// TERNARY GRID
// ============================================================================

/// A 2D grid of ternary values with common operations
#[derive(Debug, Clone)]
pub struct TernaryGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<i8>,
}

impl TernaryGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, cells: vec![0; width * height] }
    }

    pub fn get(&self, x: usize, y: usize) -> i8 {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x]
        } else { 0 }
    }

    pub fn set(&mut self, x: usize, y: usize, v: i8) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = v.clamp(-1, 1);
        }
    }

    /// Sum of values
    pub fn sum(&self) -> i32 {
        self.cells.iter().map(|&v| v as i32).sum()
    }

    /// Average value (clamped to ternary)
    pub fn average(&self) -> i8 {
        let s = self.sum();
        let n = self.cells.len() as i32;
        if n == 0 { return 0; }
        (s * 3 / n).clamp(-1, 1) as i8
    }

    /// Count of each value: (negatives, zeros, positives)
    pub fn histogram(&self) -> (usize, usize, usize) {
        self.cells.iter().fold((0, 0, 0), |(neg, zero, pos), &v| {
            match v {
                -1 => (neg + 1, zero, pos),
                0 => (neg, zero + 1, pos),
                1 => (neg, zero, pos + 1),
                _ => (neg, zero, pos),
            }
        })
    }

    /// Map each cell through a function
    pub fn map<F: Fn(i8) -> i8>(&self, f: F) -> Self {
        Self {
            width: self.width,
            height: self.height,
            cells: self.cells.iter().map(|&v| f(v)).collect(),
        }
    }

    /// Zip two grids together with a function
    pub fn zip_with<F: Fn(i8, i8) -> i8>(&self, other: &Self, f: F) -> Self {
        let mut result = Self::new(self.width.max(other.width), self.height.max(other.height));
        for y in 0..result.height {
            for x in 0..result.width {
                result.set(x, y, f(self.get(x, y), other.get(x, y)));
            }
        }
        result
    }

    /// Discrete Laplacian at position (x, y)
    pub fn laplacian_at(&self, x: usize, y: usize) -> i8 {
        let center = self.get(x, y);
        let mut neighbor_sum: i8 = 0;
        let mut count: i8 = 0;
        let offsets: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in offsets {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 {
                let n = self.get(nx as usize, ny as usize);
                neighbor_sum = tadd(neighbor_sum, n);
                count += 1;
            }
        }
        // Laplacian ≈ neighbor_sum - count * center
        let lap = neighbor_sum as i8 - count * center;
        lap.clamp(-1, 1)
    }

    /// Neighbor count: how many of the 8 (Moore) or 4 (von Neumann) neighbors have value v?
    pub fn neighbor_count(&self, x: usize, y: usize, v: i8, moore: bool) -> usize {
        let mut count = 0;
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                if !moore && dx != 0 && dy != 0 { continue; }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 && self.get(nx as usize, ny as usize) == v {
                    count += 1;
                }
            }
        }
        count
    }

    /// Fill entire grid with a value
    pub fn fill(&mut self, v: i8) {
        for c in &mut self.cells { *c = v.clamp(-1, 1); }
    }

    /// Count cells matching predicate
    pub fn count<F: Fn(i8) -> bool>(&self, pred: F) -> usize {
        self.cells.iter().filter(|&&v| pred(v)).count()
    }
}

// ============================================================================
// TERNARY GRAPH
// ============================================================================

/// A graph with ternary edge weights {-1, 0, 1}
#[derive(Debug, Clone)]
pub struct TernaryGraph {
    pub n: usize,
    pub adjacency: Vec<Vec<i8>>,
}

impl TernaryGraph {
    pub fn new(n: usize) -> Self {
        Self { n, adjacency: vec![vec![0i8; n]; n] }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: i8) {
        self.adjacency[from][to] = weight.clamp(-1, 1);
    }

    pub fn add_undirected(&mut self, a: usize, b: usize, weight: i8) {
        let w = weight.clamp(-1, 1);
        self.adjacency[a][b] = w;
        self.adjacency[b][a] = w;
    }

    pub fn edge(&self, from: usize, to: usize) -> i8 {
        self.adjacency[from][to]
    }

    pub fn neighbors(&self, node: usize) -> Vec<usize> {
        (0..self.n).filter(|&j| self.adjacency[node][j] != 0).collect()
    }

    pub fn positive_neighbors(&self, node: usize) -> Vec<usize> {
        (0..self.n).filter(|&j| self.adjacency[node][j] > 0).collect()
    }

    pub fn degree(&self, node: usize) -> usize {
        self.neighbors(node).len()
    }

    /// BFS from source, following only positive-weight edges
    pub fn bfs(&self, source: usize) -> Vec<Option<usize>> {
        let mut dist = vec![None; self.n];
        dist[source] = Some(0);
        let mut queue = vec![source];
        let mut head = 0;
        while head < queue.len() {
            let u = queue[head];
            head += 1;
            for &v in &self.positive_neighbors(u) {
                if dist[v].is_none() {
                    dist[v] = Some(dist[u].unwrap() + 1);
                    queue.push(v);
                }
            }
        }
        dist
    }

    /// Is the graph connected (positive edges only)?
    pub fn is_connected(&self) -> bool {
        if self.n == 0 { return true; }
        let dist = self.bfs(0);
        dist.iter().all(|d| d.is_some())
    }

    /// Connected components (positive edges only)
    pub fn components(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut comps = vec![];
        for start in 0..self.n {
            if visited[start] { continue; }
            let mut comp = vec![];
            let mut stack = vec![start];
            while let Some(node) = stack.pop() {
                if visited[node] { continue; }
                visited[node] = true;
                comp.push(node);
                for &nb in &self.positive_neighbors(node) {
                    if !visited[nb] { stack.push(nb); }
                }
            }
            comps.push(comp);
        }
        comps
    }

    /// Total weight of all edges
    pub fn total_weight(&self) -> i32 {
        let mut w = 0i32;
        for i in 0..self.n {
            for j in 0..self.n {
                w += self.adjacency[i][j] as i32;
            }
        }
        w
    }
}

// ============================================================================
// COMMON PATTERNS
// ============================================================================

/// A system that evolves in discrete steps with ternary state
pub trait TernaryDynamics {
    type State;
    fn step(&mut self);
    fn state(&self) -> &Self::State;
    fn run(&mut self, steps: usize) {
        for _ in 0..steps { self.step(); }
    }
}

/// A measurement that produces a ternary summary
pub trait TernaryMeasure<T> {
    fn measure(&self, input: &T) -> i8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tadd() {
        assert_eq!(tadd(1, 1), -1);
        assert_eq!(tadd(-1, -1), 1);
        assert_eq!(tadd(1, -1), 0);
        assert_eq!(tadd(0, 0), 0);
    }

    #[test]
    fn test_tsub() {
        assert_eq!(tsub(1, 1), 0);
        assert_eq!(tsub(0, -1), 1); // 0 - (-1) = 1
        assert_eq!(tsub(-1, 1), 1); // -1 - 1 = -2 ≡ 1 mod 3
    }

    #[test]
    fn test_tmul() {
        assert_eq!(tmul(1, 1), 1);
        assert_eq!(tmul(-1, -1), 1);
        assert_eq!(tmul(1, -1), -1);
        assert_eq!(tmul(0, 1), 0);
    }

    #[test]
    fn test_tneg() { assert_eq!(tneg(1), -1); assert_eq!(tneg(-1), 1); assert_eq!(tneg(0), 0); }

    #[test]
    fn test_tinv() { assert_eq!(tinv(1), Some(1)); assert_eq!(tinv(-1), Some(-1)); assert_eq!(tinv(0), None); }

    #[test]
    fn test_tdist() { assert_eq!(tdist(0, 1), 1); assert_eq!(tdist(1, -1), 1); assert_eq!(tdist(0, 0), 0); }

    #[test]
    fn test_tdot() {
        assert_eq!(tdot(&[1, 0, -1], &[1, 1, 0]), 1);
    }

    #[test]
    fn test_grid_new() {
        let g = TernaryGrid::new(3, 3);
        assert_eq!(g.get(1, 1), 0);
        assert_eq!(g.sum(), 0);
    }

    #[test]
    fn test_grid_set_get() {
        let mut g = TernaryGrid::new(3, 3);
        g.set(1, 1, 1);
        assert_eq!(g.get(1, 1), 1);
    }

    #[test]
    fn test_grid_histogram() {
        let mut g = TernaryGrid::new(3, 1);
        g.set(0, 0, -1); g.set(1, 0, 0); g.set(2, 0, 1);
        assert_eq!(g.histogram(), (1, 1, 1));
    }

    #[test]
    fn test_grid_map() {
        let mut g = TernaryGrid::new(2, 1);
        g.set(0, 0, 1); g.set(1, 0, -1);
        let mapped = g.map(tneg);
        assert_eq!(mapped.get(0, 0), -1);
        assert_eq!(mapped.get(1, 0), 1);
    }

    #[test]
    fn test_grid_fill() {
        let mut g = TernaryGrid::new(3, 3);
        g.fill(1);
        assert_eq!(g.sum(), 9);
    }

    #[test]
    fn test_graph_new() {
        let g = TernaryGraph::new(3);
        assert_eq!(g.degree(0), 0);
    }

    #[test]
    fn test_graph_add_edge() {
        let mut g = TernaryGraph::new(3);
        g.add_edge(0, 1, 1);
        assert_eq!(g.edge(0, 1), 1);
        assert_eq!(g.neighbors(0), vec![1]);
    }

    #[test]
    fn test_graph_connected() {
        let mut g = TernaryGraph::new(3);
        g.add_undirected(0, 1, 1);
        g.add_undirected(1, 2, 1);
        assert!(g.is_connected());
    }

    #[test]
    fn test_graph_disconnected() {
        let mut g = TernaryGraph::new(4);
        g.add_undirected(0, 1, 1);
        g.add_undirected(2, 3, 1);
        assert!(!g.is_connected());
        assert_eq!(g.components().len(), 2);
    }

    #[test]
    fn test_graph_negative_excluded() {
        let mut g = TernaryGraph::new(3);
        g.add_undirected(0, 1, -1);
        g.add_undirected(1, 2, 1);
        assert!(!g.is_connected()); // negative edge doesn't connect
    }

    #[test]
    fn test_ternary_value_i8() {
        assert_eq!(i8::to_ternary(5), 1);
        assert_eq!(i8::to_ternary(-3), -1);
    }

    #[test]
    fn test_ternary_value_bool() {
        assert_eq!(bool::to_ternary(true), 1);
        assert_eq!(bool::to_ternary(false), 0);
    }
}
