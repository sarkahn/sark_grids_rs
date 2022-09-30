//! Utility for drawing rectangles on a 2d grid.
use glam::{IVec2, UVec2};

use crate::{GridPoint, Size2d};

use super::GridShape;

/// A rectangle of points on a grid.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridRect {
    pub center: IVec2,
    pub size: UVec2,
}

impl GridRect {
    pub fn new(pos: impl GridPoint, size: impl Size2d) -> GridRect {
        GridRect {
            center: pos.as_ivec2(),
            size: size.as_uvec2(),
        }
    }

    /// Create a grid rect with it's position set to 0,0
    pub fn origin(size: impl Size2d) -> Self {
        Self::new([0, 0], size)
    }

    pub fn from_min_max(min: impl GridPoint, max: impl GridPoint) -> GridRect {
        let min = min.as_ivec2();
        let max = max.as_ivec2();
        let size = max - min;
        GridRect {
            center: min + size / 2,
            size: size.as_uvec2(),
        }
    }

    pub fn min(&self) -> IVec2 {
        self.center - self.size.as_ivec2() / 2
    }

    pub fn max(&self) -> IVec2 {
        self.center + self.size.as_ivec2() / 2
    }

    pub fn width(&self) -> usize {
        self.size.x as usize
    }

    pub fn height(&self) -> usize {
        self.size.y as usize
    }

    pub fn corner(&self, corner: GridCorner) -> IVec2 {
        let [w, h] = (self.size.as_ivec2() / 2).to_array();
        self.center
            + IVec2::from(match corner {
                GridCorner::TopLeft => [-w, h],
                GridCorner::TopRight => [w, h],
                GridCorner::BottomLeft => [-w, -h],
                GridCorner::BottomRight => [w, -h],
            })
    }

    /// Return a rect with the same center but resized by the given amount
    pub fn resized(&self, amount: impl GridPoint) -> GridRect {
        let size = (self.size.as_ivec2() + amount.as_ivec2())
            .max(IVec2::ONE)
            .as_uvec2();
        GridRect::new(self.center, size)
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
    fn iter() {
        let rect = GridRect::new([3, 3], [3, 3]);
        let mut canvas = Canvas::new([6, 6]);
        for p in rect {
            canvas.put(p, '*');
        }
        canvas.print();
    }
}
