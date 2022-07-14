//! Utility for drawing circular shapes on a 2d grid.
// https://www.redblobgames.com/grids/circle-drawing/

use glam::{IVec2, Vec2};

use crate::GridPoint;

use super::{grid_rect::GridRectIter, GridShape, ShapeIterator};

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
    fn iter(&self) -> ShapeIterator {
        ShapeIterator::EmptyCircle(self.center, EmptyCircleIterator::new(self.radius))
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
    fn iter(&self) -> ShapeIterator {
        ShapeIterator::FilledCircle(self.center, FilledCircleIterator::new(self.radius))
    }
}

pub struct EmptyCircleIterator {
    radius: f32,
    center: Vec2,
    r: usize,
    end: usize,
    points: [IVec2; 8],
    curr: usize,
}

impl EmptyCircleIterator {
    pub fn new(radius: usize) -> Self {
        let radius = radius as f32 + 0.5;
        let end = (radius * 0.5_f32.sqrt()).floor() as usize;

        EmptyCircleIterator {
            radius,
            center: Vec2::splat(0.5),
            r: 0,
            end,
            points: Default::default(),
            curr: 8,
        }
    }
}

impl Iterator for EmptyCircleIterator {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= 8 {
            if self.r > self.end {
                return None;
            }
            self.curr = 0;
            let r = self.r as f32;
            let d = (self.radius * self.radius - r * r).sqrt().floor();

            let c = self.center;
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

pub struct FilledCircleIterator {
    iter: GridRectIter,
    center: Vec2,
    radius: f32,
}

impl FilledCircleIterator {
    pub fn new(radius: usize) -> Self {
        let c = Vec2::splat(0.5);
        let r = radius as f32 + 0.5;
        let bl = IVec2::new((c.x - r).floor() as i32, (c.y - r).floor() as i32);
        let tr = IVec2::new((c.x + r).ceil() as i32, (c.y + r).ceil() as i32);
        let iter = GridRectIter::new((tr - bl).as_uvec2());
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

impl From<FilledCircleIterator> for ShapeIterator {
    fn from(iter: FilledCircleIterator) -> Self {
        ShapeIterator::FilledCircle(iter.center.as_ivec2(), iter)
    }
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
