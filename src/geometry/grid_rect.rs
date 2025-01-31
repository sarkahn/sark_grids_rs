use std::ops::Sub;

use glam::{ivec2, IVec2, UVec2};

use super::GridShape;
use crate::{GridPoint, GridSize, Pivot, PivotedPoint};

/// A rectangle of points on a 2d grid.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridRect {
    /// The bottom-left most tile of the rect.
    pub pos: IVec2,
    pub size: UVec2,
}

impl GridRect {
    /// Create a [GridRect] from a position (bottom left tile) and a size.
    pub fn new(pos: impl GridPoint, size: impl GridSize) -> Self {
        GridRect {
            pos: pos.to_ivec2(),
            size: size.to_uvec2(),
        }
    }

    /// Create a [GridRect] with it's center at origin (`[0,0]`).
    pub fn center_origin(size: impl GridSize) -> Self {
        Self::from_center_size([0, 0], size)
    }

    /// Create a [GridRect] from two points in space.
    pub fn from_points(a: impl GridPoint, b: impl GridPoint) -> Self {
        let min = a.min(b);
        let max = a.max(b);

        let size = (max - min) + 1;
        GridRect {
            pos: min,
            size: size.as_uvec2(),
        }
    }

    /// Create a [GridRect] from a center position and rect size.
    pub fn from_center_size(center: impl GridPoint, size: impl GridSize) -> Self {
        let bl = center.to_ivec2() - size.to_ivec2() / 2;
        Self::new(bl, size.to_uvec2())
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
    pub fn translated(&self, pos: impl GridPoint) -> GridRect {
        GridRect::new(self.pos + pos.to_ivec2(), self.size)
    }

    /// Returns a [GridRect] with each side adjusted by the given delta.
    pub fn resized(&self, delta: impl GridPoint) -> GridRect {
        GridRect::from_points(self.min() - delta.to_ivec2(), self.max() + delta.to_ivec2())
    }

    /// Resizes the rect along a given pivot point.
    pub fn resize_from_pivot(&mut self, pivot: Pivot, amount: i32) {
        let p = match pivot {
            Pivot::TopLeft => self.top_left() + ivec2(-1, 1) * amount,
            Pivot::TopCenter => self.top_left() + ivec2(0, 1) * amount,
            Pivot::TopRight => self.top_right() + ivec2(1, 1) * amount,
            Pivot::LeftCenter => self.top_left() + ivec2(-1, 0) * amount,
            Pivot::RightCenter => self.top_right() + ivec2(1, 0) * amount,
            Pivot::BottomLeft => self.bottom_left() + ivec2(-1, -1) * amount,
            Pivot::BottomCenter => self.bottom_left() + ivec2(0, -1) * amount,
            Pivot::BottomRight => self.bottom_right() + ivec2(1, -1) * amount,
            Pivot::Center => self.center(),
        };
        self.envelope_point(p);
    }

    /// Returns a [GridRect] with both rects contained in it.
    pub fn merged(&self, mut other: GridRect) -> GridRect {
        other.envelope_point(self.min());
        other.envelope_point(self.max());
        other
    }

    /// Adjusts a single corner of the rect to contain the given point.
    pub fn envelope_point(&mut self, point: impl GridPoint) {
        let point = point.to_ivec2();
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

    /// The center position of the rect.
    pub fn center(&self) -> IVec2 {
        self.pos + self.size.as_ivec2() / 2
    }

    /// The y position of the top row of the rect.
    pub fn top(&self) -> i32 {
        self.max().y
    }

    /// The y position of the bottom row of the rect.
    pub fn bottom(&self) -> i32 {
        self.min().y
    }

    /// The x position of the left column of the rect.
    pub fn left(&self) -> i32 {
        self.min().x
    }

    /// The x position of the right column of the rect.
    pub fn right(&self) -> i32 {
        self.max().x
    }

    /// The 2d position of the top left tile of the rect.
    pub fn top_left(&self) -> IVec2 {
        [self.left(), self.top()].into()
    }

    /// The 2d position of the top right tile of the rect.
    pub fn top_right(&self) -> IVec2 {
        self.max()
    }

    /// The 2d position of the bottom left tile of the rect.
    pub fn bottom_left(&self) -> IVec2 {
        self.pos
    }

    /// The 2d position of the bottom right tile of the rect.
    pub fn bottom_right(&self) -> IVec2 {
        [self.right(), self.bottom()].into()
    }

    pub fn width(&self) -> usize {
        self.size.x as usize
    }

    pub fn height(&self) -> usize {
        self.size.y as usize
    }

    /// Bottom left position of the rect.
    pub fn min(&self) -> IVec2 {
        self.pos
    }

    /// Top right position of the rect.
    pub fn max(&self) -> IVec2 {
        self.pos + self.size.as_ivec2().sub(1)
    }

    /// Index of the bottom row of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    pub fn bottom_index(&self) -> usize {
        0
    }

    /// Index of the top row of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    pub fn top_index(&self) -> usize {
        self.height().sub(1)
    }

    /// Index of the left column of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    pub fn left_index(&self) -> usize {
        0
    }

    /// The index of the right most column of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    pub fn right_index(&self) -> usize {
        self.width().sub(1)
    }

    /// Iterate over each point of the rect.
    pub fn iter_points(&self) -> impl DoubleEndedIterator<Item = IVec2> + ExactSizeIterator {
        GridRectIter::new(*self)
    }

    /// Iterate over the tile positions of a single column of the rect.
    pub fn iter_column(
        &self,
        col: usize,
    ) -> impl DoubleEndedIterator<Item = IVec2> + ExactSizeIterator {
        self.iter_points().skip(col).step_by(self.width())
    }

    /// Iterate over the tile positions of a single row of the rect.
    pub fn iter_row(
        &self,
        row: usize,
    ) -> impl DoubleEndedIterator<Item = IVec2> + ExactSizeIterator {
        self.iter_points()
            .skip(row * self.width())
            .take(self.width())
    }

    /// Iterate over all the tile positions of the border of the rect in clockwise
    /// order, starting from the bottom left.
    pub fn iter_border(&self) -> impl DoubleEndedIterator<Item = IVec2> {
        let left = self.iter_column(0);
        let top = self
            .iter_row(self.top_index())
            .skip(1)
            .take(self.width().sub(2));
        let right = self.iter_column(self.right_index()).rev();
        let bottom = self
            .iter_row(self.bottom_index())
            .rev()
            .skip(1)
            .take(self.width().sub(2));
        left.chain(top).chain(right).chain(bottom)
    }

    /// Retrieve the position of the tile at the given pivot point on the rect.
    pub fn pivot_point(&self, pivot: Pivot) -> IVec2 {
        self.bottom_left() + pivot.pivot_position(self.size)
    }

    /// Retrieve a point in the rect from the perspective of the given pivot.
    pub fn pivoted_point(&self, pivot: Pivot, point: impl Into<IVec2>) -> IVec2 {
        let origin = self.pivot_point(pivot);
        origin + (point.into() * pivot.axis())
    }

    /// Check if a given point lies inside the rect
    #[inline]
    pub fn contains_point(&self, p: impl GridPoint) -> bool {
        let p = p.to_ivec2();
        !(p.cmplt(self.min()).any() || p.cmpgt(self.max()).any())
    }

    /// Returns true if the given rect is entirely contained within this one.
    #[inline]
    pub fn contains_rect(&self, other: GridRect) -> bool {
        other.min().cmpge(self.min()).all() && other.max().cmple(self.max()).all()
    }

    /// Check if any part of a rect overlaps another.
    #[inline]
    pub fn overlaps_rect(&self, other: GridRect) -> bool {
        self.left() <= other.right()
            && other.left() <= self.right()
            && self.bottom() <= other.top()
            && other.bottom() <= self.top()
    }
}

impl SizedGrid for GridRect {
    fn size(&self) -> glam::UVec2 {
        self.size
    }
}

impl PositionedGrid for GridRect {
    fn pos(&self) -> IVec2 {
        self.pos
    }
}

/// An iterator over the 2d grid points of a [GridRect].
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridRectIter {
    origin: IVec2,
    size: UVec2,
    head: IVec2,
    tail: IVec2,
}

impl GridRectIter {
    pub fn new(rect: GridRect) -> Self {
        Self {
            origin: rect.pos,
            size: rect.size,
            head: IVec2::ZERO,
            tail: rect.size.as_ivec2().sub(1),
        }
    }

    pub fn can_iterate(&self) -> bool {
        self.head.y < self.tail.y || (self.head.y == self.tail.y && self.head.x <= self.tail.x)
    }
}

impl Iterator for GridRectIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.can_iterate() {
            return None;
        }
        let size = self.size.as_ivec2();
        let head = &mut self.head;

        let ret = self.origin + *head;
        head.x += 1;
        if head.x >= size.x {
            head.x = 0;
            head.y += 1;
        }

        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if !self.can_iterate() {
            return (0, Some(0));
        }
        let count = self
            .tail
            .as_index(self.size)
            .saturating_sub(self.head.as_index(self.size))
            + 1;
        (count, Some(count))
    }
}

impl DoubleEndedIterator for GridRectIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let size = self.size.as_ivec2();
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

impl std::fmt::Display for GridRect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

// /// The corner points of a [GridRect]. Corners can be accessed by name, index,
// /// or iterated over.
// #[derive(Default, Debug, Clone, Copy)]
// pub struct Corners(pub [IVec2; 4]);

// impl Corners {
//     pub fn bottom_left(&self) -> IVec2 {
//         self.0[0]
//     }
//     pub fn top_left(&self) -> IVec2 {
//         self.0[1]
//     }
//     pub fn top_right(&self) -> IVec2 {
//         self.0[2]
//     }
//     pub fn bottom_right(&self) -> IVec2 {
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

impl GridShape for GridRect {
    fn iter(&self) -> super::GridShapeIterator {
        super::GridShapeIterator::Rect(self.into_iter())
    }

    fn pos(&self) -> IVec2 {
        self.pos
    }

    fn set_pos(&mut self, pos: IVec2) {
        self.pos = pos;
    }

    fn bounds(&self) -> GridRect {
        *self
    }
}

/// A rectangular grid with a defined size.
pub trait SizedGrid {
    fn size(&self) -> UVec2;

    fn width(&self) -> usize {
        self.size().x as usize
    }

    fn height(&self) -> usize {
        self.size().y as usize
    }

    fn tile_count(&self) -> usize {
        self.width() * self.height()
    }

    fn in_bounds(&self, p: impl Into<PivotedPoint>) -> bool {
        let p: IVec2 = p.into().calculate(self.size());
        p.cmpge(IVec2::ZERO).all() && p.cmplt(self.size().as_ivec2()).all()
    }

    /// Transform a local 2d grid position to a 1d array index.
    #[inline]
    fn transform_lti(&self, pos: impl Into<PivotedPoint>) -> usize {
        let pos: IVec2 = pos.into().calculate(self.size());
        debug_assert!(
            self.in_bounds(pos),
            "Attempting to create 1d index from out of bounds position {} in grid sized {}",
            pos,
            self.size()
        );
        pos.as_index(self.size())
    }

    /// Transform an 1d array index to a local 2d grid position.
    fn transform_itl(&self, i: usize) -> IVec2 {
        let x = i % self.width();
        let y = i / self.width();
        IVec2::new(x as i32, y as i32)
    }

    /// Attempt to transform a 2d grid position to a 1d array index, returning
    /// [None] if the position is out of bounds.
    fn try_transform_lti(&self, pos: impl Into<PivotedPoint>) -> Option<usize> {
        let pos: IVec2 = pos.into().calculate(self.size());
        pos.get_index(self.size())
    }

    /// Attempt to transform an 1d array index to a local 2d grid position.
    /// Returns [None] if the index is out of bounds.
    fn try_transform_itl(&self, i: usize) -> Option<IVec2> {
        if i >= self.tile_count() {
            return None;
        }
        let x = i % self.width();
        let y = i / self.width();
        Some(IVec2::new(x as i32, y as i32))
    }

    /// Index of the bottom row of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    fn bottom_index(&self) -> usize {
        0
    }

    /// Index of the top row of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    fn top_index(&self) -> usize {
        self.height().sub(1)
    }

    /// Index of the left column of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    fn left_index(&self) -> usize {
        0
    }

    /// The index of the right most column of the rect.
    ///
    /// The "index" is independent of the rect's position and goes from
    /// `0` to `size-1`.
    fn right_index(&self) -> usize {
        self.width().sub(1)
    }

    fn grid_bounds(&self) -> GridRect {
        GridRect::new([0, 0], self.size())
    }

    fn iter_grid_points(&self) -> GridRectIter {
        GridRectIter::new(self.grid_bounds())
    }
}

/// A rectangular grid with a defined size and world position.
pub trait PositionedGrid: SizedGrid {
    /// The position (bottom-left tile) of the grid in world space.
    fn pos(&self) -> IVec2;

    /// The center tile of the grid.
    fn center(&self) -> IVec2 {
        self.pos() + self.size().as_ivec2() / 2
    }

    /// The x position of the right-most grid column.
    fn right(&self) -> i32 {
        self.pos().x + self.width() as i32 - 1
    }

    /// The y position of the top grid row.
    fn top(&self) -> i32 {
        self.pos().y + self.height() as i32 - 1
    }

    /// The x position of the left-most grid column.
    fn left(&self) -> i32 {
        self.pos().x
    }

    /// The y position of the bottom grid row.
    fn bottom(&self) -> i32 {
        self.pos().y
    }

    /// The grid position of the top-left grid tile.
    fn top_left(&self) -> IVec2 {
        IVec2::new(self.left(), self.top())
    }

    /// The grid position of the top-right grid tile.
    fn top_right(&self) -> IVec2 {
        IVec2::new(self.right(), self.top())
    }

    /// The grid position of the bottom-left grid tile.
    fn bottom_left(&self) -> IVec2 {
        IVec2::new(self.left(), self.bottom())
    }

    /// The grid position of the bottom-right grid tile.
    fn bottom_right(&self) -> IVec2 {
        IVec2::new(self.right(), self.bottom())
    }

    /// The grid position of the bottom-left grid tile.
    fn min(&self) -> IVec2 {
        self.pos()
    }

    /// The grid position of the top-right grid tile.
    fn max(&self) -> IVec2 {
        self.pos() + self.size().as_ivec2() - 1
    }

    fn contains_point(&self, p: impl GridPoint) -> bool {
        let p = p.to_ivec2();
        !(p.cmplt(self.min()).any() || p.cmpgt(self.max()).any())
    }

    /// Check if another [PositionedGrid] is entirely contained within this one.
    fn contains_rect(&self, other: impl PositionedGrid) -> bool {
        other.min().cmpge(self.min()).all() && other.max().cmple(self.max()).all()
    }

    /// Check if any tile of this [PositionedGrid] overlaps with another.
    fn overlaps(&self, other: impl PositionedGrid) -> bool {
        self.left() <= other.right()
            && other.left() <= self.right()
            && self.bottom() <= other.top()
            && other.bottom() <= self.top()
    }

    /// Iterate over each point of the [GridRect].
    fn iter_rect_points(&self) -> GridRectIter {
        GridRectIter::new(GridRect::new(self.pos(), self.size()))
    }

    /// Iterate over the grid points of a single column of the rect.
    fn iter_rect_column(
        &self,
        col: usize,
    ) -> impl DoubleEndedIterator<Item = IVec2> + ExactSizeIterator {
        self.iter_grid_points().skip(col).step_by(self.width())
    }

    /// Iterate over a the grid points of single row.
    fn iter_rect_row(
        &self,
        row: usize,
    ) -> impl DoubleEndedIterator<Item = IVec2> + ExactSizeIterator {
        self.iter_grid_points()
            .skip(row * self.width())
            .take(self.width())
    }

    /// Iterate over all the border tiles of the grid in clockwise order,
    /// starting from the bottom left.
    fn iter_rect_border(&self) -> impl DoubleEndedIterator<Item = IVec2> {
        let left = self.iter_rect_column(0);
        let top = self
            .iter_rect_row(self.top_index())
            .skip(1)
            .take(self.width().sub(2));
        let right = self.iter_rect_column(self.right_index()).rev();
        let bottom = self
            .iter_rect_row(self.bottom_index())
            .rev()
            .skip(1)
            .take(self.width().sub(2));
        left.chain(top).chain(right).chain(bottom)
    }

    /// Retrieve the position of the tile at the given pivot point on the grid.
    fn pivot_point(&self, pivot: Pivot) -> IVec2 {
        self.bottom_left() + pivot.pivot_position(self.size())
    }

    /// Retrieve a point in the grid from the perspective of the given pivot.
    fn pivoted_point(&self, pivot: Pivot, point: impl GridPoint) -> IVec2 {
        let origin = self.pivot_point(pivot);
        origin + (point.to_ivec2() * pivot.axis())
    }

    /// Transform a world point to this grid's local coordinates
    fn transform_wtl(&self, pos: impl GridPoint) -> IVec2 {
        pos.to_ivec2() - self.min()
    }
}

#[cfg(test)]
mod tests {
    use crate::{geometry::grid_rect::PositionedGrid, util::Canvas, Pivot};

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
        assert!(rect.contains_point([-2, -2]));
        assert!(rect.contains_point([2, 2]));
        assert!(!rect.contains_point([3, 3]));
        assert!(!rect.contains_point([-3, -3]));
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
    fn pivot_corners() {
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

    // #[test]
    // fn iter_border() {
    //     let rect = GridRect::from_points([0, 0], [5, 5]);
    //     let points: Vec<_> = rect.iter_rect_border().collect();

    //     let mut corners = rect.corners().into_iter();
    //     assert!(corners.all(|p| points.contains(&p)));
    //     assert_eq!(20, rect.iter_rect_border().count());

    //     let rect = GridRect::from_points([-13, -13], [-9, -9]);
    //     let points: Vec<_> = rect.iter_rect_border().collect();

    //     let mut corners = rect.corners().into_iter();
    //     assert!(corners.all(|p| points.contains(&p)));
    //     assert_eq!(16, rect.iter_rect_border().count());
    // }

    // #[test]
    // fn pivot_doesnt_overlap() {
    //     let a = GridRect::from_pivot_point([5, 5], Pivot::TopRight);
    //     let rects = [
    //         GridRect::from_pivot_origin([3, 3], Pivot::BottomLeft),
    //         GridRect::from_pivot_origin([7, 7], Pivot::BottomRight),
    //         GridRect::from_pivot_origin([2, 2], Pivot::TopLeft),
    //     ];
    //     let overlap = rects.iter().any(|r| r.overlaps(a));
    //     assert!(!overlap);
    // }

    #[test]
    fn clipped() {
        let a = GridRect::from_points([4, 5], [10, 13]);
        let b = GridRect::from_points([9, 10], [13, 15]);
        let c = GridRect::from_points([1, 2], [7, 10]);

        let clipped = b.clipped(a);
        assert_eq!([9, 10], clipped.min().to_array());
        assert_eq!([10, 13], clipped.max().to_array());

        let clipped = a.clipped(c);
        assert_eq!([4, 5], clipped.min().to_array());
        assert_eq!([7, 10], clipped.max().to_array());
    }
}
