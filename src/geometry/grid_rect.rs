//! Utility for handling rectangles on a 2d grid.
use std::{
    fmt::{self, Display},
    ops::{Add, Deref, Sub},
};

use glam::{IVec2, Mat2, Vec2};

use crate::{GridPoint, Pivot};

use super::GridShape;

/// A rectangle of points on a grid.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridRect {
    pub xy: IVec2,
    pub size: IVec2,
}

impl GridRect {
    /// Create a [GridRect] from a min position (bottom left tile) and a size.
    pub fn new(xy: impl Into<IVec2>, size: impl Into<IVec2>) -> Self {
        GridRect {
            xy: xy.into(),
            size: size.into().as_ivec2(),
        }
    }

    /// Create a [GridRect] with it's center at origin (`[0,0]`)
    pub fn center_origin(size: impl Into<IVec2>) -> Self {
        Self::from_center_size([0, 0], size)
    }

    /// Create a [GridRect] from two points in space.
    pub fn from_points(a: impl Into<IVec2>, b: impl Into<IVec2>) -> Self {
        let [a, b] = [a.into(), b.into()];
        let min = a.min(b);
        let max = a.max(b);

        let size = (max - min) + 1;
        GridRect { xy: min, size }
    }

    /// Create a [GridRect] from a center position and rect size.
    pub fn from_center_size(center: impl Into<IVec2>, size: impl Into<IVec2>) -> Self {
        let size = size.into();
        let bl = center.into() - size.as_ivec2() / 2;
        Self::new(bl, size)
    }

    pub fn from_pivot_pos(pivot: Pivot, pos: impl Into<IVec2>, size: impl Into<IVec2>) -> Self {
        let [x, y] = pos.into().to_array();
        let [w, h] = size.into().sub(1).to_array();
        let bottom_left = match pivot {
            Pivot::TopLeft => [x, y - h],
            Pivot::TopRight => [x - w, y - h],
            Pivot::Center => [x - w / 2, y - h / 2],
            Pivot::BottomLeft => [x, y],
            Pivot::BottomRight => [x - w, y],
        };
        let top_right = match pivot {
            Pivot::TopLeft => [x + w, y],
            Pivot::TopRight => [x, y],
            Pivot::Center => [bottom_left[0] + w, bottom_left[1] + h],
            Pivot::BottomLeft => [x + w, y + h],
            Pivot::BottomRight => [x, y + h],
        };
        Self::from_points(bottom_left, top_right)
    }

    /// Returns a [GridRect] clipped by the bounds of the given [GridRect]
    pub fn clipped(&self, clipper: GridRect) -> GridRect {
        let [bmin, bmax] = [clipper.min(), clipper.max()];
        let [amin, amax] = [self.min(), self.max()];
        let max = amax.min(bmax);
        let min = amin.max(bmin);
        GridRect::from_points(min, max)
    }

    /// Adjusts a single corner of the rect to contain the given point.
    pub fn envelope_point(&mut self, point: impl Into<IVec2>) {
        let point = point.into();
        let min = self.min().min(point);
        let max = self.max().max(point);
        *self = GridRect::from_points(min, max);
    }

    /// Adjust this rect so the given rect is entirely contained within it.
    pub fn envelope_rect(&mut self, rect: GridRect) {
        let [min, max] = [rect.min(), rect.max()];
        self.envelope_point(min);
        self.envelope_point(max);
    }

    /// The center tile of the rect.
    pub fn center(&self) -> IVec2 {
        self.xy + self.size / 2
    }

    /// The bottom-left tile of the rect.
    pub fn bottom_left(&self) -> IVec2 {
        self.xy
    }

    /// The top left tile of the rect.
    pub fn top_left(&self) -> IVec2 {
        self.xy + IVec2::Y * self.size.y.sub(1)
    }

    /// The top right tile of the rect.
    pub fn top_right(&self) -> IVec2 {
        self.max()
    }

    /// The bottom right tile of the rect.
    pub fn bottom_right(&self) -> IVec2 {
        self.xy + IVec2::X * self.size.x.sub(1)
    }

    pub fn width(&self) -> usize {
        self.size.x as usize
    }

    pub fn height(&self) -> usize {
        self.size.y as usize
    }

    /// Bottom left position of the rect.
    pub fn min(&self) -> IVec2 {
        self.xy
    }

    /// Top right position of the rect.
    pub fn max(&self) -> IVec2 {
        self.xy + self.size.sub(1)
    }

    /// Index of the bottom row of the rect
    pub fn bottom(&self) -> usize {
        0
    }

    /// Index of the top row of the rect.
    pub fn top(&self) -> usize {
        self.height().sub(1)
    }

    /// The index of the left column of the rect.
    pub fn left(&self) -> usize {
        0
    }

    /// The index of the right most column of the rect.
    pub fn right(&self) -> usize {
        self.width().sub(1)
    }

    /// Returns the 4 corners of the rect, which can be accessed by index
    /// or name.
    ///
    /// Order is BottomLeft, TopLeft, TopRight, BottomRight
    #[inline]
    pub fn corners(&self) -> Corners {
        let [min, max] = [self.min(), self.max()];
        Corners([min, [min.x, max.y].into(), max, [max.x, min.y].into()])
    }

    /// Iterate over each point of the rect.
    pub fn iter(&self) -> GridRectIter {
        GridRectIter::new(*self)
    }

    /// Iterate over a single column of the rect.
    pub fn iter_col(
        &self,
        col: usize,
    ) -> impl DoubleEndedIterator<Item = IVec2> + ExactSizeIterator {
        self.iter().skip(col).step_by(self.width())
    }

    /// Iterate iver a single row of the rect.
    pub fn iter_row(
        &self,
        row: usize,
    ) -> impl DoubleEndedIterator<Item = IVec2> + ExactSizeIterator {
        self.iter().skip(row * self.width()).take(self.width())
    }

    /// Iterate over all the border tiles of the rect in clockwise order,
    /// starting from the bottom left.
    pub fn iter_border(&self) -> impl DoubleEndedIterator<Item = IVec2> {
        let left = self.iter_col(0);
        let top = self.iter_row(self.top()).skip(1).take(self.width().sub(2));
        let right = self.iter_col(self.right()).rev();
        let bottom = self
            .iter_row(self.bottom())
            .rev()
            .skip(1)
            .take(self.width().sub(2));
        left.chain(top).chain(right).chain(bottom)
    }

    /// Retrieve the position of the tile at the given pivot point.
    pub fn pivot_point(&self, pivot: Pivot) -> IVec2 {
        let min = self.min();
        let max = self.max();
        match pivot {
            Pivot::TopLeft => [min.x, max.y],
            Pivot::TopRight => [max.x, max.y],
            Pivot::Center => self.center().to_array(),
            Pivot::BottomLeft => [min.x, min.y],
            Pivot::BottomRight => [max.x, max.y],
        }
        .into()
    }

    /// Check if a given point lies inside the rect
    #[inline]
    pub fn contains(&self, p: impl Into<IVec2>) -> bool {
        let p = p.into();
        !(p.cmplt(self.min()).any() || p.cmpgt(self.max()).any())
    }

    /// Returns true if the given rect is entirely contained within this one.
    #[inline]
    pub fn contains_rect(&self, rect: GridRect) -> bool {
        let [amin, amax] = [self.min(), self.max()];
        let [bmin, bmax] = [rect.min(), rect.max()];
        bmin.cmpge(amin).all() && bmax.cmple(amax).all()
    }

    /// Check if any part of a rect overlaps another
    #[inline]
    pub fn overlaps(&self, other: GridRect) -> bool {
        let ac = self.center().as_vec2();
        let bc = other.center().as_vec2();
        let ae = self.size.as_vec2() / 2.0;
        let be = other.size.as_vec2() / 2.0;

        ac.sub(bc).abs().cmple(ae.add(be)).all()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridRectIter {
    origin: IVec2,
    size: IVec2,
    head: IVec2,
    tail: IVec2,
}

impl GridRectIter {
    pub fn new(rect: GridRect) -> Self {
        Self {
            origin: rect.xy,
            size: rect.size,
            head: IVec2::ZERO,
            tail: rect.size.sub(1),
        }
    }
}

impl Iterator for GridRectIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.size;
        let head = &mut self.head;
        let tail = self.tail;

        if head.y > tail.y || (head.y == tail.y && head.x > tail.x) {
            None
        } else {
            let ret = self.origin + *head;
            head.x += 1;
            if head.x >= size.x {
                head.x = 0;
                head.y += 1;
            }

            Some(ret)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.size;
        let head = self.head;
        let tail = size.sub(1) - self.tail;
        let head_count = size.x * head.y + head.x;
        let tail_count = size.x * tail.y + tail.x;
        let rem = (size.x * size.y - (head_count + tail_count)) as usize;
        (rem, Some(rem))
    }
}

impl DoubleEndedIterator for GridRectIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let size = self.size;
        let tail = &mut self.tail;
        let head = self.head;

        if tail.y < head.y || (tail.y == head.y && tail.x < head.x) {
            None
        } else {
            let ret = self.origin + *tail;
            tail.x -= 1;
            if tail.x < 0 {
                tail.x = size.x.sub(1);
                tail.y -= 1;
            }

            Some(ret)
        }
    }
}

impl ExactSizeIterator for GridRectIter {}

impl IntoIterator for GridRect {
    type Item = IVec2;

    type IntoIter = GridRectIter;

    fn into_iter(self) -> Self::IntoIter {
        GridRectIter::new(self)
    }
}

/// The corner points of a [GridRect]. Corners can be accessed by name, index,
/// or iterated over.
#[derive(Default, Debug, Clone, Copy)]
pub struct Corners(pub [IVec2; 4]);

impl Corners {
    pub fn bl(&self) -> IVec2 {
        self.0[0]
    }
    pub fn tl(&self) -> IVec2 {
        self.0[1]
    }
    pub fn tr(&self) -> IVec2 {
        self.0[2]
    }
    pub fn br(&self) -> IVec2 {
        self.0[3]
    }
}

impl Deref for Corners {
    type Target = [IVec2; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for Corners {
    type Item = IVec2;

    type IntoIter = core::array::IntoIter<IVec2, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// impl Display for GridRect {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "GridRect {{
//     MinMax {:?}, {:?}
//     Center {:?}
//     Size   {:?}
// }}",
//             self.min_i().to_array(),
//             self.max_i().to_array(),
//             self.center.to_array(),
//             self.size.to_array()
//         )
//     }
// }

// impl GridRect {
//     pub fn new(center: impl GridPoint, size: impl GridPoint) -> GridRect {
//         GridRect {
//             center: center.as_ivec2(),
//             size: size.as_ivec2(),
//             extents: size.as_vec2() / 2.0,
//         }
//     }

//     /// Create a grid rect with it's center set to 0,0
//     pub fn origin(size: impl GridPoint) -> Self {
//         Self::new([0, 0], size)
//     }

//     pub fn width(&self) -> usize {
//         self.size.x as usize
//     }

//     pub fn height(&self) -> usize {
//         self.size.y as usize
//     }

//     /// Create a grid rect from a min and max position.
//     pub fn from_points(a: impl GridPoint, b: impl GridPoint) -> GridRect {
//         let min = a.as_ivec2().min(b.as_ivec2());
//         let max = a.as_ivec2().max(b.as_ivec2());
//         let size = (max - min) + 1;
//         let half = size / 2;
//         GridRect {
//             center: min + half,
//             size,
//             extents: size.as_vec2() / 2.0,
//         }
//     }

//     /// Create a rect with the bottom left corner at the given position.
//     pub fn from_bl(pos: impl GridPoint, size: impl GridPoint) -> GridRect {
//         GridRect::from_points(pos, pos.as_ivec2() + (size.as_ivec2() - 1))
//     }

//     #[inline]
//     /// Retrieve the bottom-left-most point of the rect
//     pub fn min(&self) -> Vec2 {
//         self.center.as_vec2().add(0.5) - self.extents
//     }

//     #[inline]
//     /// Retrieve the top-right-most point of the rect
//     pub fn max(&self) -> Vec2 {
//         self.center.as_vec2().add(0.5) + self.extents
//     }

//     /// Retrieve the bottom-left-most point of the rect as a grid position
//     #[inline]
//     pub fn min_i(&self) -> IVec2 {
//         self.min().floor().as_ivec2()
//     }

//     /// Retrieve the top-right-most point of the rect as a grid position
//     #[inline]
//     pub fn max_i(&self) -> IVec2 {
//         self.min_i() + self.size.sub(1)
//     }

//     pub fn min_max_i(&self) -> [IVec2; 2] {
//         [self.min_i(), self.max_i()]
//     }

//     pub fn size(&self) -> IVec2 {
//         self.size
//     }

//     #[inline]
//     /// Retrieve the position of a given corner of the rect
//     pub fn pivot_point(&self, pivot: Pivot) -> IVec2 {
//         let [min, max] = self.min_max_i();
//         match pivot {
//             Pivot::TopLeft => [min.x, max.y],
//             Pivot::TopRight => [max.x, max.y],
//             Pivot::BottomLeft => [min.x, min.y],
//             Pivot::BottomRight => [max.x, min.y],
//             Pivot::Center => self.center.to_array(),
//         }
//         .into()
//     }

//     /// Return a rect with the same center but resized by the given amount
//     /// on each axis
//     pub fn resized(&self, amount: impl GridPoint) -> GridRect {
//         let size = (self.size + amount.as_ivec2()).max(IVec2::ONE).as_ivec2();
//         GridRect::new(self.center, size)
//     }

//     /// Returns a rect adjusted to the given pivot.
//     pub fn pivoted(&self, pivot: Pivot) -> GridRect {
//         let center = self.center.as_vec2();
//         let pivot = Vec2::from(pivot) - 0.5;
//         let center = center - self.size.as_vec2() * pivot;
//         GridRect::new(center.floor().as_ivec2(), self.size)
//     }

//     /// Returns a rect with it's position adjusted by the given amount
//     pub fn translated(&self, xy: impl GridPoint) -> GridRect {
//         GridRect::new(self.center + xy.as_ivec2(), self.size)
//     }

//     /// Check if a given point lies inside the rect
//     #[inline]
//     pub fn contains(&self, p: impl GridPoint) -> bool {
//         let p = p.as_ivec2();
//         !(p.cmplt(self.min_i()).any() || p.cmpgt(self.max_i()).any())
//     }

//     /// Returns true if the given rect is entirely contained within this one.
//     #[inline]
//     pub fn contains_rect(&self, rect: GridRect) -> bool {
//         let [amin, amax] = self.min_max_i();
//         let [bmin, bmax] = rect.min_max_i();
//         bmin.cmpge(amin).all() && bmax.cmple(amax).all()
//     }

//     /// Check if any part of a rect overlaps another
//     #[inline]
//     pub fn overlaps(&self, other: GridRect) -> bool {
//         let ac = self.center.as_vec2();
//         let bc = other.center.as_vec2();
//         let ar = self.extents;
//         let br = other.extents;

//         ac.sub(bc).abs().cmple(ar.add(br)).all()
//     }

//     /// Adjusts a single corner of the rect to contain the given point if it
//     /// isn't already.
//     pub fn envelope_point(&mut self, point: impl GridPoint) {
//         let point = point.as_ivec2();
//         let min = self.min_i().min(point);
//         let max = self.max_i().max(point);
//         *self = GridRect::from_points(min, max);
//     }

//     pub fn envelope_rect(&mut self, rect: GridRect) {
//         let [min, max] = rect.min_max_i();
//         self.envelope_point(min);
//         self.envelope_point(max);
//     }

//     /// Returns the 4 corners of the rect, which can be accessed by index
//     /// or name.
//     ///
//     /// Order is BottomLeft, TopLeft, TopRight, BottomRight
//     #[inline]
//     pub fn corners(&self) -> Corners {
//         let [min, max] = self.min_max_i();
//         Corners([min, [min.x, max.y].into(), max, [max.x, min.y].into()])
//     }

//     /// Returns an iterator that visits the position of every border tile
//     /// in the rect.
//     pub fn iter_border(&self) -> BorderIterator {
//         let [min, max] = self.min_max_i();
//         BorderIterator::new(min, max)
//     }

//     /// Returns a rect clipped by the bounds of the given rect
//     pub fn clipped(&self, clipper: GridRect) -> GridRect {
//         let [bmin, bmax] = clipper.min_max_i();
//         let [amin, amax] = self.min_max_i();
//         let max = amax.min(bmax);
//         let min = amin.max(bmin);
//         GridRect::from_points(min, max)
//     }
// }

// /// The corner points of a [GridRect]. Corners can be accessed by name, index,
// /// or iterated over.
// #[derive(Default, Debug, Clone, Copy)]
// pub struct Corners(pub [IVec2; 4]);

// impl Corners {
//     pub fn bl(&self) -> IVec2 {
//         self.0[0]
//     }
//     pub fn tl(&self) -> IVec2 {
//         self.0[1]
//     }
//     pub fn tr(&self) -> IVec2 {
//         self.0[2]
//     }
//     pub fn br(&self) -> IVec2 {
//         self.0[3]
//     }
// }

// impl Deref for Corners {
//     type Target = [IVec2; 4];

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl IntoIterator for Corners {
//     type Item = IVec2;

//     type IntoIter = core::array::IntoIter<IVec2, 4>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// impl GridShape for GridRect {
//     fn iter(&self) -> super::GridShapeIterator {
//         super::GridShapeIterator::Rect(self.into_iter())
//     }

//     fn pos(&self) -> IVec2 {
//         self.center
//     }

//     fn set_pos(&mut self, pos: IVec2) {
//         self.center = pos;
//     }

//     fn bounds(&self) -> GridRect {
//         self.to_owned()
//     }
// }

// #[derive(Debug, Clone)]
// pub struct GridRectIter {
//     origin: IVec2,
//     curr: IVec2,
//     size: IVec2,
// }

// impl GridRectIter {
//     pub fn new(center: impl GridPoint, size: impl GridPoint) -> Self {
//         let size = size.as_ivec2();
//         GridRectIter {
//             origin: center.as_ivec2() - size / 2,
//             curr: IVec2::ZERO,
//             size,
//         }
//     }
// }

// impl Iterator for GridRectIter {
//     type Item = IVec2;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.curr.cmpge(self.size).any() {
//             return None;
//         }

//         let p = self.curr;
//         self.curr.x += 1;
//         if self.curr.x == self.size.x {
//             self.curr.x = 0;
//             self.curr.y += 1;
//         }
//         Some(self.origin + p)
//     }
// }

// impl IntoIterator for GridRect {
//     type Item = IVec2;
//     type IntoIter = GridRectIter;

//     fn into_iter(self) -> Self::IntoIter {
//         GridRectIter::new(self.center, self.size)
//     }
// }

// pub struct BorderIterator {
//     start: IVec2,
//     dir: IVec2,
//     curr: IVec2,
//     dest: IVec2,
//     size: IVec2,
// }

// impl BorderIterator {
//     pub fn new(min: impl GridPoint, max: impl GridPoint) -> Self {
//         let dir = IVec2::Y;
//         let rect = GridRect::from_points(min, max);
//         let size = rect.size - 1;
//         let start = rect.min_i();
//         let curr = start;
//         let dest = start + dir * size.y;
//         Self {
//             start,
//             dir,
//             curr,
//             dest,
//             size,
//         }
//     }
// }

// const ROT_CLOCKWISE: Mat2 = Mat2::from_cols_array(&[0., -1., 1., 0.]);

// impl Iterator for BorderIterator {
//     type Item = IVec2;

//     fn next(&mut self) -> Option<Self::Item> {
//         let curr = self.curr;

//         if curr == self.start && self.dir.x == -1 {
//             return None;
//         }

//         if curr == self.dest {
//             self.dir = ROT_CLOCKWISE.mul_vec2(self.dir.as_vec2()).as_ivec2();
//             self.dest = self.curr + self.dir * self.size;
//         }
//         self.curr += self.dir;

//         Some(curr)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{util::Canvas, Pivot};

    use super::GridRect;

    #[test]
    fn rect_min_max() {
        let rect = GridRect::from_points([1, 1], [3, 3]);
        assert_eq!([1, 1], rect.min().to_array());
        assert_eq!([3, 3], rect.max().to_array());

        let rect = GridRect::from_points([0, 0], [3, 3]);
        assert_eq!([0, 0], rect.min().to_array());
        assert_eq!([3, 3], rect.max().to_array());

        let rect = GridRect::from_points([-1, -1], [4, 4]);
        assert_eq!([-1, -1], rect.min().to_array());
        assert_eq!([4, 4], rect.max().to_array());

        let rect = GridRect::from_points([-5, -5], [3, 3]);
        assert_eq!([-5, -5], rect.min().to_array());
        assert_eq!([3, 3], rect.max().to_array());

        let rect = GridRect::from_points([6, 6], [7, 7]);
        assert_eq!([6, 6], rect.min().to_array());
        assert_eq!([7, 7], rect.max().to_array());
    }

    #[test]
    fn contains_point() {
        let rect = GridRect::center_origin([5, 5]);
        assert!(rect.contains([-2, -2]));
        assert!(rect.contains([2, 2]));
        assert!(!rect.contains([3, 3]));
        assert!(!rect.contains([-3, -3]));
    }

    #[test]
    fn from_bl() {
        let rect = GridRect::center_origin([5, 5]);
        let rect2 = GridRect::new([-2, -2], [5, 5]);

        assert_eq!(rect, rect2);
    }

    #[test]
    fn overlap() {
        let a = GridRect::new([-1, -1], [3, 3]);
        let b = GridRect::new([1, 1], [3, 3]);
        let c = GridRect::new([3, 3], [3, 3]);
        assert!(a.overlaps(b));
        assert!(b.overlaps(a));
        assert!(b.overlaps(c));
        assert!(c.overlaps(b));
        assert!(!a.overlaps(c));
        assert!(!c.overlaps(a));

        assert!(a.overlaps(a));
        assert!(b.overlaps(b));
        assert!(c.overlaps(c));

        let a = GridRect::new([-2, -2], [4, 4]);
        let b = GridRect::new([1, 1], [4, 4]);
        let c = GridRect::new([4, 4], [4, 4]);
        assert!(a.overlaps(b));
        assert!(b.overlaps(a));
        assert!(b.overlaps(c));
        assert!(c.overlaps(b));
        assert!(!a.overlaps(c));
        assert!(!c.overlaps(a));

        assert!(a.overlaps(a));
        assert!(b.overlaps(b));
        assert!(c.overlaps(c));
    }

    #[test]
    #[ignore]
    fn iter() {
        let rect = GridRect::center_origin([3, 3]);
        let mut canvas = Canvas::new([5, 5]);
        for p in rect {
            canvas.put(p, '*');
        }
        canvas.print();
    }

    #[test]
    #[ignore]
    fn big() {
        let rect = GridRect::center_origin([30, 30]);
        let mut canvas = Canvas::new([32, 32]);

        for p in rect {
            canvas.put(p, '*');
        }
        canvas.print();
    }

    #[test]
    fn envelope_point() {
        let mut rect = GridRect::from_points([1, 1], [3, 3]);
        rect.envelope_point([0, 0]);
        assert_eq!([0, 0], rect.min().to_array());
        assert_eq!([3, 3], rect.max().to_array());
        assert_eq!(4, rect.width());

        rect.envelope_point([4, 3]);
        assert_eq!([0, 0], rect.min().to_array());
        assert_eq!([4, 3], rect.max().to_array());
        assert_eq!(4, rect.height());

        rect.envelope_point([0, 0]);
        assert_eq!([0, 0], rect.min().to_array());
        assert_eq!([4, 3], rect.max().to_array());
    }

    #[test]
    fn envelope_rect() {
        let mut rect = GridRect::from_points([1, 1], [3, 3]);
        rect.envelope_rect(GridRect::from_points([4, 4], [8, 8]));
        assert_eq!([8, 8], rect.max().to_array());
        assert_eq!([1, 1], rect.min().to_array());

        rect.envelope_rect(GridRect::from_points([-10, -10], [4, 8]));
        assert_eq!([-10, -10], rect.min().to_array());
    }

    #[test]
    fn corners() {
        let rect = GridRect::from_points([0, 0], [5, 5]);
        assert_eq!([0, 0], rect.pivot_point(Pivot::BottomLeft).to_array());
        assert_eq!([0, 5], rect.pivot_point(Pivot::TopLeft).to_array());
        assert_eq!([5, 5], rect.pivot_point(Pivot::TopRight).to_array());
        assert_eq!([5, 0], rect.pivot_point(Pivot::BottomRight).to_array());

        let rect = GridRect::from_points([0, 0], [6, 6]);
        assert_eq!([0, 0], rect.pivot_point(Pivot::BottomLeft).to_array());
        assert_eq!([0, 6], rect.pivot_point(Pivot::TopLeft).to_array());
        assert_eq!([6, 6], rect.pivot_point(Pivot::TopRight).to_array());
        assert_eq!([6, 0], rect.pivot_point(Pivot::BottomRight).to_array());

        let rect = GridRect::from_points([-5, -5], [5, 5]);
        assert_eq!([-5, -5], rect.pivot_point(Pivot::BottomLeft).to_array());
        assert_eq!([-5, 5], rect.pivot_point(Pivot::TopLeft).to_array());
        assert_eq!([5, 5], rect.pivot_point(Pivot::TopRight).to_array());
        assert_eq!([5, -5], rect.pivot_point(Pivot::BottomRight).to_array());

        let rect = GridRect::from_points([-4, -4], [5, 5]);
        assert_eq!([-4, -4], rect.pivot_point(Pivot::BottomLeft).to_array());
        assert_eq!([-4, 5], rect.pivot_point(Pivot::TopLeft).to_array());
        assert_eq!([5, 5], rect.pivot_point(Pivot::TopRight).to_array());
        assert_eq!([5, -4], rect.pivot_point(Pivot::BottomRight).to_array());
    }

    #[test]
    fn iter_border() {
        let rect = GridRect::from_points([0, 0], [5, 5]);
        let points: Vec<_> = rect.iter_border().collect();

        let mut corners = rect.corners().into_iter();
        assert!(corners.all(|p| points.contains(&p)));
        assert_eq!(20, rect.iter_border().count());

        let rect = GridRect::from_points([-13, -13], [-9, -9]);
        let points: Vec<_> = rect.iter_border().collect();

        let mut corners = rect.corners().into_iter();
        assert!(corners.all(|p| points.contains(&p)));
        assert_eq!(16, rect.iter_border().count());
    }

    #[test]
    fn pivoted() {
        let rect = GridRect::center_origin([5, 5]);
        let rect = rect.pivoted(Pivot::BottomLeft);
        assert_eq!([0, 0], rect.pivot_point(Pivot::BottomLeft).to_array());

        let rect = GridRect::center_origin([5, 5]);
        let rect = rect.pivoted(Pivot::TopLeft);
        let rect = GridRect::from_pivot_pos(Pivot::TopLeft, [0, 0], [5, 5]);
        assert_eq!([0, -1], rect.pivot_point(Pivot::TopLeft).to_array());

        let rect = GridRect::center_origin([5, 5]);
        let rect = rect.pivoted(Pivot::TopRight);
        assert_eq!([-1, -1], rect.pivot_point(Pivot::TopRight).to_array());

        let rect = GridRect::center_origin([5, 5]);
        let rect = rect.pivoted(Pivot::BottomRight);
        assert_eq!([-1, 0], rect.pivot_point(Pivot::BottomRight).to_array());

        let rect = GridRect::center_origin([6, 6]);
        let rect = rect.pivoted(Pivot::BottomLeft);
        assert_eq!([0, 0], rect.pivot_point(Pivot::BottomLeft).to_array());

        let rect = GridRect::center_origin([6, 6]);
        let rect = rect.pivoted(Pivot::TopLeft);
        assert_eq!([0, -1], rect.pivot_point(Pivot::TopLeft).to_array());

        let rect = GridRect::center_origin([6, 6]);
        let rect = rect.pivoted(Pivot::TopRight);
        assert_eq!([-1, -1], rect.pivot_point(Pivot::TopRight).to_array());

        let rect = GridRect::center_origin([6, 6]);
        let rect = rect.pivoted(Pivot::BottomRight);
        assert_eq!([-1, 0], rect.pivot_point(Pivot::BottomRight).to_array());
    }

    #[test]
    #[ignore]
    fn pivot_corner_draw_bl_tr() {
        let mut canvas = Canvas::new([12, 12]);
        let a = GridRect::center_origin([5, 5]).pivoted(Pivot::BottomLeft);
        let b = GridRect::center_origin([5, 5]).pivoted(Pivot::TopRight);
        for p in a.into_iter().chain(b) {
            canvas.put(p, '*');
        }
        canvas.print();
    }

    #[test]
    #[ignore]
    fn pivot_corner_draw_tl_br() {
        let mut canvas = Canvas::new([12, 12]);
        let a = GridRect::center_origin([5, 5]).pivoted(Pivot::TopLeft);
        let b = GridRect::center_origin([5, 5]).pivoted(Pivot::BottomRight);
        for p in a.into_iter().chain(b) {
            canvas.put(p, '*');
        }
        canvas.print();
    }

    #[test]
    fn clipped() {
        let a = GridRect::from_points([5, 5], [10, 10]);
        let b = GridRect::new([7, 7], [30, 2]);
        let b = b.clipped(a);

        assert_eq!(a.min_i().x, b.min_i().x);
        assert_eq!(a.max_i().x, b.max_i().x);
    }
}
