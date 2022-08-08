//! Utility for drawing circular shapes on a 2d grid.
// https://www.redblobgames.com/grids/circle-drawing/

use glam::{IVec2, UVec2, Vec2};

use crate::GridPoint;

use super::{grid_rect::GridRectIter, GridRect, GridShape};

/// A filled circle. Points within the circle can be iterator over.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridCircle {
    center: IVec2,
    radius: usize,
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

    /// Create an outlined circle with this filled circles position and size.
    pub fn outline(&self) -> GridCircleOutline {
        GridCircleOutline::new(self.center, self.radius)
    }
}

impl GridShape for GridCircle {
    fn iter(&self) -> super::GridShapeIterator {
        super::GridShapeIterator::Circle(self.into_iter())
    }
}

#[derive(Debug, Clone)]
pub struct GridCircleIter {
    iter: GridRectIter,
    center: Vec2,
    radius: f32,
    bl: IVec2,
}

impl GridCircleIter {
    pub fn new(center: impl GridPoint, radius: usize) -> Self {
        let c = center.as_vec2() + Vec2::splat(0.5);
        let r = radius as f32;
        let bl = IVec2::new((c.x - r).floor() as i32, (c.y - r).floor() as i32);
        let rect = GridRect::origin(UVec2::splat(radius as u32 * 2 + 1));
        GridCircleIter {
            iter: rect.into_iter(),
            center: c,
            radius: r,
            bl,
        }
    }
}

impl Iterator for GridCircleIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        for p in self.iter.by_ref() {
            let p = self.bl + p;
            if inside_circle(self.center, p.as_vec2() + 0.5, self.radius + 0.5) {
                return Some(p);
            }
        }

        None
    }
}

#[inline]
fn inside_circle(center: Vec2, point: Vec2, radius: f32) -> bool {
    let d = center - point;
    let dist_sq = d.x * d.x + d.y * d.y;
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

            let c = self.center.as_vec2() + Vec2::splat(0.5);
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
    fn iter_outline() {
        let size = 2;
        let empty_circle = GridCircleOutline::new([8, 8], size);
        let mut canvas = Canvas::new([50, 16]);

        for p in empty_circle {
            canvas.put(p, '*');
        }

        let filled_circle = GridCircle::new([30, 8], size);

        for p in filled_circle {
            canvas.put(p, '*');
        }

        canvas.print();
    }
}
