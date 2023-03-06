//! A 2 dimensional grid that stores it's internal data in a `Vec`. The size of
//! the grid is constant and elements cannot be removed, only changed. Provides
//! very fast iteration and access speed.
//!
//! Elements can be inserted and accessed via their 1d index, 2d index, or
//! read/modified via iterators.
//!
//! # Example
//!
//! ```
//! use sark_grids::*;
//!
//! let mut grid = Grid::new([10,10]);
//!
//! grid[0] = 'a';
//! grid[ [1,0] ] = 'b';
//!
//! assert_eq!('a', grid[0]);
//! assert_eq!('b', grid[ [1,0] ]);
//!
//! grid.insert_column_at([3,2], "hello".chars());
//! let hello: String = grid.iter_column(3).skip(2).take(5).collect();
//!
//! assert_eq!("hello", hello);
//! ```

use std::ops::{Bound, Index, IndexMut, RangeBounds, Sub};

use glam::{IVec2, UVec2, Vec2};
use itertools::Itertools;

use crate::{geometry::GridRect, point::Size2d, GridPoint, Pivot};

/// A dense sized grid that stores it's elements in a `Vec`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid<T> {
    data: Vec<T>,
    size: UVec2,
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            size: Default::default(),
        }
    }
}

impl<T> Grid<T> {
    /// Creates a new [Grid<T>] with the given default value set for all elements.
    pub fn new(size: impl Size2d) -> Self
    where
        T: Default + Clone,
    {
        let size = size.as_ivec2();
        let len = (size.x * size.y) as usize;

        Self {
            data: vec![T::default(); len],
            size: size.as_uvec2(),
        }
    }

    pub fn filled(value: T, size: impl Size2d) -> Self
    where
        T: Clone,
    {
        let size = size.as_ivec2();
        let len = (size.x * size.y) as usize;

        Self {
            data: vec![value; len],
            size: size.as_uvec2(),
        }
    }

    /// Insert into a row of the grid using an iterator.
    ///
    /// Will insert up to the length of a row.
    pub fn insert_row(&mut self, y: usize, row: impl DoubleEndedIterator<Item = T>) {
        self.insert_row_at([0, y as i32], row);
    }

    /// Insert into a row of the grid using an iterator.
    ///
    /// Will insert up to the length of a row.
    pub fn insert_row_at(&mut self, xy: impl GridPoint, row: impl Iterator<Item = T>) {
        let [x, y] = xy.as_array();
        let iter = self.iter_row_mut(y as usize).skip(x as usize);
        for (v, input) in iter.zip(row) {
            *v = input;
        }
    }

    /// Insert into a column of the grid using an iterator.
    ///
    /// Will insert up to the height of a column.
    pub fn insert_column(&mut self, x: usize, column: impl IntoIterator<Item = T>) {
        self.insert_column_at([x as i32, 0], column);
    }

    /// Insert into a column of the grid using an iterator.
    ///
    /// Will insert up to the height of a column.
    pub fn insert_column_at(&mut self, xy: impl GridPoint, column: impl IntoIterator<Item = T>) {
        let [x, y] = xy.as_array();
        let iter = self.iter_column_mut(x as usize).skip(y as usize);
        for (v, input) in iter.zip(column) {
            *v = input;
        }
    }

    pub fn width(&self) -> usize {
        self.size.x as usize
    }

    pub fn height(&self) -> usize {
        self.size.y as usize
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    /// How many tiles/elements are in the grid.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get the position of the given pivot point on the grid.
    #[inline]
    pub fn pivot_position(&self, pivot: Pivot) -> IVec2 {
        let p = self.size().sub(1).as_vec2() * Vec2::from(pivot);
        p.round().as_ivec2()
    }

    /// Try to retrieve the value at the given position.
    ///
    /// Returns `None` if the position is out of bounds.
    #[inline]
    pub fn get(&self, xy: impl GridPoint) -> Option<&T> {
        if !self.in_bounds(xy) {
            return None;
        }
        let i = self.transform_lti(xy);
        Some(&self.data[i])
    }

    /// Try to retrieve the mutable value at the given position.
    ///
    /// Returns `None` if the position is out of bounds.
    pub fn get_mut(&mut self, xy: impl GridPoint) -> Option<&mut T> {
        if !self.in_bounds(xy) {
            return None;
        }
        let i = self.transform_lti(xy);
        Some(&mut self.data[i])
    }

    #[inline]
    pub fn in_bounds(&self, pos: impl GridPoint) -> bool {
        let p = pos.as_ivec2();
        !(p.cmplt(IVec2::ZERO).any() || p.cmpge(self.size.as_ivec2()).any())
    }

    /// Gets the index for a given side.
    pub fn side_index(&self, side: Side) -> usize {
        match side {
            Side::Left => 0,
            Side::Top => self.height() - 1,
            Side::Right => self.width() - 1,
            Side::Bottom => 0,
        }
    }

    // Size of the grid along a given axis, where 0 == x and 1 == y
    pub fn axis_size(&self, axis: usize) -> usize {
        match axis {
            0 => self.width(),
            1 => self.height(),
            _ => panic!("Invalid grid axis {axis}"),
        }
    }

    /// An iterator over all elements in the grid.
    #[inline]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.data.iter()
    }

    /// A mutable iterator over all elements in the grid.
    #[inline]
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut T> {
        self.data.iter_mut()
    }

    /// An iterator over a single row of the grid.
    ///
    /// Goes from left to right.
    #[inline]
    pub fn iter_row(&self, y: usize) -> impl DoubleEndedIterator<Item = &T> {
        let w = self.width();
        let i = y * w;
        self.data[i..i + w].iter()
    }

    /// A mutable iterator over a single row of the grid.
    ///
    /// Iterates from left to right.
    #[inline]
    pub fn iter_row_mut(&mut self, y: usize) -> impl DoubleEndedIterator<Item = &mut T> {
        let w = self.width();
        let i = y * w;
        self.data[i..i + w].iter_mut()
    }

    /// Iterate over a range of rows.
    ///
    /// Yields &\[T\] (Slice of T)
    pub fn iter_rows(
        &self,
        range: impl RangeBounds<usize>,
    ) -> impl DoubleEndedIterator<Item = &[T]> {
        let [start, end] = self.range_to_start_end(range, 1);
        let width = self.width();
        let x = start * width;
        let count = end.saturating_sub(start) + 1;
        let chunks = self.data[x..].chunks(width);
        chunks.take(count)
    }

    /// Iterate mutably over a range of rows.
    ///
    /// Yields &mut \[`T`\] (Slice of mutable `T`)
    pub fn iter_rows_mut(
        &mut self,
        range: impl RangeBounds<usize>,
    ) -> impl DoubleEndedIterator<Item = &mut [T]> {
        let [start, end] = self.range_to_start_end(range, 1);
        let width = self.width();
        let x = start * width;
        let count = end - start + 1;
        let chunks = self.data[x..].chunks_mut(width);
        chunks.take(count)
    }

    /// An iterator over a single column of the grid.
    ///
    /// Goes from bottom to top.
    #[inline]
    pub fn iter_column(&self, x: usize) -> impl DoubleEndedIterator<Item = &T> {
        let w = self.width();
        return self.data[x..].iter().step_by(w);
    }

    /// A mutable iterator over a single column of the grid.
    ///
    /// Goes from bottom to top.
    #[inline]
    pub fn iter_column_mut(&mut self, x: usize) -> impl DoubleEndedIterator<Item = &mut T> {
        let w = self.width();
        return self.data[x..].iter_mut().step_by(w);
    }

    /// Final index along a given axis, where 0 == width, and 1 == height.
    pub fn axis_index(&self, axis: usize) -> usize {
        match axis {
            0 => self.side_index(Side::Right),
            1 => self.side_index(Side::Top),
            _ => panic!("Invalid grid axis {axis}"),
        }
    }

    /// An iterator over a rectangular portion of the grid defined by the given range.
    ///
    /// Yields `(IVec2, &T)`, where `IVec2` is the corresponding position of the value in the grid.
    pub fn rect_iter(
        &self,
        range: impl RangeBounds<[i32; 2]>,
    ) -> impl Iterator<Item = (IVec2, &T)> {
        let (min, max) = ranges_to_min_max(range, self.size().as_ivec2());
        (min.y..=max.y)
            .cartesian_product(min.x..=max.x)
            .map(|(y, x)| {
                let i = self.transform_lti([x, y]);
                ((IVec2::new(x, y)), &self.data[i])
            })
    }

    /// Returns an iterator which enumerates the 2d position of every value in the grid.
    ///
    /// Yields `(IVec2, &T)`, where `IVec2` is the corresponding position of the value in the grid.
    pub fn iter_2d(&self) -> impl Iterator<Item = (IVec2, &T)> {
        (0..self.height())
            .cartesian_product(0..self.width())
            .map(|(y, x)| IVec2::new(x as i32, y as i32))
            .zip(self.data.iter())
    }

    /// Returns a mutable iterator which enumerates the 2d position of every value in the grid.
    ///
    /// Yields `(IVec2, &mut T)`, where `IVec2` is the corresponding position of the value in the grid.
    pub fn iter_2d_mut(&mut self) -> impl Iterator<Item = (IVec2, &mut T)> {
        //let offset = self.pivot_offset;
        (0..self.height())
            .cartesian_product(0..self.width())
            .map(move |(y, x)| (Vec2::from([x as f32, y as f32])).as_ivec2())
            .zip(self.data.iter_mut())
    }

    /// Retrieve a linear slice of the underlying grid data.
    pub fn slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Retrieve a mutable linear slice of the underlying grid data.
    pub fn slice_mut(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    /// Convert a range into a [start,end] pair.
    ///
    /// An unbounded "end" returned by this function should be treated as EXCLUSIVE.
    fn range_to_start_end(&self, range: impl RangeBounds<usize>, axis: usize) -> [usize; 2] {
        let start = match range.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(end) => *end,
            Bound::Excluded(end) => *end - 1,
            Bound::Unbounded => self.axis_size(axis),
        };

        [start, end]
    }

    /// Returns the bounds of the grid.
    #[inline]
    pub fn bounds(&self) -> GridRect {
        GridRect::from_bl([0, 0], self.size)
    }

    /// Converts a 2d grid position to it's corresponding 1D index. If a pivot
    /// was applied to the given grid point, it will be accounted for.
    #[inline(always)]
    pub fn transform_lti(&self, xy: impl GridPoint) -> usize {
        let xy = self.pivoted_point(xy);
        let [x, y] = xy.to_array();
        y as usize * self.width() + x as usize
    }

    /// Converts a 1d index to it's corresponding grid position.
    #[inline(always)]
    pub fn transform_itl(&self, index: usize) -> IVec2 {
        let index = index as i32;
        let w = self.width() as i32;
        let x = index % w;
        let y = index / w;
        IVec2::new(x, y)
    }

    /// Convert a point from grid space (bottom-left origin) to world space
    /// (center origin).
    #[inline(always)]
    pub fn transform_ltw(&self, xy: impl GridPoint) -> IVec2 {
        xy.as_ivec2() - self.size.as_ivec2() / 2
    }

    /// Convert a point from world space (center origin) to grid space
    /// (bottom-left origin).
    #[inline(always)]
    pub fn transform_wtl(&self, xy: impl GridPoint) -> IVec2 {
        xy.as_ivec2() + self.size.as_ivec2() / 2
    }

    /// Retrieve a grid position from a pivoted point.
    ///
    /// If no pivot has been applied, the point will be returned directly.
    #[inline]
    pub fn pivoted_point(&self, xy: impl GridPoint) -> IVec2 {
        if let Some(pivot) = xy.get_pivot() {
            pivot.transform_point(xy, self.size)
        } else {
            xy.as_ivec2()
        }
    }
}

fn ranges_to_min_max(range: impl RangeBounds<[i32; 2]>, max: IVec2) -> (IVec2, IVec2) {
    let min = match range.start_bound() {
        std::ops::Bound::Included([x, y]) => IVec2::new(*x, *y),
        std::ops::Bound::Excluded([x, y]) => IVec2::new(*x, *y),
        std::ops::Bound::Unbounded => IVec2::ZERO,
    };

    let max = match range.end_bound() {
        std::ops::Bound::Included([x, y]) => IVec2::new(*x, *y),
        std::ops::Bound::Excluded([x, y]) => IVec2::new(x - 1, y - 1),
        std::ops::Bound::Unbounded => max,
    };

    debug_assert!(min.cmpge(IVec2::ZERO).all() && min.cmplt(max).all());
    debug_assert!(max.cmple(max).all());

    (min, max)
}

impl<T: Clone, P: GridPoint> Index<P> for Grid<T> {
    type Output = T;

    fn index(&self, p: P) -> &Self::Output {
        let i = self.transform_lti(p);
        &self.data[i]
    }
}

impl<T: Clone, P: GridPoint> IndexMut<P> for Grid<T>
where
    T: Default,
{
    fn index_mut(&mut self, index: P) -> &mut T {
        let xy = index.as_ivec2();
        let i = self.transform_lti(xy);
        &mut self.data[i]
    }
}

impl<T: Clone> Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
impl<T: Clone> IndexMut<usize> for Grid<T>
where
    T: Default,
{
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Side {
    Left,
    Top,
    Right,
    Bottom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_convert() {
        let grid = Grid::filled(0, [5, 11]);
        let [start, end] = grid.range_to_start_end(.., 0);
        assert_eq!([start, end], [0, 5]);
        let [start, count] = grid.range_to_start_end(5..=10, 0);
        assert_eq!([start, count], [5, 10]);
        let [start, count] = grid.range_to_start_end(3..11, 0);
        assert_eq!([start, count], [3, 10]);
    }

    #[test]
    fn rows_iter() {
        let mut grid = Grid::new([3, 10]);
        grid.insert_row(3, std::iter::repeat(5));
        grid.insert_row(4, std::iter::repeat(6));

        let mut iter = grid.iter_rows(3..=4);

        assert!(iter.next().unwrap().iter().all(|v| *v == 5));
        assert!(iter.next().unwrap().iter().all(|v| *v == 6));
    }

    #[test]
    fn rows_iter_mut() {
        let mut grid = Grid::new([3, 4]);
        for row in grid.iter_rows_mut(..) {
            row.iter_mut().for_each(|v| *v = 5);
        }

        assert!(grid.iter().all(|v| *v == 5));
    }

    #[test]
    fn row_iter() {
        let mut grid = Grid::new([10, 15]);

        let chars = "hello".chars();

        for (elem, ch) in grid.iter_row_mut(3).take(5).zip(chars) {
            *elem = ch;
        }

        let hello = grid.iter_row(3).take(5).collect::<String>();

        assert_eq!("hello", hello);

        assert_eq!(grid.iter_row(6).count(), 10);
    }

    #[test]
    fn column_iter() {
        let mut grid = Grid::new([10, 15]);

        let chars = ['h', 'e', 'l', 'l', 'o'];

        for (elem, ch) in grid.iter_column_mut(5).take(5).zip(chars) {
            *elem = ch;
        }

        let hello = grid.iter_column(5).take(5).collect::<String>();

        assert_eq!("hello", hello);

        assert_eq!(grid.iter_column(2).count(), 15);
    }

    #[test]
    fn iter_2d() {
        let mut grid = Grid::new([5, 3]);
        grid[[0, 0]] = 5;
        grid[[3, 1]] = 10;
        grid[[4, 2]] = 20;

        let vec: Vec<_> = grid.iter_2d().collect();

        assert_eq!(vec.len(), 5 * 3);
        assert_eq!(*vec[grid.transform_lti([0, 0])].1, 5);
        assert_eq!(*vec[grid.transform_lti([3, 1])].1, 10);
        assert_eq!(*vec[grid.transform_lti([4, 2])].1, 20);

        let mut iter = grid.iter_2d();
        let (p, _) = iter.next().unwrap();
        assert_eq!(0, p.x);
        assert_eq!(0, p.y);
        let (p, _) = iter.next().unwrap();
        assert_eq!(1, p.x);
        assert_eq!(0, p.y);

        let (p, _) = iter.nth(3).unwrap();
        assert_eq!(0, p.x);
        assert_eq!(1, p.y);
    }

    #[test]
    fn iter() {
        let grid = Grid::<i32>::filled(5, [10, 10]);

        let v: Vec<_> = grid.iter().collect();

        assert_eq!(v.len(), 100);
        assert_eq!(*v[0], 5);
        assert_eq!(*v[99], 5);
    }

    #[test]
    fn iter_mut() {
        let mut grid = Grid::new([10, 10]);

        for i in grid.iter_mut() {
            *i = 10;
        }

        assert_eq!(grid[0], 10);
    }

    #[test]
    fn rect_iter() {
        let mut grid = Grid::new([11, 15]);

        grid[[2, 2]] = 5;
        grid[[4, 4]] = 10;

        let iter = grid.rect_iter([2, 2]..=[4, 4]);
        let vec: Vec<_> = iter.collect();

        assert_eq!(vec.len(), 9);
        assert_eq!(*vec[0].1, 5);
        assert_eq!(*vec[8].1, 10);

        let mut iter = grid.rect_iter([2, 2]..=[4, 4]);

        let (p, _) = iter.next().unwrap();
        assert_eq!(p, IVec2::new(2, 2));
        assert_eq!(iter.nth(7).unwrap().0, IVec2::new(4, 4));
    }

    #[test]
    fn column_insert() {
        let mut grid = Grid::new([10, 10]);

        grid.insert_column(3, "Hello".chars());

        let hello: String = grid.iter_column(3).take(5).collect();

        assert_eq!(hello, "Hello");
    }

    #[test]
    fn row_insert() {
        let mut grid = Grid::new([10, 10]);

        grid.insert_row(3, "Hello".chars());

        let hello: String = grid.iter_row(3).take(5).collect();

        assert_eq!(hello, "Hello");
    }

    #[test]
    fn ltw() {
        let grid = Grid::filled(0, [10, 10]);
        assert_eq!([5, 5], grid.transform_wtl([0, 0]).to_array());
        assert_eq!([0, 0], grid.transform_ltw([5, 5]).to_array());

        let grid = Grid::filled(0, [9, 9]);
        assert_eq!([4, 4], grid.transform_wtl([0, 0]).to_array());
        assert_eq!([0, 0], grid.transform_ltw([4, 4]).to_array());
    }
}
