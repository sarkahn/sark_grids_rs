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

/// A trait for iterating over the grid points of geometric shapes.
pub trait GridShape {
    type Iterator: Iterator<Item = IVec2>;
    fn iter(&self) -> Self::Iterator;
}
