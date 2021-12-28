use std::{slice::{Iter, IterMut}, iter::StepBy};

use glam::{IVec2, UVec2};

use crate::grid::{Iter2d, Iter2dMut};

pub trait GridReader<T: Clone> {
    fn get(&self, i: usize) -> &T;
    fn get_from_pos(&self, i: (i32, i32));
    fn get_from_upos(&self, i: (u32, u32));
    fn index_to_pos(&self, i: usize) -> IVec2;
    fn index_to_upos(&self, i: usize) -> UVec2;
    fn row_iter(&self) -> Iter<T>;
    fn column_iter(&self) -> StepBy<Iter<T>>;
    fn iter(&self) -> Iter<T>;
    fn iter_2d(&self) -> Iter2d<T>;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn size(&self) -> UVec2;
    fn len(&self) -> usize;
    fn is_in_bounds(&self) -> usize;
    fn left_index(&self) -> usize;
    fn right_index(&self) -> usize;
    fn top_index(&self) -> usize;
    fn bottom_index(&self) -> usize;
}

pub trait GridWriter<T: Clone> {
    fn set(&mut self, i: usize) -> &T;
    fn set_from_pos(&mut self, i: (i32, i32));
    fn set_from_upos(&mut self, i: (u32, u32));
    fn iter_mut(&self) -> IterMut<T>;
    fn iter_mut_2d(&self) -> Iter2dMut<T>;
    fn row_iter_mut(&self) -> IterMut<T>;
    fn column_iter_mut(&self) -> StepBy<IterMut<T>>;
}