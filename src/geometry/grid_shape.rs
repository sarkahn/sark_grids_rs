//! Shapes on a 2d grid.
use glam::IVec2;

use crate::GridRect;

use super::{
    grid_circle::{GridCircleIter, GridCircleOutlineIter},
    grid_cone::GridConeIter,
    grid_diamond::GridDiamondIter,
    grid_line::{GridLineIter, GridLineOrthoIter},
    grid_rect::GridRectIter,
};

/// A trait for accessing the points of shapes on a grid.
pub trait GridShape: ShapeClone + Sync + Send + 'static {
    /// Iterate over all the points contained within the grid shape.
    fn iter(&self) -> GridShapeIterator;
    /// The position of the grid shape.
    fn pos(&self) -> IVec2;
    /// Set the position of the grid shape without changing it's size.
    fn set_pos(&mut self, pos: IVec2);
    /// Get a rect encompassing the entire shape.
    fn bounds(&self) -> GridRect;
}

#[derive(Debug, Clone)]
pub enum GridShapeIterator {
    Point(std::iter::Once<IVec2>),
    Circle(GridCircleIter),
    CircleOutline(GridCircleOutlineIter),
    Rect(GridRectIter),
    Line(GridLineIter),
    LineOrtho(GridLineOrthoIter),
    Cone(GridConeIter),
    Diamond(GridDiamondIter),
}

impl Iterator for GridShapeIterator {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            GridShapeIterator::Point(i) => i.next(),
            GridShapeIterator::Circle(i) => i.next(),
            GridShapeIterator::CircleOutline(i) => i.next(),
            GridShapeIterator::Rect(i) => i.next(),
            GridShapeIterator::Line(i) => i.next(),
            GridShapeIterator::LineOrtho(i) => i.next(),
            GridShapeIterator::Cone(i) => i.next(),
            GridShapeIterator::Diamond(i) => i.next(),
        }
    }
}

pub trait ShapeClone {
    fn clone_box(&self) -> Box<dyn GridShape>;
}

impl<T> ShapeClone for T
where
    T: 'static + GridShape + Clone,
{
    fn clone_box(&self) -> Box<dyn GridShape> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn GridShape> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for Box<dyn GridShape> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GridShade {{ pos {:?}, points {:?} }}",
            self.pos(),
            self.iter().collect::<Vec<IVec2>>()
        )
    }
}

impl PartialEq for Box<dyn GridShape> {
    fn eq(&self, other: &Self) -> bool {
        self.pos() == other.pos() && self.iter().eq(other.iter())
    }
}
