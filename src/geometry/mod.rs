//! Utilities for building geometric shapes on a grid.
mod grid_circle;
mod grid_cone;
mod grid_diamond;
mod grid_line;
mod grid_rect;
use glam::IVec2;

use self::grid_circle::GridCircleIter;
use self::grid_circle::GridCircleOutlineIter;
use self::grid_cone::GridConeIter;
use self::grid_diamond::GridDiamondIter;
use self::grid_line::GridLineIter;
use self::grid_line::GridLineOrthoIter;
use self::grid_rect::GridRectIter;

pub use grid_circle::GridCircle;
pub use grid_circle::GridCircleOutline;
pub use grid_cone::GridCone;
pub use grid_diamond::GridDiamond;
pub use grid_line::GridLine;
pub use grid_line::GridLineOrtho;
pub use grid_rect::GridRect;

pub trait GridShape: ShapeClone + Sync + Send + 'static {
    fn iter(&self) -> GridShapeIterator;
    fn pos(&self) -> IVec2;
    fn set_pos(&mut self, pos: IVec2);
    /// Get a rect encompassing the entire shape
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
