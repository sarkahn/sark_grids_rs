//! Utility for handlines lines on a 2d grid.
use std::ops::Sub;

// https://www.redblobgames.com/grids/line-drawing.html
use glam::IVec2;

use crate::GridPoint;

use super::{GridRect, GridShape};

/// A line of points on a grid.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridLine {
    pub start: IVec2,
    pub end: IVec2,
}

impl GridLine {
    /// Create a new grid line. Note that `end` is included and will be
    /// the final point on the line.
    pub fn new(start: impl GridPoint, end: impl GridPoint) -> Self {
        GridLine {
            start: start.as_ivec2(),
            end: end.as_ivec2(),
        }
    }

    /// Create a line with the start point at 0,0
    pub fn origin(end: impl GridPoint) -> Self {
        Self::new([0, 0], end)
    }

    #[inline]
    pub fn length(&self) -> usize {
        self.end.as_vec2().sub(self.start.as_vec2()).length() as usize
    }
}

impl GridShape for GridLine {
    fn iter(&self) -> super::GridShapeIterator {
        super::GridShapeIterator::Line(self.into_iter())
    }

    fn pos(&self) -> IVec2 {
        self.start
    }

    fn set_pos(&mut self, pos: IVec2) {
        let v = self.end - self.start;
        self.start = pos;
        self.end = self.start + v;
    }

    fn bounds(&self) -> super::GridRect {
        let min = self.start.min(self.end);
        let max = self.start.max(self.end);
        GridRect::from_points(min, max)
    }
}

#[derive(Debug, Clone)]
pub struct GridLineIter {
    start: IVec2,
    dist: i32,
    step: i32,
    end: IVec2,
}

impl GridLineIter {
    pub fn new(start: impl GridPoint, end: impl GridPoint) -> Self {
        let start = start.as_ivec2();
        let end = end.as_ivec2();
        GridLineIter {
            start,
            end,
            step: 0,
            dist: diag_distance(start, end),
        }
    }
}

impl Iterator for GridLineIter {
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

impl IntoIterator for GridLine {
    type Item = IVec2;
    type IntoIter = GridLineIter;

    fn into_iter(self) -> Self::IntoIter {
        GridLineIter::new(self.start, self.end)
    }
}

#[inline]
fn lerp_pos(p1: IVec2, p2: IVec2, t: f32) -> IVec2 {
    let p1 = p1.as_vec2();
    let p2 = p2.as_vec2();

    p1.lerp(p2, t).round().as_ivec2()
}

#[inline]
fn diag_distance(p1: IVec2, p2: IVec2) -> i32 {
    let d = p2 - p1;

    i32::max(d.x.abs(), d.y.abs())
}

/// An orthogonal line of points on a grid.
///
/// Unlike [GridLine] every point on this line is orthogonal to the next so
/// there are no diagonal jumps between points.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridLineOrtho {
    start: IVec2,
    end: IVec2,
}

impl GridLineOrtho {
    /// Create a new orthogonal line. Note that `end` is included and will be
    /// the final point on the line.
    pub fn new(start: impl GridPoint, end: impl GridPoint) -> Self {
        Self {
            start: start.as_ivec2(),
            end: end.as_ivec2(),
        }
    }

    /// Create an orthogonal line with it's start point at 0,0
    pub fn origin(end: impl GridPoint) -> Self {
        Self::new([0, 0], end)
    }
}

impl GridShape for GridLineOrtho {
    fn iter(&self) -> super::GridShapeIterator {
        super::GridShapeIterator::LineOrtho(self.into_iter())
    }

    fn pos(&self) -> IVec2 {
        self.start
    }

    fn set_pos(&mut self, pos: IVec2) {
        let v = self.end - self.start;
        self.start = pos;
        self.end = self.start + v;
    }

    fn bounds(&self) -> super::GridRect {
        let min = self.start.min(self.end);
        let max = self.start.max(self.end);
        GridRect::from_points(min, max)
    }
}

#[derive(Debug, Clone)]
pub struct GridLineOrthoIter {
    n: IVec2,
    p: IVec2,
    i: IVec2,
    sign: IVec2,
}

impl GridLineOrthoIter {
    pub fn new(start: impl GridPoint, end: impl GridPoint) -> GridLineOrthoIter {
        let end = end.as_ivec2();
        let start = start.as_ivec2();
        let d = end - start;
        let n = d.abs();
        let sign = d.signum();
        // Offset so we can add direction on first step
        let first = next_dir(IVec2::ZERO, n, sign);

        GridLineOrthoIter {
            n,
            p: start - first,
            i: -first.abs(),
            sign,
        }
    }
}

#[inline]
fn next_dir(i: IVec2, n: IVec2, sign: IVec2) -> IVec2 {
    let i = i.as_vec2();
    let n = n.as_vec2();
    if (0.5 + i.x) / n.x < (0.5 + i.y) / n.y {
        // Horizontal
        IVec2::new(sign.x, 0)
    } else {
        // Vertical
        IVec2::new(0, sign.y)
    }
}

impl Iterator for GridLineOrthoIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        let dir = next_dir(self.i, self.n, self.sign);

        self.i += dir.abs();

        if self.i.cmpgt(self.n).any() {
            return None;
        }

        self.p += dir;

        Some(self.p)
    }
}

impl IntoIterator for GridLineOrtho {
    type Item = IVec2;
    type IntoIter = GridLineOrthoIter;

    fn into_iter(self) -> Self::IntoIter {
        GridLineOrthoIter::new(self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Canvas;

    use super::*;

    #[test]
    #[ignore]
    fn line_rect() {
        let mut canvas = Canvas::new([11, 11]);
        let lines = [
            GridLine::new([-4, -4], [-4, 4]),
            GridLine::new([-4, 4], [4, 4]),
            GridLine::new([4, 4], [4, -4]),
            GridLine::new([4, -4], [-4, -4]),
        ];
        for p in lines.iter().flat_map(|l| l.iter()) {
            canvas.put(p, '*');
        }
        canvas.print();
    }

    #[test]
    #[ignore]
    fn line_orthogonal() {
        let mut canvas = Canvas::new([11, 11]);
        let lines = [
            GridLineOrtho::new([0, 0], [4, 4]),
            GridLineOrtho::new([0, 0], [-4, 4]),
            GridLineOrtho::new([0, 0], [-4, -4]),
            GridLineOrtho::new([0, 0], [4, -4]),
        ];
        for p in lines.iter().flat_map(|l| l.iter()) {
            canvas.put(p, '*');
        }
        canvas.print();
    }
}
