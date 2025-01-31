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

use std::ops::{Index, IndexMut};

use glam::{IVec2, UVec2};

use crate::{geometry::GridRect, GridPoint, PositionedGrid, SizedGrid};

/// A dense [SizedGrid] that stores it's elements in a [Vec].
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
    pub fn new(size: impl GridPoint) -> Self
    where
        T: Default + Clone,
    {
        let size = size.to_ivec2();
        let len = (size.x * size.y) as usize;

        Self {
            data: vec![T::default(); len],
            size: size.as_uvec2(),
        }
    }

    /// Creates a new [Grid] with all elements set to the given default value.
    pub fn filled(value: T, size: impl GridPoint) -> Self
    where
        T: Clone,
    {
        let size = size.to_ivec2();
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
        let [x, y] = xy.to_array();
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
        let [x, y] = xy.to_array();
        let iter = self.iter_column_mut(x as usize).skip(y as usize);
        for (v, input) in iter.zip(column) {
            *v = input;
        }
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

    /// An iterator over a single column of the grid.
    ///
    /// Goes from bottom to top.
    #[inline]
    pub fn iter_column(&self, x: usize) -> impl DoubleEndedIterator<Item = &T> {
        let w = self.width();
        self.data[x..].iter().step_by(w)
    }

    /// A mutable iterator over a single column of the grid.
    ///
    /// Goes from bottom to top.
    #[inline]
    pub fn iter_column_mut(&mut self, x: usize) -> impl DoubleEndedIterator<Item = &mut T> {
        let w = self.width();
        self.data[x..].iter_mut().step_by(w)
    }

    /// Iterate over all grid elements along with their 2d positions.
    pub fn iter_xy(&self) -> impl Iterator<Item = (IVec2, &T)> {
        self.iter_rect(self.bounds())
    }

    /// Iterate over all grid elements along with their 2d positions.
    pub fn iter_xy_mut(&mut self) -> impl Iterator<Item = (IVec2, &mut T)> {
        let bounds = self.bounds();
        self.iter_rect_mut(bounds)
    }

    /// Iterate over a rectangular section of grid elements along with their 2d positions.
    pub fn iter_rect(&self, rect: GridRect) -> impl Iterator<Item = (IVec2, &T)> {
        rect.iter_rect_points().map(|p| {
            let i = self.transform_lti(p);
            (p, &self.data[i])
        })
    }

    /// Iterate over a rectangular section of grid elements along with their 2d positions.
    pub fn iter_rect_mut(&mut self, rect: GridRect) -> impl Iterator<Item = (IVec2, &mut T)> {
        let w = self.width();
        let iter = self
            .data
            .chunks_mut(w)
            .skip(rect.bottom() as usize)
            .flat_map(move |tiles| tiles[rect.left() as usize..=rect.right() as usize].iter_mut());
        rect.iter_rect_points().zip(iter)
    }

    /// Retrieve a slice of the underlying grid data.
    pub fn slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Retrieve a mutable slice of the underlying grid data.
    pub fn slice_mut(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    /// Returns the bounds of the grid, with it's bottom left tile at
    /// world origin.
    #[inline]
    pub fn bounds(&self) -> GridRect {
        GridRect::new([0, 0], self.size)
    }
}

impl<T> SizedGrid for Grid<T> {
    fn size(&self) -> UVec2 {
        self.size
    }
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
        let xy = index.to_ivec2();
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

#[cfg(test)]
mod tests {
    use super::*;

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

        let iter = grid.iter_rect(GridRect::from_points([2, 2], [4, 4]));
        let vec: Vec<_> = iter.collect();

        assert_eq!(vec.len(), 9);
        assert_eq!(*vec[0].1, 5);
        assert_eq!(*vec[8].1, 10);

        let mut iter = grid.iter_rect(GridRect::from_points([2, 2], [4, 4]));

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
}
