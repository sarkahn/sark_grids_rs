//! Utility for drawing rectangles on a 2d grid.
use glam::{IVec2, UVec2};

use crate::{GridPoint, Size2d};

use super::{GridShape, ShapeIter};

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
    type Iterator = ShapeIter;

    fn iter(&self) -> Self::Iterator {
        ShapeIter::Rect(GridRectIter::new(self))
    }
}

pub struct GridRectIter {
    curr: IVec2,
    start: IVec2,
    end: IVec2,
}

impl GridRectIter {
    pub fn new(rect: &GridRect) -> Self {
        GridRectIter {
            start: rect.position,
            curr: rect.position,
            end: rect.position + rect.size.as_ivec2(),
        }
    }
}

impl Iterator for GridRectIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.cmpge(self.end).any() {
            return None;
        }

        let p = self.curr;
        self.curr.x += 1;
        if self.curr.x == self.end.x {
            self.curr.x = self.start.x;
            self.curr.y += 1;
        }
        Some(p)
    }
}

pub struct GridBorderIter {
    curr: IVec2,
    count: i32,
    max: i32,
}

impl Iterator for GridBorderIter {
    type Item=IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.max {
            return None;
        };
        let curr = self.curr;
        self.count += 1;

        Some(curr)
    }
}

#[cfg(test)]
mod tests {
    use crate::{geometry::GridShape, util::Canvas};

    use super::GridRect;

    #[test]
    fn iter() {
        let rect = GridRect::new([1, 1], [3, 3]);
        let mut canvas = Canvas::new([5, 5]);
        for p in rect.iter() {
            canvas.put(p, '*');
        }
        canvas.print();
    }
}
