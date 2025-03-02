//! A rectangular grid of float values with utility functions for performing operations
//! across the grid.

use std::ops::{Index, IndexMut};

use glam::{IVec2, UVec2};

use crate::{GridPoint, GridRect, GridSize, SizedGrid};

/// A rectangular grid of floating point values.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FloatGrid {
    data: Vec<f32>,
    size: UVec2,
}

impl SizedGrid for FloatGrid {
    fn size(&self) -> glam::UVec2 {
        self.size
    }
}

impl FloatGrid {
    pub fn new(size: impl GridSize) -> Self {
        Self {
            data: vec![0.0; size.tile_count()],
            size: size.to_uvec2(),
        }
    }

    /// Set the value for a position.
    pub fn set_value(&mut self, xy: impl GridPoint, value: f32) {
        *self.value_mut(xy) = value;
    }

    /// Retrieve the value at the given position. Will panic if the position is
    /// out of bounds.
    #[inline]
    pub fn value(&self, xy: impl GridPoint) -> f32 {
        let i = xy.as_index(self.size());
        self.data[i]
    }

    /// Attempt to the retrieve the value at a given position. Returns [None] if
    /// the position is out of bounds.
    #[inline]
    pub fn get_value(&self, xy: impl GridPoint) -> Option<f32> {
        let i = xy.get_index(self.size())?;
        Some(self.data[i])
    }

    #[inline]
    pub fn get_value_mut(&mut self, xy: impl GridPoint) -> Option<&mut f32> {
        let i = xy.get_index(self.size())?;
        Some(&mut self.data[i])
    }

    pub fn value_mut(&mut self, xy: impl GridPoint) -> &mut f32 {
        let i = self.transform_lti(xy);
        &mut self.data[i]
    }

    pub fn set_all(&mut self, value: f32) {
        self.data.fill(value);
    }

    pub fn values(&self) -> &[f32] {
        &self.data
    }
    pub fn values_mut(&mut self) -> &mut [f32] {
        &mut self.data
    }

    /// Apply a mathematical operation on all values in the grid.
    pub fn apply_operation(&mut self, operation: impl Fn(f32) -> f32) {
        for v in self.data.iter_mut() {
            *v = operation(*v);
        }
    }

    /// Iterate over a rectangular section of values.
    pub fn iter_rect(&self, rect: GridRect) -> impl DoubleEndedIterator<Item = &f32> {
        let iter = self
            .data
            .chunks(self.width())
            .skip(rect.bottom() as usize)
            .flat_map(move |tiles| tiles[rect.left() as usize..=rect.right() as usize].iter());

        iter
    }

    /// Iterate over a rectangular section of values.
    pub fn iter_rect_mut(&mut self, rect: GridRect) -> impl DoubleEndedIterator<Item = &mut f32> {
        let w = self.width();
        self.data
            .chunks_mut(w)
            .skip(rect.bottom() as usize)
            .flat_map(move |tiles| tiles[rect.left() as usize..=rect.right() as usize].iter_mut())
    }

    /// Reset all values in the [FloatGrid] to 0.
    pub fn clear(&mut self) {
        self.data.fill(0.0);
    }

    pub fn iter_xy(&self) -> impl Iterator<Item = (IVec2, f32)> + '_ {
        self.iter_grid_points()
            .enumerate()
            .map(move |(i, p)| (p, self[i]))
    }

    pub fn iter_xy_muy(&mut self) -> impl Iterator<Item = (IVec2, &mut f32)> + '_ {
        self.iter_grid_points().zip(self.data.iter_mut())
    }
}

impl<P: GridPoint> Index<P> for FloatGrid {
    type Output = f32;

    fn index(&self, p: P) -> &Self::Output {
        let i = self.transform_lti(p);
        &self.data[i]
    }
}

impl<P: GridPoint> IndexMut<P> for FloatGrid {
    fn index_mut(&mut self, index: P) -> &mut Self::Output {
        let xy = index.to_ivec2();
        let i = self.transform_lti(xy);
        &mut self.data[i]
    }
}

impl Index<usize> for FloatGrid {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
impl IndexMut<usize> for FloatGrid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
