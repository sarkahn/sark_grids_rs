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
        self.get_index(i)
    }

    /// Retrieve the value of the bit at the given index.
    #[inline]
    pub fn get_index(&self, i: usize) -> bool {
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

    // /// Perform an operation on the bits that overlap with the given grid.
    // pub fn overlap_operation(&mut self, other: &BitGrid, operation: impl Fn(bool, bool) -> bool) {
    //     let inner = self.rect.clipped_to(other.rect);
    //     for y in inner.bottom()..=inner.top() {
    //         let a = self.transform_wtl([inner.left(), y]);
    //         let b = other.transform_wtl([inner.left(), y]);
    //         let ai = self.transform_lti(a);
    //         let bi = other.transform_lti(b);
    //         for x in 0..inner.width() {
    //             let a = self.bits.get(ai + x).unwrap();
    //             let b = other.bits.get(bi + x).unwrap();
    //             self.bits.set(ai + x, operation(a, b));
    //         }
    //     }
    // }

    // /// Perform an `or "a | b"` operation on any bits that overlap with the given grid.
    // pub fn overlap_or(&mut self, other: &BitGrid) {
    //     self.overlap_operation(other, |a, b| a | b);
    // }

    // /// Perform a `nor "!(a | b)"` operation on any bits that overlap with the given grid.
    // pub fn overlap_nor(&mut self, other: &BitGrid) {
    //     self.overlap_operation(other, |a, b| !(a | b));
    // }

    // /// Perform a `xor "a ^ b"` operation on any bits that overlap with the given grid.
    // pub fn overlap_xor(&mut self, other: &BitGrid) {
    //     self.overlap_operation(other, |a, b| a ^ b);
    // }

    // /// Perform an `xnor "a == b"` operation on any bits that overlap with the given grid.
    // pub fn overlap_xnor(&mut self, other: &BitGrid) {
    //     self.overlap_operation(other, |a, b| a == b);
    // }

    // /// Perform an `and "a & b"` operation on any bits that overlap with the given grid.
    // pub fn overlap_and(&mut self, other: &BitGrid) {
    //     self.overlap_operation(other, |a, b| a & b);
    // }

    // /// Perform a `nand "!(a & b)"` operation on any bits that overlap with the given grid.
    // pub fn overlap_nand(&mut self, other: &BitGrid) {
    //     self.overlap_operation(other, |a, b| !(a & b));
    // }

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
