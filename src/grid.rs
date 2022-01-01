//! A 2d dense grid of [T].
//! 
//! Elements can be inserted and accessed via their 1d index, 2d index, or 
//! read/modified via iterators.
//! 
//! # Example 
//! 
//! ```
//! use sark_grids::grid::Grid;
//! 
//! let mut grid = Grid::default([10,10]);
//! 
//! grid[0] = 'a';
//! grid[ [1,0] ] = 'b';
//! 
//! assert_eq!('a', grid[0]);
//! assert_eq!('b', grid[ [1,0] ]);
//! 
//! grid.insert_column_at([3,2], "hello".chars());
//! let hello: String = grid.column_iter(3).skip(2).take(5).collect();
//! 
//! assert_eq!("hello", hello);
//! ```

use std::{
    iter::StepBy,
    ops::{Index, IndexMut, RangeBounds},
    slice::{Iter, IterMut},
};

use glam::{IVec2, UVec2};
use itertools::Itertools;

use crate::{world_grid::WorldGrid, Pivot};

/// A dense sized grid of [T].
///
/// This grid assumes that `[0,0]` refers to the bottom-left most tile, and
/// `[width -1, height -1]` refers to the top-right-most tile.
pub struct Grid<T: Clone> {
    data: Vec<T>,
    size: UVec2,
}

impl<T: Clone> Grid<T> {
    /// Creates a new [Grid<T>] with the given default value set for all elements.
    pub fn new(value: T, size: [u32; 2]) -> Self {
        let size = UVec2::from(size);
        let len = (size.x * size.y) as usize;

        Self {
            data: vec![value; len],
            size,
        }
    }

    /// Creates a new [Grid<T>] with all elements initialized to default values.
    pub fn default(size: [u32; 2]) -> Self
    where
        T: Default,
    {
        Grid::new(T::default(), size)
    }

    /// An iterator over all elements in the grid.
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    /// A mutable iterator over all elements in the grid.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    /// An iterator over a single row of the grid.
    ///
    /// Goes from left to right.
    #[inline]
    pub fn row_iter(&self, y: usize) -> Iter<T> {
        let w = self.width() as usize;
        let i = y * w;
        self.data[i..i + w].iter()
    }

    /// A mutable iterator over a single row of the grid.
    ///
    /// Goes from left to right.
    #[inline]
    pub fn row_iter_mut(&mut self, y: usize) -> IterMut<T> {
        let w = self.width() as usize;
        let i = y * w;
        self.data[i..i + w].iter_mut()
    }

    /// Insert into a row of the grid using an iterator.
    /// 
    /// Will insert up to the length of a row.
    pub fn insert_row(&mut self, y: usize, row: impl IntoIterator<Item=T>)
    {
        self.insert_row_at([0,y as i32], row);
    }

    /// Insert into a row of the grid using an iterator.
    /// 
    /// Will insert up to the length of a row.
    pub fn insert_row_at(&mut self, xy: [i32;2], row: impl IntoIterator<Item=T>) {
        let [x,y] = xy;
        let iter = self.row_iter_mut(y as usize).skip(x as usize);
        for (v, input) in iter.zip(row) {
            *v = input;
        }
    }
    
    /// Insert into a column of the grid using an iterator.
    /// 
    /// Will insert up to the height of a column.
    pub fn insert_column(&mut self, x: usize, column: impl IntoIterator<Item=T>) {
        self.insert_column_at([x as i32,0], column);
    }
    
    /// Insert into a column of the grid using an iterator.
    /// 
    /// Will insert up to the height of a column.
    pub fn insert_column_at(&mut self, xy: [i32;2], column: impl IntoIterator<Item=T>) {
        let [x,y] = xy;
        let iter = self.column_iter_mut(x as usize).skip(y as usize);
        for (v, input) in iter.zip(column) {
            *v = input;
        }
    }

    /// An iterator over a single column of the grid.
    ///
    /// Goes from bottom to top.
    #[inline]
    pub fn column_iter(&self, x: usize) -> StepBy<Iter<T>> {
        let w = self.width() as usize;
        return self.data[x..].iter().step_by(w);
    }

    /// A mutable iterator over a single column of the grid.
    ///
    /// Goes from bottom to top.
    #[inline]
    pub fn column_iter_mut(&mut self, x: usize) -> StepBy<IterMut<T>> {
        let w = self.width() as usize;
        return self.data[x..].iter_mut().step_by(w);
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
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.data.len()
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

    #[allow(dead_code)]
    pub(crate) fn debug_bounds_check(&self, pos: IVec2) {
        debug_assert!(
            self.is_in_bounds(pos),
            "Position {} is out of grid bounds {}",
            pos,
            self.size()
        );
    }

    /// An iterator over a rectangular portion of the grid defined by the given range.
    ///
    /// Yields `(IVec2, &T)`, where `IVec2` is the corresponding position of the value in the grid.
    pub fn rect_iter<RANGE: RangeBounds<[i32; 2]>>(
        &self,
        range: RANGE,
    ) -> impl Iterator<Item = (IVec2, &T)> {
        let (min, max) = ranges_to_min_max(range, self.size().as_ivec2());
        (min.y..=max.y)
            .cartesian_product(min.x..=max.x)
            .map(|(y, x)| ((IVec2::new(x, y)), &self[[x as u32, y as u32]]))
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
        (0..self.height())
            .cartesian_product(0..self.width())
            .map(|(y, x)| IVec2::new(x as i32, y as i32))
            .zip(self.data.iter_mut())
    }

    /// Creates a [crate::world_grid::WorldGrid] from this grid with the given pivot. This can be used to translate
    /// between grid points and world space.
    pub fn to_world_pivot(&self, pivot: Pivot) -> WorldGrid {
        WorldGrid::origin(self.size.into(), pivot)
    }

    /// Creates a [crate::world_grid::WorldGrid] from this grid with the default bottom left pivot. This can be used to translate
    /// between grid points and world space.
    pub fn to_world(&self) -> WorldGrid {
        self.to_world_pivot(Pivot::BottomLeft)
    }
}

fn ranges_to_min_max<RANGE: RangeBounds<[i32; 2]>>(range: RANGE, max: IVec2) -> (IVec2, IVec2) {
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

impl<T: Clone> Index<[u32; 2]> for Grid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: [u32; 2]) -> &Self::Output {
        &self.data[self.upos_to_index(index)]
    }
}

impl<T: Clone> IndexMut<[u32; 2]> for Grid<T> {
    #[inline(always)]
    fn index_mut(&mut self, pos: [u32; 2]) -> &mut Self::Output {
        let index = self.upos_to_index(pos);
        &mut self.data[index]
    }
}

impl<T: Clone> Index<usize> for Grid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: Clone> IndexMut<usize> for Grid<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Clone> Index<IVec2> for Grid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: IVec2) -> &Self::Output {
        &self.data[self.pos_to_index(index.into())]
    }
}

impl<T: Clone> IndexMut<IVec2> for Grid<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: IVec2) -> &mut Self::Output {
        let index = self.pos_to_index(index.into());
        &mut self.data[index]
    }
}
impl<T: Clone> Index<UVec2> for Grid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: UVec2) -> &Self::Output {
        &self.data[self.upos_to_index(index.into())]
    }
}

impl<T: Clone> IndexMut<UVec2> for Grid<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: UVec2) -> &mut Self::Output {
        let index = self.upos_to_index(index.into());
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_iter() {
        let mut grid = Grid::default([10, 15]);

        let chars = "hello".chars();

        for (elem, ch) in grid.row_iter_mut(3).take(5).zip(chars) {
            *elem = ch;
        }

        let hello = grid.row_iter(3).take(5).collect::<String>();

        assert_eq!("hello", hello);

        assert_eq!(grid.row_iter(6).len(), 10);
    }

    #[test]
    fn column_iter() {
        let mut grid = Grid::default([10, 15]);

        let chars = ['h', 'e', 'l', 'l', 'o'];

        for (elem, ch) in grid.column_iter_mut(5).take(5).zip(chars) {
            *elem = ch;
        }

        let hello = grid.column_iter(5).take(5).collect::<String>();

        assert_eq!("hello", hello);

        assert_eq!(grid.column_iter(2).len(), 15);
    }

    #[test]
    fn iter_2d() {
        let mut grid = Grid::new(0, [5, 3]);
        grid[[0, 0]] = 5;
        grid[[3, 1]] = 10;
        grid[[4, 2]] = 20;

        let vec: Vec<_> = grid.iter_2d().collect();

        assert_eq!(vec.len(), 5 * 3);
        assert_eq!(*vec[grid.pos_to_index([0, 0])].1, 5);
        assert_eq!(*vec[grid.pos_to_index([3, 1])].1, 10);
        assert_eq!(*vec[grid.pos_to_index([4, 2])].1, 20);

        let mut iter = grid.iter_2d();
        let (p, _) = iter.next().unwrap();
        assert_eq!(0, p.x);
        assert_eq!(0, p.y);
        let (p, _) = iter.next().unwrap();
        assert_eq!(1, p.x);
        assert_eq!(0, p.y);

        let (p, _) = iter.skip(3).next().unwrap();
        assert_eq!(0, p.x);
        assert_eq!(1, p.y);
    }

    #[test]
    fn iter() {
        let grid = Grid::new(5, [10, 10]);

        let v: Vec<_> = grid.iter().collect();

        assert_eq!(v.len(), 100);
        assert_eq!(*v[0], 5);
        assert_eq!(*v[99], 5);
    }

    #[test]
    fn iter_mut() {
        let mut grid = Grid::new(5, [10, 10]);

        for i in grid.iter_mut() {
            *i = 10;
        }

        assert_eq!(grid[0], 10);
    }

    #[test]
    fn positions() {
        let grid = Grid::new(0, [4, 4]);

        assert_eq!(grid.pivot_position(Pivot::TopLeft), IVec2::new(0, 3));
        assert_eq!(grid.pivot_position(Pivot::TopRight), IVec2::new(3, 3));
        assert_eq!(grid.pivot_position(Pivot::BottomRight), IVec2::new(3, 0));
        assert_eq!(grid.pivot_position(Pivot::BottomLeft), IVec2::new(0, 0));
        assert_eq!(grid.pivot_position(Pivot::Center), IVec2::new(1, 1));

        let grid = Grid::new(0, [5, 5]);

        assert_eq!(grid.pivot_position(Pivot::TopLeft), IVec2::new(0, 4));
        assert_eq!(grid.pivot_position(Pivot::TopRight), IVec2::new(4, 4));
        assert_eq!(grid.pivot_position(Pivot::BottomRight), IVec2::new(4, 0));
        assert_eq!(grid.pivot_position(Pivot::BottomLeft), IVec2::new(0, 0));
        assert_eq!(grid.pivot_position(Pivot::Center), IVec2::new(2, 2));
    }

    #[test]
    fn rect_iter() {
        let mut grid = Grid::new(0, [11, 15]);

        grid[[2, 2]] = 5_i32;
        grid[[4, 4]] = 10;

        let iter = grid.rect_iter([2, 2]..=[4, 4]);
        let vec: Vec<_> = iter.collect();

        assert_eq!(vec.len(), 9);
        assert_eq!(*vec[0].1, 5);
        assert_eq!(*vec[8].1, 10);

        let mut iter = grid.rect_iter([2, 2]..=[4, 4]);

        let (p, _) = iter.next().unwrap();
        assert_eq!(p, IVec2::new(2, 2));
        assert_eq!(iter.skip(7).next().unwrap().0, IVec2::new(4, 4));
    }

    #[test]
    fn column_insert() {
        let mut grid = Grid::default([10,10]);

        grid.insert_column(3, "Hello".chars());

        let hello: String = grid.column_iter(3).take(5).collect();

        assert_eq!(hello, "Hello");
    }

    #[test]
    fn row_insert() {
        let mut grid = Grid::default([10,10]);

        grid.insert_row(3, "Hello".chars());

        let hello: String = grid.row_iter(3).take(5).collect();

        assert_eq!(hello, "Hello");
    }
}
