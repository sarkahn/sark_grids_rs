use glam::{IVec2, Vec2};

use crate::{GridPoint, point::Point2d};

pub struct GridCone {
    pub origin: IVec2,
    pub angle_dir_rad: f32,
    pub angle_size_rad: f32,
    pub range: usize, 
}

impl GridCone {
    pub fn new(xy: impl GridPoint, dir_deg: f32, size_deg: f32, range: usize) -> Self {
        let angle_size_deg = size_deg + 1.0;
        Self {
            origin: xy.as_ivec2(),
            angle_dir_rad: dir_deg.to_radians(),
            angle_size_rad: angle_size_deg.to_radians(),
            range,
        }
    }

    pub fn origin(dir_deg: f32, size_deg: f32, range: usize) -> Self {
        Self::new([0,0], dir_deg, size_deg, range)
    }
}

pub struct GridConeIter {
    pub points: [Vec2;3],
    min: IVec2,
    width: usize,
    curr: usize,
    len: usize,
}

impl GridConeIter {
    pub fn from_cone(cone: &GridCone) -> Self {

        let points = calc_triangle_points(cone);

        let min = points.iter().cloned().reduce(|a,b|a.min(b)).unwrap();
        let mut max = points.iter().cloned().reduce(|a,b|a.max(b)).unwrap();

        max += IVec2::ONE;
        let d = max - min;

        let len = (d.x * d.y) as usize;

        let points = [
            points[0].as_vec2() + Vec2::splat(0.5),
            points[1].as_vec2() + Vec2::splat(0.5),
            points[2].as_vec2() + Vec2::splat(0.5),
        ];
        println!("Cone iter min {} max {} d {} len {}.\nPoints {}, {}, {}", min, max, d, len, points[0], points[1], points[2]);

        Self {
            points,
            min,
            curr: 0,
            width: d.x as usize,
            len,
        }
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
            
            let p = self.min + IVec2::new(x,y);
            let p_center = p.as_vec2() + Vec2::splat(0.5); 

            // if point_in_triangle(p_center, &self.points) {
            //     return Some(p);
            // }
            let [a,b,c] = [
                self.points[0].as_ivec2(),
                self.points[1].as_ivec2(),
                self.points[2].as_ivec2(),
            ];
            let inside = point_inside_trigon(p_center.as_ivec2(), a, b, c);
            println!("{} is inside: {}", p_center, inside);
            if inside {
                return Some(p);
            }
        }
        // for i in self.curr..self.len {
        //     self.curr += 1;

        //     let x = (i % self.width) as i32;
        //     let y = (i / self.width) as i32;

        //     let p = self.min + IVec2::new(x,y);
        //     let p_center = p.as_vec2() + Vec2::ONE * 0.5; 

        //     if point_in_triangle(p_center, &self.points) {
        //         return Some(p);
        //     }
        // }
        None
    }
}



impl IntoIterator for GridCone {
    type Item=IVec2;

    type IntoIter=GridConeIter;

    fn into_iter(self) -> Self::IntoIter {
        GridConeIter::from_cone(&self)
    }
}

#[inline]
fn sign(p1: Vec2, p2: Vec2, p3: Vec2) -> f32 {
    (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
}

#[inline]
fn point_in_triangle(pt:Vec2, points: &[Vec2;3]) -> bool {
    let (d1, d2, d3);
    let (has_neg, has_pos);

    d1 = sign(pt, points[0], points[1]);
    d2 = sign(pt, points[1], points[2]);
    d3 = sign(pt, points[2], points[1]);

    has_neg = (d1 < 0.) || (d2 < 0.) || (d3 < 0.);
    has_pos = (d1 > 0.) || (d2 > 0.) || (d3 > 0.);

    !(has_neg && has_pos)
}

// https://www.reddit.com/r/roguelikedev/comments/6htorz/wondering_if_there_is_any_easy_cone_algorithm_out/
fn calc_triangle_points(cone: &GridCone) -> [IVec2;3] {
    let origin = cone.origin;
    let angle = cone.angle_dir_rad;
    let size = cone.angle_size_rad;
    let dist = cone.range;
    let op = angle.sin() * dist as f32;
    let ad = angle.cos() * dist as f32;
    let t = origin + IVec2::new(ad.round() as i32, op.round() as i32);

    let dir = t - origin;

    let v = IVec2::new(dir.y * -1, dir.x).as_vec2();
    let scale = (size / 2.0).tan();
    let v = (v * scale).as_ivec2();
    let p1 = t + v;
    let p2 = t - v;

    let edge1 = origin;
    let edge2 = p1;
    let edge3 = p2;
    [edge1, edge2, edge3]
}

fn point_inside_trigon(
    s: impl GridPoint, 
    a: impl GridPoint, 
    b: impl GridPoint, 
    c: impl GridPoint
) -> bool {
    let s = s.as_ivec2();
    let a = a.as_ivec2();
    let b = b.as_ivec2();
    let c = c.as_ivec2();
    
    let as_x = s.x - a.x;
    let as_y = s.y - a.y;
    let s_ab = (b.x - a.x) * as_y - (b.y - a.y) * as_x > 0;

    let p1 = (c.x-a.x) * as_y - (c.y-a.y) * as_x;
    if (p1 > 0) == s_ab {
        return false;
    }

    let p1 = (c.x-b.x)*(s.y-b.y)-(c.y-b.y)*(s.x-b.x);
    if ( p1 > 0) != s_ab {
        return false;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::Canvas;

    #[test]
    fn tri() {
        let size = IVec2::new(10,10);
        let mut canvas = Canvas::new(size);

        let cone = GridCone::new([0,1], 0., 35., 4);

        let points = calc_triangle_points(&cone);

        for p in cone {
            canvas.put(p, '*');
        }

        for p in points {
            //canvas.put(p, 'P');
        }

        canvas.print();
    }
}