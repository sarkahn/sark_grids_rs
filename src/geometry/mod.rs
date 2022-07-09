//! Utilities for building geometric shapes on a grid.
use glam::IVec2;

pub mod grid_circle;
pub mod grid_line;
pub mod grid_rect;

pub use grid_circle::GridCircleFilled;
pub use grid_circle::GridCircleOutline;
pub use grid_line::GridLine;
pub use grid_line::GridLineOrthogonal;
pub use grid_rect::GridRect;

use self::grid_circle::EmptyCircleIterator;
use self::grid_circle::FilledCircleIterator;
use self::grid_line::LineIter;
use self::grid_line::LineOrthogonalIter;
use self::grid_rect::GridRectIter;

/// A trait for iterating over the grid points of geometric shapes.
pub trait GridShape {
    type Iterator: Iterator<Item = IVec2>;
    fn iter(&self) -> Self::Iterator;
}

pub enum ShapeIter {
    EmptyCircle(EmptyCircleIterator),
    FilledCircle(FilledCircleIterator),
    Rect(GridRectIter),
    Line(LineIter),
    LineOrtho(LineOrthogonalIter),
}

impl Iterator for ShapeIter {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ShapeIter::EmptyCircle(i) => i.next(),
            ShapeIter::FilledCircle(i) => i.next(),
            ShapeIter::Rect(i) => i.next(),
            ShapeIter::Line(i) => i.next(),
            ShapeIter::LineOrtho(i) => i.next(),
        }
    }
}
