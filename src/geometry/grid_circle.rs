//! Utility for drawing circular shapes on a 2d grid.
// https://www.redblobgames.com/grids/circle-drawing/
use std::iter::Flatten;

use glam::{IVec2, Vec2};

use crate::GridPoint;

use super::{grid_rect::GridRectIter, GridRect, GridShape};

/// A hollow circle.
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
}

impl GridShape for GridCircleOutline {
    type Iterator = Flatten<EmptyCircleIterator>;

    fn iter(&self) -> Self::Iterator {
        EmptyCircleIterator::new(self).flatten()
    }
}

/// A filled circle.
pub struct GridCircleFilled {
    center: IVec2,
    radius: usize,
}

impl GridCircleFilled {
    pub fn new(center: impl GridPoint, radius: usize) -> Self {
        GridCircleFilled {
            center: center.as_ivec2(),
            radius,
        }
    }
}

impl GridShape for GridCircleFilled {
    type Iterator = FilledCircleIterator;

    fn iter(&self) -> Self::Iterator {
        FilledCircleIterator::new(self)
    }
}

pub struct EmptyCircleIterator {
    radius: f32,
    center: Vec2,
    r: usize,
    end: usize,
}

impl EmptyCircleIterator {
    fn new(circle: &GridCircleOutline) -> Self {
        let radius = circle.radius as f32 + 0.5;
        let end = (radius * 0.5_f32.sqrt()).floor() as usize;

        EmptyCircleIterator {
            radius,
            center: circle.center.as_vec2() + 0.5,
            r: 0,
            end,
        }
    }
}

impl Iterator for EmptyCircleIterator {
    type Item = [IVec2; 8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.r > self.end {
            return None;
        }

        let r = self.r as f32;
        let d = (self.radius * self.radius - r * r).sqrt().floor();

        let c = self.center;
        let points = [
            Vec2::new(c.x - d, c.y + r).as_ivec2(),
            Vec2::new(c.x + d, c.y + r).as_ivec2(),
            Vec2::new(c.x - d, c.y - r).as_ivec2(),
            Vec2::new(c.x + d, c.y - r).as_ivec2(),
            Vec2::new(c.x + r, c.y - d).as_ivec2(),
            Vec2::new(c.x + r, c.y + d).as_ivec2(),
            Vec2::new(c.x - r, c.y - d).as_ivec2(),
            Vec2::new(c.x - r, c.y + d).as_ivec2(),
        ];

        self.r += 1;

        Some(points)
    }
}

pub struct FilledCircleIterator {
    iter: GridRectIter,
    center: Vec2,
    radius: f32,
}

impl FilledCircleIterator {
    pub fn new(circle: &GridCircleFilled) -> Self {
        let c = circle.center.as_vec2() + 0.5;
        let r = circle.radius as f32 + 0.5;
        let bl = IVec2::new((c.x - r).floor() as i32, (c.y - r).floor() as i32);
        let tr = IVec2::new((c.x + r).ceil() as i32, (c.y + r).ceil() as i32);
        let iter = GridRect::new(bl, tr - bl).iter();
        FilledCircleIterator {
            iter,
            center: c,
            radius: r,
        }
    }
}

impl Iterator for FilledCircleIterator {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        for p in self.iter.by_ref() {
            if inside_circle(self.center, p.as_vec2() + 0.5, self.radius) {
                return Some(p);
            }
        }

        None
    }
}

fn inside_circle(center: Vec2, point: Vec2, radius: f32) -> bool {
    let d = center - point;
    let dist_sq = d.x * d.x + d.y * d.y;
    dist_sq <= radius * radius
}

#[cfg(test)]
mod tests {
    use crate::util::Canvas;

    use super::*;

    #[test]
    fn iter_outline() {
        let size = 5;
        let circle = GridCircleOutline::new([8, 8], size);
        let mut canvas = Canvas::new([50, 16]);

        for p in circle.iter() {
            canvas.put(p, '*');
        }

        let filled_circle = GridCircleFilled::new([30, 8], size);

        for p in filled_circle.iter() {
            canvas.put(p, '*');
        }

        canvas.print();
    }
}
