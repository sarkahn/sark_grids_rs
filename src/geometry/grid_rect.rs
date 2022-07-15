//! Utility for drawing rectangles on a 2d grid.
use glam::{IVec2, UVec2};

use crate::{GridPoint, Size2d};

use super::{GridShape, ShapeIterator};

/// A filled rectangle.
pub struct GridRect {
    pub position: IVec2,
    pub size: UVec2,
}

impl GridRect {
    pub fn new(pos: impl GridPoint, size: impl Size2d) -> GridRect {
        GridRect {
            position: pos.as_ivec2(),
            size: size.as_uvec2(),
        }
    }

    pub fn from_min_max(min: impl GridPoint, max: impl GridPoint) -> GridRect {
        let min = min.as_ivec2();
        let max = max.as_ivec2();
        GridRect {
            position: min,
            size: (max - min).as_uvec2(),
        }
    }

    pub fn from_center_size(center: impl GridPoint, size: impl Size2d) -> GridRect {
        let position = center.as_ivec2() - (size.as_ivec2() / 2);
        GridRect {
            position,
            size: size.as_uvec2(),
        }
    }

    pub fn move_center(&mut self, position: impl GridPoint) {
        self.position = position.as_ivec2() - (self.size / 2).as_ivec2()
    }

    pub fn min(&self) -> IVec2 {
        self.position
    }

    pub fn max(&self) -> IVec2 {
        self.position + self.size.as_ivec2()
    }

    pub fn center(&self) -> IVec2 {
        self.position + self.size.as_ivec2() / 2
    }
}

impl GridShape for GridRect {
    fn iter(&self) -> ShapeIterator {
        ShapeIterator::Rect(self.position, GridRectIter::new(self.size))
    }
}

pub struct GridRectIter {
    curr: IVec2,
    size: IVec2,
}

impl GridRectIter {
    pub fn new(size: UVec2) -> Self {
        GridRectIter {
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
        Some(p)
    }
}

#[cfg(test)]
mod tests {
    use crate::{geometry::GridShape, util::Canvas};

    use super::GridRect;

    #[test]
    fn iter() {
        let rect = GridRect::new([1, 1], [3, 3]);
        let mut canvas = Canvas::new([6, 6]);
        for p in rect.iter() {
            canvas.put(p, '*');
        }
        canvas.print();
    }
}
