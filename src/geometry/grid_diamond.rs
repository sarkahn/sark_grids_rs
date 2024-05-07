use glam::IVec2;

use crate::{
    direction::{DOWN_LEFT, DOWN_RIGHT, UP, UP_LEFT, UP_RIGHT},
    GridPoint,
};

/// A diamond of points on a 2d grid.
#[derive(Default, Clone, Debug, Copy, PartialEq, Eq)]
pub struct GridDiamond {
    pub pos: IVec2,
    pub radius: usize,
}

impl GridDiamond {
    pub fn new(pos: impl GridPoint, size: usize) -> Self {
        Self {
            pos: pos.as_ivec2(),
            radius: size,
        }
    }

    pub fn origin(size: usize) -> Self {
        Self::new([0, 0], size)
    }
}

const DIRECTIONS: &[IVec2; 4] = &[DOWN_RIGHT, DOWN_LEFT, UP_LEFT, UP_RIGHT];

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GridDiamondIter {
    origin: IVec2,
    p: IVec2,
    layer: usize,
    i: i32,
    dir_index: i32,
    size: usize,
}

impl GridDiamondIter {
    pub fn new(pos: impl GridPoint, size: usize) -> Self {
        Self {
            origin: pos.as_ivec2(),
            p: pos.as_ivec2(),
            layer: 0,
            i: -1,
            dir_index: -1,
            size,
        }
    }
}

impl Iterator for GridDiamondIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        if self.i >= 4 * self.layer as i32 {
            self.layer += 1;
            if self.layer > self.size {
                return None;
            }
            self.i = -1;
            self.dir_index = -1;
            self.p = self.origin + UP * self.layer as i32;
            return Some(self.p);
        }
        if self.i % self.layer as i32 == 0 {
            self.dir_index += 1;
        }
        let dir = DIRECTIONS[self.dir_index as usize];
        self.p += dir;
        Some(self.p)
    }
}

impl IntoIterator for GridDiamond {
    type Item = IVec2;

    type IntoIter = GridDiamondIter;

    fn into_iter(self) -> Self::IntoIter {
        GridDiamondIter::new(self.pos, self.radius)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Canvas;

    use super::GridDiamond;

    #[test]
    #[ignore]
    fn draw() {
        for size in 0..10 {
            let diamond = GridDiamond::origin(size);
            let origin = diamond.pos;
            let mut canvas = Canvas::new([size * 2 + 1, size * 2 + 1]);
            for p in diamond {
                canvas.put(p, '*');
            }
            canvas.put(origin, 'o');

            canvas.print();
        }
    }
}
