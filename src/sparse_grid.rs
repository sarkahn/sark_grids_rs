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
//! grid.insert_index(4, 'i');
//! grid.insert([3,0], 'h');
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

use std::{collections::BTreeMap, ops::Index};

use glam::{IVec2, UVec2};

use crate::Pivot;

/// A sparse grid that stores elements in a [BTreeMap].
#[derive(Default, Debug, Clone)]
pub struct SparseGrid<T: Clone> {
    data: BTreeMap<u32, T>,
    size: UVec2,
}

impl<T: Clone> SparseGrid<T> {
    /// Creates a new [SparseGrid<T>].
    pub fn new(size: [u32; 2]) -> Self {
        let size = UVec2::from(size);

        Self {
            data: BTreeMap::new(),
            size,
        }
    }

    /// An iterator over all elements in the grid.
    ///
    /// Yields `(&u32,&mut T)` where `u32` is the 1d position of the element in the grid.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&u32, &T)> {
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
    /// Yields `(&u32,&mut T)` where `u32` is the 1d position of the element in the grid.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&u32, &mut T)> {
        self.data.iter_mut()
    }

    /// A 2d iterator over all elements in the grid.
    ///
    /// Yields `(UVec2,&mut T)` where `UVec2` is the 2d position of the element in the grid.
    #[inline]
    pub fn iter_2d(&self) -> impl Iterator<Item = (UVec2, &T)> {
        let w = self.width();
        self.data.iter().map(move |(i, v)| {
            let x = i % w;
            let y = i / w;
            (UVec2::new(x, y), v)
        })
    }

    /// A mutable iterator over all elements in the grid.
    ///
    /// Yields `(UVec,&mut T)` where `UVec2` is the 2d position of the element in the grid.
    #[inline]
    pub fn iter_mut_2d(&mut self) -> impl Iterator<Item = (UVec2, &mut T)> {
        let w = self.width();
        self.data.iter_mut().map(move |(i, v)| {
            let x = i % w;
            let y = i / w;
            (UVec2::new(x, y), v)
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
        xy: [i32; 2],
        row: impl IntoIterator<Item = T> + Iterator<Item = T>,
    ) {
        let start = self.pos_to_index(xy) as u32;
        let max = self.width() as usize - 1 - xy[0] as usize;
        for (x, v) in row.take(max).enumerate() {
            self.data.insert(start + x as u32, v);
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

    /// Insert into a column of the grid using an iterator.
    ///
    /// Will insert up to the height of a column.
    pub fn insert_column_at(
        &mut self,
        xy: [i32; 2],
        column: impl IntoIterator<Item = T> + Iterator<Item = T>,
    ) {
        let start = self.pos_to_index(xy) as u32;
        let max = self.height() as usize - 1 - xy[1] as usize;
        for (y, v) in column.take(max).enumerate() {
            let i = start + (y as u32 * self.width());
            self.data.insert(i, v);
        }
    }

    /// Remove the element/tile at the given position.
    ///
    /// Returns the removed element if one was present.
    pub fn remove(&mut self, pos: [u32; 2]) -> Option<T> {
        let i = self.upos_to_index(pos) as u32;
        self.data.remove(&i)
    }

    /// Remove the element/tile at the given 1d index.
    ///
    /// Returns the removed element if one was present.
    pub fn remove_index(&mut self, index: usize) -> Option<T> {
        let index = index as u32;
        self.data.remove(&index)
    }

    /// Clears the grid, removing all elements.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn width(&self) -> u32 {
        self.size.x
    }

    pub fn height(&self) -> u32 {
        self.size.y
    }

    pub fn size(&self) -> UVec2 {
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
    pub fn pos_to_index(&self, pos: [i32; 2]) -> usize {
        (pos[1] * self.width() as i32 + pos[0]) as usize
    }

    /// Converts a 2d grid position to it's corresponding 1D index.
    #[inline(always)]
    pub fn upos_to_index(&self, pos: [u32; 2]) -> usize {
        (pos[1] * self.width() as u32 + pos[0]) as usize
    }

    /// Converts a 1d index to it's corresponding grid position.
    #[inline(always)]
    pub fn index_to_pos(&self, index: usize) -> IVec2 {
        let index = index as i32;
        let w = self.width() as i32;
        let x = index % w;
        let y = index / w;
        IVec2::new(x, y)
    }

    /// Converts a 1d index to it's corresponding grid position.
    #[inline(always)]
    pub fn index_to_upos(&self, index: usize) -> UVec2 {
        self.index_to_pos(index).as_uvec2()
    }

    /// Returns the index of the top row.
    #[inline(always)]
    pub fn top_index(&self) -> usize {
        (self.height() - 1) as usize
    }

    /// Returns the index of the bottom row (`0`).
    #[inline(always)]
    pub fn bottom_index(&self) -> usize {
        0
    }

    /// Returns the index of the left-most column (`0`).
    #[inline(always)]
    pub fn left_index(&self) -> usize {
        0
    }

    /// Returns the index of the right-most column.
    #[inline(always)]
    pub fn right_index(&self) -> usize {
        (self.width() - 1) as usize
    }

    /// Get the position of a tile on the grid at the given pivot.
    ///
    /// Note that for even-sized grids the "center" will be rounded down.
    /// For example, for a a 4x4 grid calling `pivot_position(Pivot::Center)` will return `(1,1)`.
    pub fn pivot_position(&self, pivot: Pivot) -> IVec2 {
        match pivot {
            Pivot::TopLeft => IVec2::new(0, self.top_index() as i32),
            Pivot::TopRight => IVec2::new(self.right_index() as i32, self.top_index() as i32),
            Pivot::Center => {
                let tr = self.pivot_position(Pivot::TopRight);
                (tr.as_vec2() / 2.0).as_ivec2()
            }
            Pivot::BottomLeft => IVec2::ZERO,
            Pivot::BottomRight => IVec2::new(self.right_index() as i32, 0),
        }
    }

    pub fn is_in_bounds(&self, pos: IVec2) -> bool {
        pos.cmpge(IVec2::ZERO).all() && pos.cmplt(self.size().as_ivec2()).all()
    }

    /// Insert a value in the grid.
    ///
    /// Returns `None` if no value was already present. Otherwise the old value
    /// is returned.
    #[inline]
    pub fn insert_index(&mut self, index: usize, value: T) -> Option<T> {
        self.data.insert(index as u32, value)
    }

    #[inline]
    pub fn insert(&mut self, pos: [i32; 2], value: T) -> Option<T> {
        let pos = IVec2::from(pos);
        let i = self.pos_to_index(pos.into());
        self.data.insert(i as u32, value)
    }

    /// Retrieve a value in the grid from it's 1d index.
    ///
    /// Returns `None` if there is no value at the index.
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<&T> {
        let i = index as u32;
        self.data.get(&i)
    }

    /// Retrieve a mutable value in the grid from it's 1d index.
    ///
    /// Returns `None` if there is no value at the index.
    #[inline]
    pub fn get_mut_index(&mut self, index: usize) -> Option<&mut T> {
        let i = index as u32;
        self.data.get_mut(&i)
    }

    /// Retrieve a value in the grid from it's 2d position.
    ///
    /// Returns `None` if there is no value at the position.
    #[inline]
    pub fn get(&self, pos: [i32; 2]) -> Option<&T> {
        let pos = IVec2::from(pos);
        let i = self.pos_to_index(pos.into());
        self.get_index(i)
    }

    /// Retrieve a mutable value in the grid from it's 2d position.
    ///
    /// Returns `None` if there is no value at the position.
    #[inline]
    pub fn get_mut(&mut self, pos: [i32; 2]) -> Option<&mut T> {
        let pos = IVec2::from(pos);
        let i = self.pos_to_index(pos.into()) as u32;
        self.data.get_mut(&i)
    }

    #[allow(dead_code)]
    pub(crate) fn debug_bounds_check(&self, pos: IVec2) {
        debug_assert!(
            self.is_in_bounds(pos),
            "Position {} is out of grid bounds {}",
            pos,
            self.size()
        );
    }
}

impl<T: Clone> Index<[u32; 2]> for SparseGrid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: [u32; 2]) -> &Self::Output {
        let i = self.upos_to_index(index) as u32;
        &self.data[&i]
    }
}

impl<T: Clone> Index<usize> for SparseGrid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        let index = index as u32;
        &self.data[&index]
    }
}

impl<T: Clone> Index<IVec2> for SparseGrid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: IVec2) -> &Self::Output {
        let index = self.pos_to_index(index.into()) as u32;
        &self.data[&index]
    }
}

impl<T: Clone> Index<UVec2> for SparseGrid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: UVec2) -> &Self::Output {
        let index = self.upos_to_index(index.into()) as u32;
        &self.data[&index]
    }
}

#[cfg(test)]
mod test {
    use glam::UVec2;

    use super::SparseGrid;

    #[test]
    fn insert_row() {
        let mut grid = SparseGrid::new([10, 10]);

        grid.insert_row(5, "Hello".chars());

        assert_eq!(5, grid.len());

        let str: String = grid.iter_2d().map(|(_, v)| v).collect();

        assert_eq!("Hello", str);

        for (i, (p, _)) in grid.iter_2d().enumerate() {
            assert_eq!((i as u32, 5), p.into());
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

        assert_eq!((UVec2::new(3, 3), &'H'), kvp[0]);
        assert_eq!((UVec2::new(4, 3), &'e'), kvp[1]);
        assert_eq!((UVec2::new(5, 3), &'l'), kvp[2]);
        assert_eq!((UVec2::new(6, 3), &'l'), kvp[3]);
        assert_eq!((UVec2::new(7, 3), &'o'), kvp[4]);
    }

    #[test]
    fn insert_col() {
        let mut grid = SparseGrid::new([10, 10]);

        grid.insert_column(5, "Hello".chars());

        assert_eq!(5, grid.len());

        let str: String = grid.iter_2d().map(|(_, v)| v).collect();

        assert_eq!("Hello", str);

        for (i, (p, _)) in grid.iter_2d().enumerate() {
            assert_eq!((5, i as u32,), p.into());
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

        assert_eq!((UVec2::new(3, 3), &'H'), kvp[0]);
        assert_eq!((UVec2::new(3, 4), &'e'), kvp[1]);
        assert_eq!((UVec2::new(3, 5), &'l'), kvp[2]);
        assert_eq!((UVec2::new(3, 6), &'l'), kvp[3]);
        assert_eq!((UVec2::new(3, 7), &'o'), kvp[4]);
    }

    #[test]
    fn insert() {
        let mut grid = SparseGrid::new([10, 10]);

        grid.insert([0, 0], 'h');
        grid.insert([1, 3], '3');

        assert_eq!(2, grid.len());

        assert_eq!('h', grid[[0, 0]]);
        assert_eq!('3', *grid.get([1, 3]).unwrap());
    }
}
