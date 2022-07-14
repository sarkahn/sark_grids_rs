//! Utilities for building geometric shapes on a grid.
use glam::IVec2;

use self::grid_circle::EmptyCircleIterator;
use self::grid_circle::FilledCircleIterator;
use self::grid_line::LineIter;
use self::grid_line::LineOrthogonalIter;
use self::grid_rect::GridRectIter;

pub mod grid_circle;
pub mod grid_line;
pub mod grid_rect;

pub use grid_circle::GridCircleFilled;
pub use grid_circle::GridCircleOutline;
pub use grid_line::GridLine;
pub use grid_line::GridLineOrthogonal;
pub use grid_rect::GridRect;

/// A trait for iterating over the grid points of geometric shapes.
pub trait GridShape: Sync + Send + 'static {
    /// Iterate over all points contained in the shape.
    fn iter(&self) -> ShapeIterator;
}

pub enum ShapeIterator {
    EmptyCircle(IVec2, EmptyCircleIterator),
    FilledCircle(IVec2, FilledCircleIterator),
    Rect(IVec2, GridRectIter),
    Line(IVec2, LineIter),
    LineOrtho(IVec2, LineOrthogonalIter),
}

impl Iterator for ShapeIterator {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        let (pos, next) = match self {
            ShapeIterator::EmptyCircle(p, i) => (p, i.next()),
            ShapeIterator::FilledCircle(p, i) => (p, i.next()),
            ShapeIterator::Rect(p, i) => (p, i.next()),
            ShapeIterator::Line(p, i) => (p, i.next()),
            ShapeIterator::LineOrtho(p, i) => (p, i.next()),
        };
        next.map(|p| p + *pos)
    }
}
