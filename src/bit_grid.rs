//! A rectangular grid of bit values for representing simple state across a large grid.

use bit_vec::BitVec;
use glam::{IVec2, UVec2};

use crate::{GridPoint, GridRect, GridSize, SizedGrid};

/// A rectangular grid with it's underlying data defined as a [BitVec].
#[derive(Default, Clone)]
pub struct BitGrid {
    bits: BitVec,
    size: UVec2,
}

impl SizedGrid for BitGrid {
    fn size(&self) -> UVec2 {
        self.size
    }
}

impl BitGrid {
    /// Create a new BitGrid with all bits set to false.
    pub fn new(size: impl GridSize) -> Self {
        Self {
            bits: BitVec::from_elem(size.tile_count(), false),
            size: size.to_uvec2(),
        }
    }

    /// Set the initial value for all bits.
    pub fn with_value(mut self, value: bool) -> Self {
        self.set_all(value);
        self
    }

    /// Retrieve the value of a bit at the given 2d index.
    #[inline]
    pub fn get(&self, xy: impl GridPoint) -> bool {
        let i = self.transform_lti(xy);
        self.get_index(i)
    }

    /// Retrieve the value of the bit at the given index.
    #[inline]
    pub fn get_index(&self, i: usize) -> bool {
        self.bits.get(i).unwrap()
    }

    #[inline]
    pub fn set_true(&mut self, xy: impl GridPoint) {
        self.set(xy, true);
    }

    #[inline]
    pub fn set_false(&mut self, xy: impl GridPoint) {
        self.set(xy, false);
    }

    /// Set the bit at the given 2d index.
    #[inline]
    pub fn set(&mut self, xy: impl GridPoint, value: bool) {
        let i = self.transform_lti(xy);
        self.bits.set(i, value);
    }

    /// Set the bit at the given 1d index.
    #[inline]
    pub fn set_index(&mut self, i: usize, value: bool) {
        self.bits.set(i, value);
    }

    /// Set the bit at the given 2d index to true.
    #[inline]
    pub fn set_index_true(&mut self, i: usize) {
        self.set_index(i, true);
    }

    /// Set the bit at the given 2d index to false.
    #[inline]
    pub fn set_index_false(&mut self, i: usize) {
        self.set_index(i, false);
    }

    /// Toggle the value of the given bit.
    #[inline]
    pub fn toggle(&mut self, xy: impl GridPoint) {
        let i = self.transform_lti(xy);
        self.toggle_index(i)
    }

    /// Toggle the value of the bit at the given index.
    #[inline]
    pub fn toggle_index(&mut self, i: usize) {
        let v = self.bits.get(i).unwrap();
        self.bits.set(i, !v);
    }

    /// Set the value for all bits.
    pub fn set_all(&mut self, value: bool) {
        match value {
            true => self.bits.set_all(),
            false => self.bits.clear(),
        }
    }

    /// A reference to the underlying bit data.
    pub fn bits(&self) -> &BitVec {
        &self.bits
    }

    /// A mutable reference to the underlying bit data.
    pub fn bits_mut(&mut self) -> &mut BitVec {
        &mut self.bits
    }

    /// Returns true if any bits in the grid are set.
    pub fn any(&self) -> bool {
        self.bits.any()
    }

    /// Returns true if none of the bits in the grid are set.
    pub fn none(&self) -> bool {
        self.bits.none()
    }

    /// Negate the bits in the grid.
    pub fn all_negate(&mut self) {
        self.bits.negate();
    }

    /// Unset all bits in the grid.
    pub fn clear(&mut self) {
        self.bits.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = bool> + '_ {
        self.bits.iter()
    }

    pub fn iter_xy(&self) -> impl Iterator<Item = (IVec2, bool)> + '_ {
        self.iter_grid_points().map(move |p| (p, self.get(p)))
    }

    /// Create a new BitGrid from a rectangular area within this grid.
    pub fn clone_rect(&self, area: GridRect) -> BitGrid {
        let mut grid = BitGrid::new(area.size());
        for p in area.iter_points() {
            grid.set(p, self.get(p));
        }
        grid
    }
}

impl std::fmt::Debug for BitGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = format!("{:?}", self.bits);
        for y in (0..self.height()).rev() {
            let start = self.transform_lti([0, y]);
            let end = start + self.width();
            writeln!(f, "{}", &a[start..end])?;
        }
        Ok(())
    }
}

impl<T: GridPoint> std::ops::Index<T> for BitGrid {
    type Output = bool;

    fn index(&self, index: T) -> &Self::Output {
        &self.bits[self.transform_lti(index)]
    }
}

impl std::ops::Index<usize> for BitGrid {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bits[index]
    }
}

#[cfg(test)]
mod tests {
    use super::BitGrid;
    use crate::SizedGrid;

    #[test]
    fn iter() {
        let mut grid = BitGrid::new([10, 5]);

        grid.set_true([0, 0]);
        grid.set_true([0, 1]);
        grid.set_true([9, 2]);
        grid.set_true([9, 4]);

        let points: Vec<_> = grid.iter_xy().collect();
        assert!(points[grid.transform_lti([0, 0])].1);
        assert!(points[grid.transform_lti([0, 1])].1);
        assert!(points[grid.transform_lti([9, 2])].1);
        assert!(points[grid.transform_lti([9, 4])].1);
    }

    // #[test]
    // fn overlap() {
    //     let mut grid = BitGrid::new([0, 0], [5, 6]);
    //     let other = BitGrid::new([2, 3], [3, 3]).with_value(true);
    //     grid.overlap_xor(&other);
    //     for p in other.rect.iter_rect_points() {
    //         assert!(grid.get(p));
    //     }

    //     grid.clear();
    //     grid.set([3, 4], true);
    //     grid.overlap_and(&other);

    //     assert!(!grid.get([2, 3]));
    //     assert!(grid.get([3, 4]));
    // }
}
