//! A read-only iterator over a rectangular portion of the grid.

use std::ops::{Index, RangeBounds, Bound};

use glam::{IVec2, UVec2};

use crate::grid::{Grid, Pivot};

/// A read-only view into a rectangular portion of the grid.
pub struct GridView<'a, T: Clone> {
    min: IVec2,
    size: IVec2,
    grid: &'a Grid<T>,
}

impl<'a, T: Clone> GridView<'a, T> {
    pub fn new<RANGE: RangeBounds<(i32,i32)>>(range: RANGE, grid: &'a Grid<T>) -> GridView<'a, T> {
        let min = match range.start_bound() {
            Bound::Included( (x,y) ) => (*x,*y),
            Bound::Excluded( (x,y) ) => (x + 1, y + 1),
            Bound::Unbounded => (0,0),
        };
        let min = IVec2::from(min);
 
        let max = match range.end_bound() {
            Bound::Included((x,y)) => (*x,*y),
            Bound::Excluded((x,y)) => (x - 1, y - 1),
            Bound::Unbounded => grid.pivot_position(Pivot::TopRight).into(),
        };
        let max = IVec2::from(max);
        let size = max - min + IVec2::ONE;

        GridView {
            min,
            size,
            grid,
        }
    }

    pub fn iter(&'a self) -> GridViewIter<'a, T> {
        GridViewIter::new(self)
    }

    pub fn iter_2d(&'a self) -> GridViewIter2d<'a, T> {
        GridViewIter2d::new(self)
    }

    pub fn len(&self) -> usize {
        (self.size.x * self.size.y) as usize
    }

    #[inline(always)]
    /// Converts an index into it's local position in the view
    pub fn index_to_local(&self, i: usize) -> IVec2 {
        let i = i as i32;
        let x = i % self.size.x;
        let y = i / self.size.x;
        IVec2::new(x,y)
    }
    
    #[inline(always)]
    /// Converts a view-local position to it's corresponding position on the grid
    pub fn local_to_grid(&self, local: IVec2) -> IVec2 {
        self.min + local
    }

    /// Converts a view index to it's corresponding position on the grid.
    #[inline(always)]
    pub fn index_to_grid(&self, i: usize) -> IVec2 {
        self.local_to_grid(self.index_to_local(i))
    }

}

pub struct GridViewIter<'a, T: Clone> {
    index: usize,
    len: usize,
    view: &'a GridView<'a, T>,
}

impl<'a, T: Clone> GridViewIter<'a, T> {
    pub fn new(view: &'a GridView<'a, T>) -> Self {
        Self {
            index: 0,
            len: view.len(),
            view,
        }
    }
}

impl<'a, T: Clone> Iterator for GridViewIter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let xy = self.view.index_to_grid(self.index);
            self.index += 1;
            return Some(&self.view.grid[xy]);
        }

        None
    }
} 

impl<'a, T: Clone> ExactSizeIterator for GridViewIter<'a, T> {
    fn len(&self) -> usize {
        self.view.len() - self.index
    }
} 

pub struct GridViewIter2d<'a, T: Clone> {
    index: usize,
    len: usize,
    view: &'a GridView<'a, T>,
}

impl<'a, T: Clone> GridViewIter2d<'a, T> {
    pub fn new(view: &'a GridView<'a, T>) -> Self {
        Self {
            index: 0,
            len: view.len(),
            view,
        }
    }
}

impl<'a, T: Clone> Iterator for GridViewIter2d<'a, T> {
    type Item = (IVec2, &'a T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let xy = self.view.index_to_grid(self.index);
            self.index += 1;
            return Some((xy, &self.view.grid[xy]));
        }

        None
    }
} 

impl<'a, T: Clone> ExactSizeIterator for GridViewIter2d<'a, T> {
    fn len(&self) -> usize {
        self.view.len() - self.index
    }
} 

impl<'a, T: Clone> Index<(u32, u32)> for GridView<'a, T> {
    type Output = T;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        let local = UVec2::from(index).as_i32();
        &self.grid[self.local_to_grid(local)]
    }
}

impl<'a, T: Clone> Index<usize> for GridView<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[self.index_to_grid(index)]
    }
}