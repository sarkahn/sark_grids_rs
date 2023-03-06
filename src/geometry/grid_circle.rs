//! Utility for handling circular shapes on a 2d grid.
// https://www.redblobgames.com/grids/circle-drawing/
use glam::{IVec2, Vec2};

use crate::GridPoint;

use super::{grid_rect::GridRectIter, GridRect, GridShape};

/// A filled circle. Points within the circle can be iterator over.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridCircle {
    pub center: IVec2,
    pub radius: usize,
}

impl GridCircle {
    pub fn new(center: impl GridPoint, radius: usize) -> Self {
        GridCircle {
            center: center.as_ivec2(),
            radius,
        }
    }

    /// Create a circle centered at 0,0
    pub fn origin(radius: usize) -> Self {
        Self::new([0, 0], radius)
    }

    /// Create an outlined circle with this circle's position and size.
    pub fn outline(&self) -> GridCircleOutline {
        GridCircleOutline::new(self.center, self.radius)
    }

    #[inline]
    pub fn overlaps(&self, other: GridCircle) -> bool {
        let a = (self.radius + other.radius) as i32;
        let d = self.center - other.center;
        a * a > (d.x * d.x + d.y * d.y)
    }

    #[inline]
    pub fn contains(&self, p: impl GridPoint) -> bool {
        let p = p.as_ivec2() - self.center;
        let dist_sq = p.x * p.x + p.y * p.y;
        dist_sq <= (self.radius * self.radius) as i32
    }
}

impl GridShape for GridCircle {
    fn iter(&self) -> super::GridShapeIterator {
        super::GridShapeIterator::Circle(self.into_iter())
    }

    fn pos(&self) -> IVec2 {
        self.center
    }

    fn set_pos(&mut self, pos: IVec2) {
        self.center = pos;
    }

    fn bounds(&self) -> GridRect {
        let r = self.radius * 2;
        GridRect::new(self.center, [r, r])
    }
}

#[derive(Debug, Clone)]
pub struct GridCircleIter {
    rect_iter: GridRectIter,
    center: Vec2,
    radius: f32,
}

impl GridCircleIter {
    pub fn new(center: impl GridPoint, radius: usize) -> Self {
        let c = center.as_vec2() + 0.5;
        let r = radius as f32;
        let rect = GridRect::origin(IVec2::splat(radius as i32 * 2 + 1));
        GridCircleIter {
            rect_iter: rect.into_iter(),
            center: c,
            radius: r,
        }
    }
}

impl Iterator for GridCircleIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        for p in self.rect_iter.by_ref() {
            if inside_circle(p.as_vec2(), self.radius + 0.5) {
                return Some(self.center.as_ivec2() + p);
            }
        }

        None
    }
}

#[inline]
fn inside_circle(p: Vec2, radius: f32) -> bool {
    let dist_sq = p.x * p.x + p.y * p.y;
    dist_sq <= radius * radius
}

impl IntoIterator for GridCircle {
    type Item = IVec2;
    type IntoIter = GridCircleIter;

    fn into_iter(self) -> Self::IntoIter {
        GridCircleIter::new(self.center, self.radius)
    }
}

/// A hollow circle.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridCircleOutline {
    center: IVec2,
    radius: usize,
}

impl GridCircleOutline {
    pub fn new(center: impl GridPoint, radius: usize) -> Self {
        GridCircleOutline {
            center: center.as_ivec2(),
            radius,
        }
    }

    /// Create a circle centered a 0,0
    pub fn origin(radius: usize) -> Self {
        Self::new([0, 0], radius)
    }

    /// Create a filled circle with this circle's center and radius
    pub fn filled(&self) -> GridCircle {
        GridCircle::new(self.center, self.radius)
    }
}

#[derive(Debug, Clone)]
pub struct GridCircleOutlineIter {
    radius: f32,
    center: IVec2,
    r: usize,
    end: usize,
    points: [IVec2; 8],
    curr: usize,
}

impl GridShape for GridCircleOutline {
    fn iter(&self) -> super::GridShapeIterator {
        super::GridShapeIterator::CircleOutline(self.into_iter())
    }

    fn pos(&self) -> IVec2 {
        self.center
    }

    fn set_pos(&mut self, pos: IVec2) {
        self.center = pos;
    }

    fn bounds(&self) -> GridRect {
        let r = self.radius * 2;
        GridRect::new(self.center, [r, r])
    }
}

impl GridCircleOutlineIter {
    pub fn new(center: impl GridPoint, radius: usize) -> Self {
        let radius = radius as f32 + 0.5;
        let end = (radius * 0.5_f32.sqrt()).floor() as usize;

        GridCircleOutlineIter {
            radius,
            center: center.as_ivec2(),
            r: 0,
            end,
            points: Default::default(),
            curr: 8,
        }
    }
}

impl Iterator for GridCircleOutlineIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= 8 {
            if self.r > self.end {
                return None;
            }
            self.curr = 0;
            let r = self.r as f32;
            let d = (self.radius * self.radius - r * r).sqrt().floor();

            let c = self.center.as_vec2();
            self.points[0] = Vec2::new(c.x - d, c.y + r).as_ivec2();
            self.points[1] = Vec2::new(c.x + d, c.y + r).as_ivec2();
            self.points[2] = Vec2::new(c.x - d, c.y - r).as_ivec2();
            self.points[3] = Vec2::new(c.x + d, c.y - r).as_ivec2();
            self.points[4] = Vec2::new(c.x + r, c.y - d).as_ivec2();
            self.points[5] = Vec2::new(c.x + r, c.y + d).as_ivec2();
            self.points[6] = Vec2::new(c.x - r, c.y - d).as_ivec2();
            self.points[7] = Vec2::new(c.x - r, c.y + d).as_ivec2();

            self.r += 1;
        }
        let curr = self.points[self.curr];

        self.curr += 1;

        Some(curr)
    }
}

impl IntoIterator for GridCircleOutline {
    type Item = IVec2;
    type IntoIter = GridCircleOutlineIter;

    fn into_iter(self) -> Self::IntoIter {
        GridCircleOutlineIter::new(self.center, self.radius)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Canvas;

    use super::*;

    #[test]
    #[ignore]
    fn draw_circles() {
        for size in 1..15 {
            let empty_circle = GridCircleOutline::new([-(size as i32) / 2 - 2, 0], size);
            let mut canvas = Canvas::new([size * 4 + 3, size * 2 + 3]);

            for p in empty_circle {
                canvas.put(p, '*');
            }

            let filled_circle = GridCircle::new([size / 2 + 2, 0], size);

            for p in filled_circle {
                canvas.put(p, '*');
            }

            canvas.print();
            println!();
        }
    }
}
