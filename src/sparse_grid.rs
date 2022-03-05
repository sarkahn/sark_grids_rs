//! A grid that stores it's internal data in a `BTreeMap`. Elements don't take up any memory until
//! they're inserted, and can be removed as needed, but iteration and access speed will be slower
//! than a `Grid` for large full grids.
//!
//! Elements can be inserted and accessed via their 1d index or 2d index, or
//! read/modified via iterators.
//!
//! # Example
//!
//! ```
//! use sark_grids::sparse_grid::SparseGrid;
//!
//! let mut grid = SparseGrid::new([10,10]);
//!
//! grid[4] = 'i';
//! grid[[3,0]]= 'h';
//!
//! assert_eq!(2, grid.len());
//!
//! let hi: String = grid.iter_values().collect();
//! assert_eq!("hi", hi);
//!
//! grid.insert_row_at([3,0], "ih".chars());
//! let ih: String = grid.iter_values().collect();
//!
//! assert_eq!("ih", ih);
//! ```

use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
};

use glam::IVec2;

use crate::{grid::Side, point::*};

/// A sparse grid that stores elements in a [BTreeMap].
#[derive(Default, Debug, Clone)]
pub struct SparseGrid<T: Clone> {
    data: BTreeMap<usize, T>,
    size: IVec2,
}

impl<T: Clone> SparseGrid<T> {
    /// Creates a new [SparseGrid<T>].
    pub fn new(size: impl Size2d) -> Self {
        Self {
            data: BTreeMap::new(),
            size: size.as_ivec2(),
        }
    }

    /// An iterator over all elements in the grid.
    ///
    /// Yields `(&usize,&mut T)` where `usize` is the 1d position of the element in the grid.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&usize, &T)> {
        self.data.iter()
    }

    /// An iterator over just the values in the grid.
    ///
    /// Yields `&T`.
    pub fn iter_values(&self) -> impl Iterator<Item = &T> {
        self.data.iter().map(move |(_, v)| v)
    }

    /// A mutable iterator over just the values in the grid.
    ///
    /// Yields `&mut T`.
    pub fn iter_values_mut(&self) -> impl Iterator<Item = &T> {
        self.data.iter().map(move |(_, v)| v)
    }

    /// A mutable iterator over all elements in the grid.
    ///
    /// Yields `(&usize,&mut T)` where `usize` is the 1d position of the element in the grid.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&usize, &mut T)> {
        self.data.iter_mut()
    }

    /// A 2d iterator over all elements in the grid.
    ///
    /// Yields `(IVec2,&mut T)` where `IVec2` is the 2d position of the element in the grid.
    #[inline]
    pub fn iter_2d(&self) -> impl Iterator<Item = (IVec2, &T)> {
        let w = self.width();
        self.data.iter().map(move |(i, v)| {
            let x = i % w;
            let y = i / w;
            (IVec2::new(x as i32, y as i32), v)
        })
    }

    /// A mutable iterator over all elements in the grid.
    ///
    /// Yields `(IVec,&mut T)` where `IVec2` is the 2d position of the element in the grid.
    #[inline]
    pub fn iter_mut_2d(&mut self) -> impl Iterator<Item = (IVec2, &mut T)> {
        let w = self.width();
        self.data.iter_mut().map(move |(i, v)| {
            let x = i % w;
            let y = i / w;
            (IVec2::new(x as i32, y as i32), v)
        })
    }

    /// Insert into a row of the grid using an iterator.
    ///
    /// Will insert up to the length of a row.
    pub fn insert_row(&mut self, y: usize, row: impl IntoIterator<Item = T> + Iterator<Item = T>) {
        self.insert_row_at([0, y as i32], row);
    }

    /// Insert into a row of the grid using an iterator.
    ///
    /// Will insert up to the length of a row.
    pub fn insert_row_at(
        &mut self,
        xy: impl Point2d,
        row: impl IntoIterator<Item = T> + Iterator<Item = T>,
    ) {
        let start = self.pos_to_index(xy);
        let max = self.width() - 1 - xy.x() as usize;
        for (x, v) in row.take(max).enumerate() {
            self.data.insert(start + x, v);
        }
    }

    /// Insert into a column of the grid using an iterator.
    ///
    /// Will insert up to the height of a column.
    pub fn insert_column(
        &mut self,
        x: usize,
        column: impl IntoIterator<Item = T> + Iterator<Item = T>,
    ) {
        self.insert_column_at([x as i32, 0], column);
    }

    /// Insert into a column of the grid starting from some point using an iterator.
    ///
    /// Will insert up to the height of a column.
    pub fn insert_column_at(
        &mut self,
        xy: impl Point2d,
        column: impl IntoIterator<Item = T> + Iterator<Item = T>,
    ) {
        let start = self.pos_to_index(xy);
        let max = self.height() - 1 - xy.y() as usize;
        for (y, v) in column.take(max).enumerate() {
            let i = start + (y * self.width());
            self.data.insert(i, v);
        }
    }

    /// Remove the element/tile at the given position.
    ///
    /// Returns the removed element if one was present.
    pub fn remove(&mut self, pos: impl Point2d) -> Option<T> {
        let i = self.pos_to_index(pos);
        self.data.remove(&i)
    }

    /// Remove the element/tile at the given 1d index.
    ///
    /// Returns the removed element if one was present.
    pub fn remove_index(&mut self, index: usize) -> Option<T> {
        let index = index;
        self.data.remove(&index)
    }

    /// Clears the grid, removing all elements.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn width(&self) -> usize {
        self.size.x as usize
    }

    pub fn height(&self) -> usize {
        self.size.y as usize
    }

    pub fn size(&self) -> impl Point2d {
        self.size
    }

    /// How many tiles/elements are in the grid.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Converts a 2d grid position to it's corresponding 1D index.
    #[inline(always)]
    pub fn pos_to_index(&self, pos: impl Point2d) -> usize {
        let [x, y] = pos.to_array();
        (y * self.width() as i32 + x) as usize
    }

    /// Converts a 1d index to it's corresponding grid position.
    #[inline(always)]
    pub fn index_to_pos(&self, index: usize) -> impl Point2d {
        let index = index as i32;
        let w = self.width() as i32;
        let x = index % w;
        let y = index / w;
        [x, y]
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

    pub fn is_in_bounds(&self, pos: impl Point2d) -> bool {
        let xy = pos.as_ivec2();
        xy.cmpge(IVec2::ZERO).all() && xy.cmplt(self.size).all()
    }

    /// Insert a value in the grid.
    ///
    /// Returns `None` if no value was already present. Otherwise the old value
    /// is returned.
    #[inline]
    pub fn insert_index(&mut self, index: usize, value: T) -> Option<T> {
        self.data.insert(index, value)
    }

    /// Insert a value in the grid.
    ///
    /// Returns `None` if no value was already present. Otherwise the old value
    /// is returned.
    #[inline]
    pub fn insert(&mut self, pos: impl Point2d, value: T) -> Option<T> {
        let pos = pos.as_ivec2();
        let i = self.pos_to_index(pos);
        self.data.insert(i, value)
    }

    /// Retrieve a value in the grid from it's 1d index.
    ///
    /// Returns `None` if there is no value at the index.
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<&T> {
        self.data.get(&index)
    }

    /// Retrieve a mutable value in the grid from it's 1d index.
    ///
    /// Returns `None` if there is no value at the index.
    #[inline]
    pub fn get_mut_index(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(&index)
    }

    /// Retrieve a value in the grid from it's 2d position.
    ///
    /// Returns `None` if there is no value at the position.
    #[inline]
    pub fn get(&self, pos: impl Point2d) -> Option<&T> {
        let i = self.pos_to_index(pos.as_ivec2());
        self.get_index(i)
    }

    /// Retrieve a mutable value in the grid from it's 2d position.
    ///
    /// Returns `None` if there is no value at the position.
    #[inline]
    pub fn get_mut(&mut self, pos: impl Point2d) -> Option<&mut T> {
        let i = self.pos_to_index(pos.as_ivec2());
        self.data.get_mut(&i)
    }
}

impl<T: Clone, P: Point2d> Index<P> for SparseGrid<T> {
    type Output = T;

    fn index(&self, index: P) -> &Self::Output {
        let xy = index.as_ivec2();
        let i = self.pos_to_index(xy);
        &self.data[&i]
    }
}

impl<T: Clone, P: Point2d> IndexMut<P> for SparseGrid<T>
where
    T: Default,
{
    fn index_mut(&mut self, index: P) -> &mut T {
        let xy = index.as_ivec2();
        let i = self.pos_to_index(xy);
        &mut *self.data.entry(i).or_default()
    }
}

impl<T: Clone> Index<usize> for SparseGrid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[&index]
    }
}
impl<T: Clone> IndexMut<usize> for SparseGrid<T>
where
    T: Default,
{
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut *self.data.entry(index).or_default()
    }
}

#[cfg(test)]
mod test {
    use glam::IVec2;

    use crate::point::Point2d;

    use super::SparseGrid;

    #[test]
    fn index() {
        let mut grid = SparseGrid::new([10, 17]);

        let [x, y] = grid.index_to_pos(5).to_array();

        grid[[5, 6]] = 10;

        assert_eq!(grid[[5, 6]], 10);

        let xy = IVec2::new(x, y);

        grid[xy] = 15;
        assert_eq!(grid[xy], 15);
    }

    #[test]
    fn insert_row() {
        let mut grid = SparseGrid::new([10, 10]);

        grid.insert_row(5, "Hello".chars());

        assert_eq!(5, grid.len());

        let str: String = grid.iter_2d().map(|(_, v)| v).collect();

        assert_eq!("Hello", str);

        for (i, (p, _)) in grid.iter_2d().enumerate() {
            assert_eq!([i as i32, 5], p.to_array());
        }
    }

    #[test]
    fn insert_row_at() {
        let mut grid = SparseGrid::new([10, 10]);

        grid.insert_row_at([3, 3], "Hello".chars());

        assert_eq!(5, grid.len());

        let str: String = grid.iter_values().collect();

        assert_eq!("Hello", str);

        let kvp: Vec<_> = grid.iter_2d().collect();

        assert_eq!((IVec2::new(3, 3), &'H'), kvp[0]);
        assert_eq!((IVec2::new(4, 3), &'e'), kvp[1]);
        assert_eq!((IVec2::new(5, 3), &'l'), kvp[2]);
        assert_eq!((IVec2::new(6, 3), &'l'), kvp[3]);
        assert_eq!((IVec2::new(7, 3), &'o'), kvp[4]);
    }

    #[test]
    fn insert_col() {
        let mut grid = SparseGrid::new([10, 10]);

        grid.insert_column(5, "Hello".chars());

        assert_eq!(5, grid.len());

        let str: String = grid.iter_2d().map(|(_, v)| v).collect();

        assert_eq!("Hello", str);

        for (i, (p, _)) in grid.iter_2d().enumerate() {
            assert_eq!([5, i as i32], p.to_array());
        }
    }

    #[test]
    fn insert_col_at() {
        let mut grid = SparseGrid::new([10, 10]);

        grid.insert_column_at([3, 3], "Hello".chars());

        assert_eq!(5, grid.len());

        let str: String = grid.iter_2d().map(|(_, v)| v).collect();

        assert_eq!("Hello", str);

        let kvp: Vec<_> = grid.iter_2d().collect();

        assert_eq!((IVec2::new(3, 3), &'H'), kvp[0]);
        assert_eq!((IVec2::new(3, 4), &'e'), kvp[1]);
        assert_eq!((IVec2::new(3, 5), &'l'), kvp[2]);
        assert_eq!((IVec2::new(3, 6), &'l'), kvp[3]);
        assert_eq!((IVec2::new(3, 7), &'o'), kvp[4]);
    }

    #[test]
    fn insert() {
        let mut grid = SparseGrid::new([10, 10]);

        grid[[0, 0]] = 'h';
        grid[[1, 3]] = '3';

        assert_eq!(2, grid.len());

        assert_eq!('h', grid[[0, 0]]);
        assert_eq!('3', grid[[1, 3]]);
    }
}
