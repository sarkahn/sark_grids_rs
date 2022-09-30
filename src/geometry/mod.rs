//! Utilities for building geometric shapes on a grid.
mod grid_circle;
mod grid_cone;
mod grid_line;
mod grid_rect;
use glam::IVec2;

use self::grid_circle::GridCircleIter;
use self::grid_circle::GridCircleOutlineIter;
use self::grid_cone::GridConeIter;
use self::grid_line::GridLineIter;
use self::grid_line::GridLineOrthoIter;
use self::grid_rect::GridRectIter;

pub use grid_circle::GridCircle;
pub use grid_circle::GridCircleOutline;
pub use grid_line::GridLine;
pub use grid_line::GridLineOrtho;
pub use grid_rect::GridRect;

pub trait GridShape: Sync + Send + 'static {
    fn iter(&self) -> GridShapeIterator;
    fn pos(&self) -> IVec2;
    fn set_pos(&mut self, pos: IVec2);
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
        }
    }
}
