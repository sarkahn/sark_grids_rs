use std::ops::{Index, IndexMut};

use glam::IVec2;

use crate::{GridPoint, GridRect, GridSize, PositionedGrid, SizedGrid};

/// A rectangular grid of floating point values.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FloatGrid {
    data: Vec<f32>,
    rect: GridRect,
}

impl SizedGrid for FloatGrid {
    fn size(&self) -> glam::UVec2 {
        self.rect.size
    }
}

impl PositionedGrid for FloatGrid {
    fn pos(&self) -> IVec2 {
        self.rect.pos()
    }
}

impl FloatGrid {
    pub fn new(position: impl GridPoint, size: impl GridSize) -> Self {
        Self {
            data: vec![0.0; size.tile_count()],
            rect: GridRect::new(position, size),
        }
    }

    pub fn new_origin(size: impl GridSize) -> Self {
        Self::new(IVec2::ZERO, size)
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

    /// Perform an operation on the values that overlap with the given grid.
    pub fn overlap_apply_operation(
        &mut self,
        other: &FloatGrid,
        operation: impl Fn(f32, f32) -> f32,
    ) {
        let inner = self.rect.clipped(other.rect);
        for y in inner.bottom()..=inner.top() {
            let a = self.transform_wtl([inner.left(), y]);
            let b = other.transform_wtl([inner.left(), y]);
            let ai = self.transform_lti(a);
            let bi = other.transform_lti(b);
            for x in 0..inner.width() {
                let a = &mut self.data[ai + x];
                let b = other.data[bi + x];
                *a = operation(*a, b);
            }
        }
    }

    pub fn bounds(&self) -> GridRect {
        self.rect
    }

    /// Reset all values in the [FloatGrid] to 0.
    pub fn clear(&mut self) {
        self.data.fill(0.0);
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

#[cfg(test)]
mod tests {
    use super::FloatGrid;

    #[test]
    fn overlap() {
        let mut a = FloatGrid::new_origin([20, 20]);
        let mut b = FloatGrid::new([10, 10], [10, 10]);
        a.set_all(1.0);
        b.set_all(10.0);
        a.overlap_apply_operation(&b, |a, b| a + b);
        assert_eq!(1.0, a.value([9, 9]));
        assert_eq!(11.0, a.value([10, 10]));
    }
}
