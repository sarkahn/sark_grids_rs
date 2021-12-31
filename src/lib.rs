pub mod grid;
pub mod world_grid;

pub use grid::Grid;
pub use world_grid::WorldGrid;

use glam::{IVec2, Vec2};

#[derive(Clone, Copy)]
pub enum Pivot {
    /// +Y Down, +X Right
    TopLeft,
    /// +Y Down, +X Left
    TopRight,
    /// +Y Up, +X Right
    Center,
    /// +Y Up, +X Right
    BottomLeft,
    /// +Y Up, +X Left
    BottomRight,
}

impl Pivot {
    pub fn normalized(&self) -> Vec2 {
        match self {
            Pivot::TopLeft => Vec2::new(0.0, 1.0),
            Pivot::TopRight => Vec2::new(1.0, 1.0),
            Pivot::Center => Vec2::new(0.5, 0.5),
            Pivot::BottomLeft => Vec2::new(0.0, 0.0),
            Pivot::BottomRight => Vec2::new(1.0, 0.0),
        }
    }

    pub fn axis(&self) -> IVec2 {
        match self {
            Pivot::TopLeft => IVec2::new(1, -1),
            Pivot::TopRight => IVec2::new(-1, -1),
            Pivot::Center => IVec2::new(1, 1),
            Pivot::BottomLeft => IVec2::new(1, 1),
            Pivot::BottomRight => IVec2::new(-1, 1),
        }
    }
}
