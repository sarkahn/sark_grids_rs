// TODO: Replace with bevy hashmap
use std::{slice::{Iter, IterMut}, collections::BTreeMap, ops::{Index, IndexMut}};

use glam::{UVec2, IVec2};

use crate::Pivot;

/// A sparse grid of [T].
pub struct SparseGrid<T: Clone> 
{
    data: BTreeMap<u32, T>,
    size: UVec2,
}

impl<T: Clone> SparseGrid<T> {
    /// Creates a new [Grid<T>].
    pub fn new(size: [u32; 2]) -> Self
    {
        let size = UVec2::from(size);

        Self {
            data: BTreeMap::new(),
            size,
        }
    }

    /// An iterator over all elements in the grid.
    /// 
    /// Yields `(UVec,&mut T)` where UVec is the 2d position of the element in the grid.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=(UVec2,&T)> {
        let w = self.width();
        self.data.iter().map(move |(i,v)| {
            let x = i % w;
            let y = i / w;
            (UVec2::new(x,y), v)
        })
    }

    /// A mutable iterator over all elements in the grid.
    /// 
    /// Yields `(UVec,&mut T)` where UVec is the 2d position of the element in the grid.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item=(UVec2,&mut T)> {
        let w = self.width();
        self.data.iter_mut().map(move |(i,v)| {
            let x = i % w;
            let y = i / w;
            (UVec2::new(x,y), v)
        })
    }

    pub fn insert_row(&mut self, y: usize, row: impl IntoIterator<Item=T> + Iterator<Item=T>) {
        self.insert_row_at([0,y as i32], row);
    }

    pub fn insert_row_at(&mut self, xy: [i32;2], row: impl IntoIterator<Item=T> + Iterator<Item=T>) {
        let start = self.pos_to_index(xy) as u32;
        let max = self.width() as usize - 1 - xy[0] as usize;
        for (x,v) in row.take(max).enumerate() {
            self.data.insert(start + x as u32, v);
        }
    }

    pub fn insert_col(&mut self, x: usize, column: impl IntoIterator<Item=T> + Iterator<Item=T>) {
        self.insert_col_at([x as i32,0], column);
    }
    
    pub fn insert_col_at(&mut self, xy: [i32;2], column: impl IntoIterator<Item=T> + Iterator<Item=T>) {
        let start = self.pos_to_index(xy) as u32;
        let max = self.height() as usize - 1 - xy[1] as usize;
        for (y,v) in column.take(max).enumerate() {
            let i = start + (y as u32 * self.width());
            self.data.insert(i, v);
        }
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
        return self.data.is_empty()
    }

    /// Returns a grid position relative to a pivot point on the grid.
    ///
    /// The position will be axis-aligned with the given pivot point. For instance,
    /// for a top left pivot point the x coordinate will increase to the right,
    /// but the y coordinate will increase downwards.
    ///
    /// The center pivot will act the same as the default - where the x coordinate increases
    /// to the right and the y coordinate increases upward.
    ///
    /// Note that for even-sized grids the "center" will be rounded down.
    /// For example, for a a 4x4 grid calling `pos_from_pivot([0,0], Pivot::Center)` will return [1,1].
    #[inline]
    pub fn pos_from_pivot(&self, pos: [i32; 2], pivot: Pivot) -> IVec2 {
        let tr = self.pivot_position(Pivot::TopRight);
        let pivot_offset = (tr.as_vec2() * pivot.normalized()).as_ivec2();
        IVec2::from(pos) * pivot.axis() + pivot_offset
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
}

impl<T: Clone> Index<[u32; 2]> for SparseGrid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: [u32; 2]) -> &Self::Output {
        let i = self.upos_to_index(index) as u32;
        &self.data[&i]
    }
}

// impl<T: Clone> IndexMut<[u32; 2]> for SparseGrid<T> {
//     #[inline(always)]
//     fn index_mut(&mut self, pos: [u32; 2]) -> &mut Self::Output {
//         let index = self.upos_to_index(pos);
//         &mut self.data[index]
//     }
// }

// impl<T: Clone> Index<usize> for SparseGrid<T> {
//     type Output = T;

//     #[inline(always)]
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.data[index]
//     }
// }

// impl<T: Clone> IndexMut<usize> for SparseGrid<T> {
//     #[inline(always)]
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.data[index]
//     }
// }

// impl<T: Clone> Index<IVec2> for SparseGrid<T> {
//     type Output = T;

//     #[inline(always)]
//     fn index(&self, index: IVec2) -> &Self::Output {
//         &self.data[self.pos_to_index(index.into())]
//     }
// }

// impl<T: Clone> IndexMut<IVec2> for SparseGrid<T> {
//     #[inline(always)]
//     fn index_mut(&mut self, index: IVec2) -> &mut Self::Output {
//         let index = self.pos_to_index(index.into());
//         &mut self.data[index]
//     }
// }
// impl<T: Clone> Index<UVec2> for SparseGrid<T> {
//     type Output = T;

//     #[inline(always)]
//     fn index(&self, index: UVec2) -> &Self::Output {
//         &self.data[self.upos_to_index(index.into())]
//     }
// }

// impl<T: Clone> IndexMut<UVec2> for SparseGrid<T> {
//     #[inline(always)]
//     fn index_mut(&mut self, index: UVec2) -> &mut Self::Output {
//         let index = self.upos_to_index(index.into());
//         &mut self.data[index]
//     }
// }

#[cfg(test)]
mod test {
    use glam::{UVec2};

    use crate::SparseGrid;

    #[test]
    fn insert_row() {
        let mut grid = SparseGrid::new( [10,10] );

        grid.insert_row(5, "Hello".chars());

        assert_eq!(5, grid.len());

        let str: String = grid.iter().map(|(_,v)| v).collect();

        assert_eq!("Hello", str);

        for (i, (p,_)) in grid.iter().enumerate() {
            assert_eq!((i as u32, 5), p.into());
        }
    }

    #[test]
    fn insert_row_at() {
        let mut grid = SparseGrid::new( [10,10] );

        grid.insert_row_at([3,3], "Hello".chars());

        assert_eq!(5, grid.len());

        let str: String = grid.iter().map(|(_,v)| v).collect();

        assert_eq!("Hello", str);

        let kvp: Vec<_> = grid.iter().collect();

        assert_eq!((UVec2::new(3,3), &'H'), kvp[0]);
        assert_eq!((UVec2::new(4,3), &'e'), kvp[1]);
        assert_eq!((UVec2::new(5,3), &'l'), kvp[2]);
        assert_eq!((UVec2::new(6,3), &'l'), kvp[3]);
        assert_eq!((UVec2::new(7,3), &'o'), kvp[4]);
    }

    #[test]
    fn insert_col() {
        let mut grid = SparseGrid::new( [10,10] );

        grid.insert_col(5, "Hello".chars());

        assert_eq!(5, grid.len());

        let str: String = grid.iter().map(|(_,v)| v).collect();

        assert_eq!("Hello", str);

        for (i, (p,_)) in grid.iter().enumerate() {
            assert_eq!((5, i as u32, ), p.into());
        }
    }

    #[test]
    fn insert_col_at() {
        let mut grid = SparseGrid::new( [10,10] );

        grid.insert_col_at([3,3], "Hello".chars());

        assert_eq!(5, grid.len());

        let str: String = grid.iter().map(|(_,v)| v).collect();

        assert_eq!("Hello", str);

        let kvp: Vec<_> = grid.iter().collect();

        assert_eq!((UVec2::new(3,3), &'H'), kvp[0]);
        assert_eq!((UVec2::new(3,4), &'e'), kvp[1]);
        assert_eq!((UVec2::new(3,5), &'l'), kvp[2]);
        assert_eq!((UVec2::new(3,6), &'l'), kvp[3]);
        assert_eq!((UVec2::new(3,7), &'o'), kvp[4]);
    }

}