use std::{
    fmt::{self, Display},
    ops::{Add, Deref, Sub},
};

use glam::IVec2;

use super::GridShape;
use crate::{GridPoint, Pivot};

/// A rectangle of points on a 2d grid.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridRect {
    /// The bottom-left most tile of the rect.
    pub xy: IVec2,
    pub size: IVec2,
}

impl GridRect {
    /// Create a [GridRect] from a position (bottom left tile) and a size.
    pub fn new(xy: impl Into<IVec2>, size: impl GridPoint) -> Self {
        GridRect {
            xy: xy.into(),
            size: size.as_ivec2(),
        }
    }

    /// Create a [GridRect] with it's center at origin (`[0,0]`).
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

    /// Creates a [GridRect] from the given pivot position and size.
    pub fn from_pivot_pos(pivot: Pivot, pos: impl Into<IVec2>, size: impl Into<IVec2>) -> Self {
        let xy = pos.into();
        let size = size.into();

        let bl = xy - (size.sub(1).as_vec2() * pivot.normalized()).as_ivec2();
        let tr = bl + size.sub(1);
        Self::from_points(bl, tr)
    }

    /// Returns a [GridRect] clipped by the bounds of the given [GridRect]
    pub fn clipped(&self, clipper: GridRect) -> GridRect {
        let [bmin, bmax] = [clipper.min(), clipper.max()];
        let [amin, amax] = [self.min(), self.max()];
        let max = amax.min(bmax);
        let min = amin.max(bmin);
        GridRect::from_points(min, max)
    }

    /// Returns a [GridRect] with it's position adjusted by the given amount
    pub fn translated(&self, xy: impl Into<IVec2>) -> GridRect {
        GridRect::new(xy, self.size)
    }

    /// Returns a [GridRect] of the same size, adjusted to the given pivot.
    pub fn pivoted(&self, pivot: Pivot) -> GridRect {
        Self::from_pivot_pos(pivot, self.xy, self.size)
    }

    /// Returns a [GridRect] with both rects contained in it.
    pub fn merged(&self, mut other: GridRect) -> GridRect {
        let [min, max] = [self.min(), self.max()];
        other.envelope_point(min);
        other.envelope_point(max);
        other
    }

    /// Adjusts a single corner of the rect to contain the given point.
    pub fn envelope_point(&mut self, point: impl Into<IVec2>) {
        let point = point.into();
        let min = self.min().min(point);
        let max = self.max().max(point);
        *self = GridRect::from_points(min, max);
    }

    /// Adjust this rect so the given rect is entirely contained within it.
    pub fn merge(&mut self, rect: GridRect) {
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

    /// Index of the bottom row of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    pub fn bottom(&self) -> usize {
        0
    }

    /// Index of the top row of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    pub fn top(&self) -> usize {
        self.height().sub(1)
    }

    /// Index of the left column of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    pub fn left(&self) -> usize {
        0
    }

    /// The index of the right most column of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
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

    /// Retrieve the position of the tile at the given pivot point on the rect.
    pub fn pivot_point(&self, pivot: Pivot) -> IVec2 {
        self.bottom_left() + pivot.size_offset(self.size)
    }

    /// Retrieve a point in the rect from the perspective of the given pivot.
    pub fn pivoted_point(&self, pivot: Pivot, point: impl Into<IVec2>) -> IVec2 {
        let origin = self.pivot_point(pivot);
        origin + (point.into() * pivot.axis())
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

    /// Check if any part of a rect overlaps another.
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

impl Display for GridRect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GridRect {{
    MinMax {:?}, {:?}
    Center {:?}
    Size   {:?}
}}",
            self.min().to_array(),
            self.max().to_array(),
            self.center().to_array(),
            self.size.to_array()
        )
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

impl GridShape for GridRect {
    fn iter(&self) -> super::GridShapeIterator {
        super::GridShapeIterator::Rect(self.into_iter())
    }

    fn pos(&self) -> IVec2 {
        self.xy
    }

    fn set_pos(&mut self, pos: IVec2) {
        self.xy = pos;
    }

    fn rect(&self) -> GridRect {
        *self
    }
}

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
        rect.merge(GridRect::from_points([4, 4], [8, 8]));
        assert_eq!([8, 8], rect.max().to_array());
        assert_eq!([1, 1], rect.min().to_array());

        rect.merge(GridRect::from_points([-10, -10], [4, 8]));
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
    #[ignore]
    fn pivot_corner_draw_bl_tr() {
        let mut canvas = Canvas::new([12, 12]);
        let a = GridRect::from_pivot_pos(Pivot::BottomLeft, [0, 0], [5, 5]);
        let b = GridRect::from_pivot_pos(Pivot::TopRight, [0, 0], [5, 5]);
        // let a = GridRect::center_origin([5, 5]).pivoted(Pivot::BottomLeft);
        // let b = GridRect::center_origin([5, 5]).pivoted(Pivot::TopRight);
        for p in a.into_iter().chain(b) {
            canvas.put(p, '*');
        }
        canvas.print();
    }

    // #[test]
    // #[ignore]
    // fn pivot_corner_draw_tl_br() {
    //     let mut canvas = Canvas::new([12, 12]);
    //     let a = GridRect::center_origin([5, 5]).pivoted(Pivot::TopLeft);
    //     let b = GridRect::center_origin([5, 5]).pivoted(Pivot::BottomRight);
    //     for p in a.into_iter().chain(b) {
    //         canvas.put(p, '*');
    //     }
    //     canvas.print();
    // }

    // #[test]
    // fn clipped() {
    //     let a = GridRect::from_points([5, 5], [10, 10]);
    //     let b = GridRect::new([7, 7], [30, 2]);
    //     let b = b.clipped(a);

    //     assert_eq!(a.min_i().x, b.min_i().x);
    //     assert_eq!(a.max_i().x, b.max_i().x);
    // }
}
