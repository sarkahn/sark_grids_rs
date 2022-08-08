//! Utility for drawing rectangles on a 2d grid.
use glam::{IVec2, UVec2};

use crate::{GridPoint, Size2d};

use super::GridShape;

/// A filled rectangle.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridRect {
    pub pos: IVec2,
    pub size: UVec2,
}

impl GridRect {
    pub fn new(pos: impl GridPoint, size: impl Size2d) -> GridRect {
        GridRect {
            pos: pos.as_ivec2(),
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
        GridRect {
            pos: min,
            size: (max - min).as_uvec2(),
        }
    }

    pub fn from_center_size(center: impl GridPoint, size: impl Size2d) -> GridRect {
        let position = center.as_ivec2() - (size.as_ivec2() / 2);
        GridRect {
            pos: position,
            size: size.as_uvec2(),
        }
    }

    /// Returns a new rect with it's center moved to it's current position.
    pub fn centered(&self) -> GridRect {
        GridRect::from_center_size(self.pos, self.size)
    }

    pub fn move_center(&mut self, position: impl GridPoint) {
        self.pos = position.as_ivec2() - (self.size / 2).as_ivec2()
    }

    pub fn min(&self) -> IVec2 {
        self.pos
    }

    pub fn max(&self) -> IVec2 {
        self.pos + self.size.as_ivec2()
    }

    pub fn center(&self) -> IVec2 {
        self.pos + self.size.as_ivec2() / 2
    }
}

impl GridShape for GridRect {
    fn iter(&self) -> super::GridShapeIterator {
        super::GridShapeIterator::Rect(self.into_iter())
    }
}

#[derive(Debug, Clone)]
pub struct GridRectIter {
    pos: IVec2,
    curr: IVec2,
    size: IVec2,
}

impl GridRectIter {
    pub fn new(pos: impl GridPoint, size: impl Size2d) -> Self {
        GridRectIter {
            pos: pos.as_ivec2(),
            curr: IVec2::ZERO,
            size: size.as_ivec2(),
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
        Some(self.pos + p)
    }
}

impl IntoIterator for GridRect {
    type Item = IVec2;
    type IntoIter = GridRectIter;

    fn into_iter(self) -> Self::IntoIter {
        GridRectIter::new(self.pos, self.size)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Canvas;

    use super::GridRect;

    #[test]
    fn iter() {
        let rect = GridRect::new([1, 1], [3, 3]);
        let mut canvas = Canvas::new([6, 6]);
        for p in rect {
            canvas.put(p, '*');
        }
        canvas.print();
    }
}
