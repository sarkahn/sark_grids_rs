use std::collections::BTreeMap;

use glam::{IVec2, UVec2};

use crate::{GridPoint, SizedGrid};

/// A simple utility for storing data in a sparse grid, backed by
/// a BTreeMap
pub struct SparseGrid<T> {
    data: BTreeMap<Cell, T>,
    size: UVec2,
}

impl<T> SparseGrid<T> {
    pub fn new(size: UVec2) -> Self {
        Self {
            data: BTreeMap::new(),
            size,
        }
    }

    pub fn insert(&mut self, pos: impl GridPoint, value: T) {
        self.data.insert(Cell(pos.to_ivec2()), value);
    }

    pub fn get(&self, pos: impl GridPoint) -> Option<&T> {
        self.data.get(&Cell(pos.to_ivec2()))
    }

    pub fn get_mut(&mut self, pos: impl GridPoint) -> Option<&mut T> {
        self.data.get_mut(&Cell(pos.to_ivec2()))
    }

    pub fn remove(&mut self, pos: impl GridPoint) -> Option<T> {
        self.data.remove(&Cell(pos.to_ivec2()))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&IVec2, &T)> {
        self.data.iter().map(|(cell, value)| (&cell.0, value))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&IVec2, &mut T)> {
        self.data.iter_mut().map(|(cell, value)| (&cell.0, value))
    }
}

impl<T> SizedGrid for SparseGrid<T> {
    fn size(&self) -> UVec2 {
        self.size
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Cell(IVec2);

impl Ord for Cell {
    // Order by y then x
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.y.cmp(&other.0.y).then(self.0.x.cmp(&other.0.x))
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
