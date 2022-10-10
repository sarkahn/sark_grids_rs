//! Utility for handling rectangles on a 2d grid.
use std::ops::{Add, Sub};

use glam::{IVec2, Vec2};

use crate::{GridPoint, Size2d};

use super::GridShape;

/// A rectangle of points on a grid.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct GridRect {
    pub center: IVec2,
    size: IVec2,
    // Stored for the common case of testing overlaps and boundaries
    extents: Vec2,
}

impl GridRect {
    pub fn new(pos: impl GridPoint, size: impl Size2d) -> GridRect {
        GridRect {
            center: pos.as_ivec2(),
            size: size.as_ivec2(),
            extents: size.as_vec2() / 2.0,
        }
    }

    /// Create a grid rect with it's center set to 0,0
    pub fn origin(size: impl Size2d) -> Self {
        Self::new([0, 0], size)
    }

    /// Create a grid rect from a min and max position. The given points
    /// will make up the bottom left and top right most points on the rect.
    pub fn from_min_max(min: impl GridPoint, max: impl GridPoint) -> GridRect {
        debug_assert!(min.as_ivec2().cmplt(max.as_ivec2()).all());

        let min = min.as_ivec2();
        let max = max.as_ivec2();
        let size = (max - min) + 1;
        GridRect {
            center: min + size / 2,
            size,
            extents: size.as_vec2() / 2.0,
        }
    }

    /// Create a rect with the bottom left corner at the given position.
    pub fn from_bl(pos: impl GridPoint, size: impl Size2d) -> GridRect {
        GridRect::from_min_max(pos, pos.as_ivec2() + (size.as_ivec2() - 1))
    }

    /// Retrieve the bottom-left-most point of the rect
    pub fn min(&self) -> Vec2 {
        self.center.as_vec2() - self.extents
    }

    /// Retrieve the top-right-most point of the rect
    pub fn max(&self) -> Vec2 {
        self.center.as_vec2() + self.extents
    }

    /// Retrieve the bottom-left-most point of the rect as a grid position
    pub fn min_i(&self) -> IVec2 {
        self.min().add(0.5).as_ivec2()
    }

    /// Retrieve the top-right-most point of the rect as a grid position
    pub fn max_i(&self) -> IVec2 {
        self.min_i() + self.size.sub(1)
    }

    pub fn width(&self) -> usize {
        self.size.x as usize
    }

    pub fn height(&self) -> usize {
        self.size.y as usize
    }

    /// Retrieve the grid position of a given corner of the rect
    pub fn corner(&self, corner: GridCorner) -> IVec2 {
        let [w, h] = (self.size / 2).to_array();
        self.center
            + IVec2::from(match corner {
                GridCorner::TopLeft => [-w, h],
                GridCorner::TopRight => [w, h],
                GridCorner::BottomLeft => [-w, -h],
                GridCorner::BottomRight => [w, -h],
            })
    }

    /// Return a rect with the same center but resized by the given amount
    /// on each axis
    pub fn resized(&self, amount: impl GridPoint) -> GridRect {
        let size = (self.size + amount.as_ivec2()).max(IVec2::ONE).as_uvec2();
        GridRect::new(self.center, size)
    }

    /// Check if a given point lies inside the rect
    #[inline]
    pub fn contains(&self, p: impl GridPoint) -> bool {
        let p = p.as_ivec2();
        !(p.cmplt(self.min_i()).any() || p.cmpgt(self.max_i()).any())
    }

    /// Check if any part of a rect overlaps another
    #[inline]
    pub fn overlaps(&self, other: GridRect) -> bool {
        let ac = self.center.as_vec2();
        let bc = other.center.as_vec2();
        let ar = self.extents;
        let br = other.extents;

        ac.sub(bc).abs().cmple(ar.add(br)).all()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GridCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl GridShape for GridRect {
    fn iter(&self) -> super::GridShapeIterator {
        super::GridShapeIterator::Rect(self.into_iter())
    }

    fn pos(&self) -> IVec2 {
        self.center
    }

    fn set_pos(&mut self, pos: IVec2) {
        self.center = pos;
    }
}

#[derive(Debug, Clone)]
pub struct GridRectIter {
    origin: IVec2,
    curr: IVec2,
    size: IVec2,
}

impl GridRectIter {
    pub fn new(center: impl GridPoint, size: impl Size2d) -> Self {
        let size = size.as_ivec2();
        GridRectIter {
            origin: center.as_ivec2() - size / 2,
            curr: IVec2::ZERO,
            size,
        }
    }
}

impl Iterator for GridRectIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.cmpge(self.size).any() {
            return None;
        }

        let p = self.curr;
        self.curr.x += 1;
        if self.curr.x == self.size.x {
            self.curr.x = 0;
            self.curr.y += 1;
        }
        Some(self.origin + p)
    }
}

impl IntoIterator for GridRect {
    type Item = IVec2;
    type IntoIter = GridRectIter;

    fn into_iter(self) -> Self::IntoIter {
        GridRectIter::new(self.center, self.size)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Canvas;

    use super::GridRect;

    #[test]
    fn rect_min_max() {
        let rect = GridRect::new([0, 0], [3, 3]);
        assert_eq!([-1, -1], rect.min_i().to_array());
        assert_eq!([1, 1], rect.max_i().to_array());

        let rect = GridRect::new([0, 0], [4, 4]);
        assert_eq!([-1, -1], rect.min_i().to_array());
        assert_eq!([2, 2], rect.max_i().to_array());

        let rect = GridRect::new([-5, -5], [3, 3]);
        assert_eq!([-6, -6], rect.min_i().to_array());
        assert_eq!([-4, -4], rect.max_i().to_array());

        let rect = GridRect::new([-5, -5], [4, 4]);
        assert_eq!([-6, -6], rect.min_i().to_array());
        assert_eq!([-3, -3], rect.max_i().to_array());
    }

    #[test]
    fn contains_point() {
        let rect = GridRect::origin([5, 5]);
        assert!(rect.contains([-2, -2]));
        assert!(rect.contains([2, 2]));
        assert!(!rect.contains([3, 3]));
        assert!(!rect.contains([-3, -3]));
    }

    #[test]
    fn from_bl() {
        let rect = GridRect::origin([5, 5]);
        let rect2 = GridRect::from_bl([-2, -2], [5, 5]);

        assert_eq!(rect, rect2);
    }

    #[test]
    fn overlap() {
        let a = GridRect::new([-1, -1], [3, 3]);
        let b = GridRect::new([1, 1], [3, 3]);
        let c = GridRect::new([3, 3], [3, 3]);
        assert!(a.overlaps(b));
        assert!(b.overlaps(a));
        assert!(b.overlaps(c));
        assert!(c.overlaps(b));
        assert!(!a.overlaps(c));
        assert!(!c.overlaps(a));

        assert!(a.overlaps(a));
        assert!(b.overlaps(b));
        assert!(c.overlaps(c));

        let a = GridRect::new([-2, -2], [4, 4]);
        let b = GridRect::new([1, 1], [4, 4]);
        let c = GridRect::new([4, 4], [4, 4]);
        assert!(a.overlaps(b));
        assert!(b.overlaps(a));
        assert!(b.overlaps(c));
        assert!(c.overlaps(b));
        assert!(!a.overlaps(c));
        assert!(!c.overlaps(a));

        assert!(a.overlaps(a));
        assert!(b.overlaps(b));
        assert!(c.overlaps(c));
    }

    #[test]
    #[ignore]
    fn iter() {
        let rect = GridRect::new([3, 3], [3, 3]);
        let mut canvas = Canvas::new([6, 6]);
        for p in rect {
            canvas.put(p, '*');
        }
        canvas.print();
    }

    #[test]
    #[ignore]
    fn big() {
        let rect = GridRect::new([16, 16], [30, 30]);
        let mut canvas = Canvas::new([32, 32]);

        for p in rect {
            canvas.put(p, '*');
        }
        canvas.print();
    }
}
