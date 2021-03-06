//! Utility for drawing lines on a 2d grid.
// https://www.redblobgames.com/grids/line-drawing.html
use glam::{BVec2, IVec2, Vec2};

use crate::GridPoint;

use super::{GridShape, ShapeIterator};

/// A simple grid line.
pub struct GridLine {
    start: IVec2,
    end: IVec2,
}

impl GridLine {
    pub fn new(start: impl GridPoint, end: impl GridPoint) -> Self {
        GridLine {
            start: start.as_ivec2(),
            end: end.as_ivec2(),
        }
    }
}

impl GridShape for GridLine {
    fn iter(&self) -> ShapeIterator {
        ShapeIterator::Line(self.start, LineIter::new(self.end - self.start))
    }
}

pub struct LineIter {
    dist: i32,
    step: i32,
    start: IVec2,
    end: IVec2,
}

impl LineIter {
    pub fn new(end: IVec2) -> Self {
        let start = IVec2::ZERO;
        LineIter {
            start,
            end,
            step: 0,
            dist: diag_distance(start, end),
        }
    }
}

impl Iterator for LineIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step > self.dist {
            return None;
        }

        let t = self.step as f32 / self.dist as f32;
        self.step += 1;

        Some(lerp_pos(self.start, self.end, t))
    }
}

#[inline]
fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (t * (end - start))
}

#[inline]
fn lerp_pos(p1: IVec2, p2: IVec2, t: f32) -> IVec2 {
    let p1 = p1.as_vec2();
    let p2 = p2.as_vec2();

    let x = lerp(p1.x, p2.x, t);
    let y = lerp(p1.y, p2.y, t);

    Vec2::new(x, y).round().as_ivec2()
}

#[inline]
fn diag_distance(p1: IVec2, p2: IVec2) -> i32 {
    let d = p2 - p1;

    i32::max(d.x.abs(), d.y.abs())
}

/// An grid line with only orthogonal movement.
pub struct GridLineOrthogonal {
    start: IVec2,
    end: IVec2,
}

impl GridLineOrthogonal {
    pub fn new(start: impl GridPoint, end: impl GridPoint) -> Self {
        Self {
            start: start.as_ivec2(),
            end: end.as_ivec2(),
        }
    }
}

impl GridShape for GridLineOrthogonal {
    fn iter(&self) -> ShapeIterator {
        ShapeIterator::LineOrtho(self.start, LineOrthogonalIter::new(self.end))
    }
}

pub struct LineOrthogonalIter {
    nxy: Vec2,
    i: Vec2,
    sign: Vec2,
    curr: Vec2,
    start: IVec2,
    yielded_start: bool,
}

impl LineOrthogonalIter {
    pub fn new(end: IVec2) -> LineOrthogonalIter {
        let start = Vec2::ZERO;
        let dxy = end.as_vec2();
        let nxy = dxy.abs();
        let sign = dxy.signum();

        LineOrthogonalIter {
            i: Vec2::ZERO,
            nxy,
            sign,
            start: start.as_ivec2(),
            curr: start,
            yielded_start: false,
        }
    }
}

impl Iterator for LineOrthogonalIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i.cmpge(self.nxy).any() {
            return None;
        }
        if !self.yielded_start {
            self.yielded_start = true;
            return Some(self.start);
        }

        let cmp = (self.i + 0.5) / self.nxy;
        let cmp = if cmp.x < cmp.y {
            BVec2::new(true, false)
        } else {
            BVec2::new(false, true)
        };

        let cd = Vec2::select(cmp, self.sign, Vec2::ZERO);
        let id = Vec2::select(cmp, Vec2::ONE, Vec2::ZERO);

        self.curr += cd;
        self.i += id;

        Some(self.curr.as_ivec2())
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Canvas;

    use super::*;

    #[test]
    fn line() {
        let line = GridLine::new([9, 4], [0, 0]);
        let mut canvas = Canvas::new([10, 5]);
        for p in line.iter() {
            canvas.put(p, '*');
        }
        canvas.print();
    }

    #[test]
    fn line_orthogonal() {
        let line = GridLineOrthogonal::new([9, 4], [0, 0]);
        let mut canvas = Canvas::new([10, 5]);
        for p in line.iter() {
            canvas.put(p, '*');
        }
        canvas.print();
    }
}
