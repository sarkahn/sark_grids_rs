//! A rectangular grid of bit values for representing simple state across a large grid.

use bit_vec::BitVec;
use glam::UVec2;

use crate::{GridPoint, GridSize, SizedGrid};

#[derive(Default, Clone)]
/// A rectangular grid with it's underlying data defined as a [BitVec].
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
    /// Create a new BitGrid
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

    /// Retrieve the value of a bit.
    #[inline]
    pub fn get(&self, xy: impl GridPoint) -> bool {
        let i = self.transform_lti(xy);
        self.value_from_index(i)
    }

    /// Retrieve the value of the bit at the given index.
    #[inline]
    pub fn value_from_index(&self, i: usize) -> bool {
        self.bits.get(i).unwrap()
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

#[cfg(test)]
mod tests {

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
