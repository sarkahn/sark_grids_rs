//! Utility for handling cones/triangles on a 2d grid.

use glam::IVec2;

use crate::GridPoint;

use super::{GridRect, GridShape, GridShapeIterator};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct GridCone {
    pub pos: IVec2,
    /// Angle of the cone in radians
    pub angle_dir_rad: f32,
    /// Size/Arc of the cone in radians
    pub angle_size_rad: f32,
    pub range: usize,
}

impl GridCone {
    /// Create a new grid from angles represented in degrees.
    ///
    /// Note an angle of `0.` points to the right, and an angle of `90.` points
    /// straight up (angle increases counter-clockwise).
    pub fn new(xy: impl GridPoint, dir_deg: f32, size_deg: f32, range: usize) -> Self {
        Self {
            pos: xy.as_ivec2(),
            angle_dir_rad: dir_deg.to_radians(),
            angle_size_rad: size_deg.to_radians(),
            range,
        }
    }

    /// Create a cone with it's position set to 0,0
    pub fn origin(dir_deg: f32, size_deg: f32, range: usize) -> Self {
        Self::new([0, 0], dir_deg, size_deg, range)
    }

    /// Retrieve the 3 grid positions of the corners of the cone
    ///
    /// The first point is the position of the cone, and the
    /// next two points are the two corners making up the cone triangle
    pub fn corners(&self) -> [IVec2; 3] {
        calc_triangle_points(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GridConeIter {
    points: [IVec2; 3],
    min: IVec2,
    width: usize,
    curr: usize,
    len: usize,
}

impl GridConeIter {
    pub fn from_cone(cone: &GridCone) -> Self {
        let points = calc_triangle_points(cone);

        let min = points.iter().cloned().reduce(|a, b| a.min(b)).unwrap();
        let max = points.iter().cloned().reduce(|a, b| a.max(b)).unwrap();
        let max = max + 1;
        let d = max - min;

        let len = (d.x * d.y) as usize;

        Self {
            points,
            min,
            curr: 0,
            width: d.x as usize,
            len,
        }
    }
}

impl GridShape for GridCone {
    fn iter(&self) -> super::GridShapeIterator {
        GridShapeIterator::Cone(GridConeIter::from_cone(self))
    }

    fn pos(&self) -> IVec2 {
        self.pos
    }

    fn set_pos(&mut self, pos: IVec2) {
        self.pos = pos;
    }

    fn bounds(&self) -> GridRect {
        let min = self
            .corners()
            .into_iter()
            .reduce(|a, b| a.min(b))
            .expect("Error getting corners from grid cone");
        let max = self.corners().into_iter().reduce(|a, b| a.max(b)).unwrap();

        GridRect::from_points(min, max)
    }
}

impl Iterator for GridConeIter {
    type Item = IVec2;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.curr < self.len {
            let curr = self.curr;
            self.curr += 1;

            let x = (curr % self.width) as i32;
            let y = (curr / self.width) as i32;

            let p = self.min + IVec2::new(x, y);
            let inside = point_in_triangle(p, &self.points);
            if inside {
                return Some(p);
            }
        }
        None
    }
}

impl IntoIterator for GridCone {
    type Item = IVec2;

    type IntoIter = GridConeIter;

    fn into_iter(self) -> Self::IntoIter {
        GridConeIter::from_cone(&self)
    }
}

#[inline]
fn sign(p1: IVec2, p2: IVec2, p3: IVec2) -> i32 {
    (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
}

// https://stackoverflow.com/a/2049593
#[inline]
fn point_in_triangle(pt: impl GridPoint, tri: &[IVec2; 3]) -> bool {
    let pt = pt.as_ivec2();
    let (d1, d2, d3);
    let (has_neg, has_pos);

    d1 = sign(pt, tri[0], tri[1]);
    d2 = sign(pt, tri[1], tri[2]);
    d3 = sign(pt, tri[2], tri[0]);

    has_neg = (d1 < 0) || (d2 < 0) || (d3 < 0);
    has_pos = (d1 > 0) || (d2 > 0) || (d3 > 0);

    !(has_neg && has_pos)
}

// https://www.reddit.com/r/roguelikedev/comments/6htorz/wondering_if_there_is_any_easy_cone_algorithm_out/
fn calc_triangle_points(cone: &GridCone) -> [IVec2; 3] {
    let origin = cone.pos;
    let angle = cone.angle_dir_rad;
    let size = cone.angle_size_rad;
    let dist = cone.range;
    let op = angle.sin() * dist as f32;
    let ad = angle.cos() * dist as f32;
    let t = origin + IVec2::new(ad.round() as i32, op.round() as i32);

    let dir = t - origin;

    let v = IVec2::new(-dir.y, dir.x).as_vec2();
    let scale = (size / 2.0).tan();
    let v = (v * scale).as_ivec2();
    let p1 = t + v;
    let p2 = t - v;

    let edge1 = origin;
    let edge2 = p1;
    let edge3 = p2;
    [edge1, edge2, edge3]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::Canvas;

    #[test]
    fn perp() {
        let size = IVec2::new(20, 20);
        let mut canvas = Canvas::new(size + 1);

        let arc = 85.;
        let len = (size.x / 2 - 1) as usize;
        let cones = [
            GridCone::origin(0., arc, len),
            GridCone::origin(90., arc, len),
            GridCone::origin(180., arc, len),
            GridCone::origin(270., arc, len),
        ];

        for cone in cones {
            for p in cone {
                canvas.put(p, '*');
                for (i, p) in cone.corners().iter().enumerate() {
                    let i = char::from_digit(i as u32, 10).unwrap();
                    canvas.put(*p, i);
                }
            }
        }

        canvas.print();
    }

    #[test]
    fn diag() {
        let size = IVec2::new(11, 11);
        let mut canvas = Canvas::new(size + 1);

        let arc = 45.;
        let len = (size.x / 2 - 1) as usize;
        let cones = [
            GridCone::origin(45., arc, len),
            GridCone::origin(135., arc, len),
            GridCone::origin(225., arc, len),
            GridCone::origin(315., arc, len),
        ];

        for cone in cones {
            for p in cone {
                canvas.put(p, '*');
                for (i, p) in cone.corners().iter().enumerate() {
                    let i = char::from_digit(i as u32, 10).unwrap();
                    canvas.put(*p, i);
                }
            }
        }

        canvas.print();
    }
}
