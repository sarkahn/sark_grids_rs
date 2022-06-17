//! Utilities for building shapes on a grid.
use glam::IVec2;

mod grid_rect;
mod grid_line;
mod grid_circle;

pub use grid_rect::GridRect;
pub use grid_line::GridLine;
pub use grid_line::GridLineOrthogonal;
pub use grid_circle::GridCircleOutline;
pub use grid_circle::GridCircleFilled;

/// A trait for iterating over the grid points of geometric shapes.
pub trait GridShape {
    type Iterator: Iterator<Item=IVec2>;
    fn iter(&self) -> Self::Iterator;
}