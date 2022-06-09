//! Traits for more easily dealing with the various types to represent 2d points/sizes

use glam::{IVec2, UVec2, Vec2};

use crate::Pivot;

/// A trait for an integer point on a 2d grid.
pub trait GridPoint: Clone + Copy {
    fn x(&self) -> i32;
    fn y(&self) -> i32;

    fn as_ivec2(&self) -> IVec2 {
        IVec2::new(self.x(), self.y())
    }
    fn as_uvec2(&self) -> UVec2 {
        self.as_ivec2().as_uvec2()
    }
    fn as_vec2(&self) -> Vec2 {
        self.as_ivec2().as_vec2()
    }
    fn to_array(&self) -> [i32; 2] {
        self.as_ivec2().to_array()
    }
}

macro_rules! impl_grid_point {
    ($type:ty) => {
        impl GridPoint for $type {
            fn x(&self) -> i32 {
                self[0] as i32
            }

            fn y(&self) -> i32 {
                self[1] as i32
            }
        }
    };
}

impl GridPoint for IVec2 {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

impl GridPoint for [i32; 2] {
    fn x(&self) -> i32 {
        self[0]
    }

    fn y(&self) -> i32 {
        self[1]
    }
}

impl_grid_point!(UVec2);
impl_grid_point!([u32; 2]);

#[allow(clippy::len_without_is_empty)]
/// A trait for mixing of the different types representing a 2d size.
pub trait Size2d: Clone + Copy {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn as_uvec2(&self) -> UVec2 {
        UVec2::new(self.width() as u32, self.height() as u32)
    }

    fn len(&self) -> usize {
        self.width() * self.height()
    }

    fn as_vec2(&self) -> Vec2 {
        self.as_uvec2().as_vec2()
    }

    fn as_ivec2(&self) -> IVec2 {
        self.as_uvec2().as_ivec2()
    }
    fn to_array(&self) -> [usize; 2] {
        [self.width(), self.height()]
    }
}

macro_rules! impl_size2d {
    ($type:ty) => {
        impl Size2d for $type {
            fn width(&self) -> usize {
                self[0] as usize
            }

            fn height(&self) -> usize {
                self[1] as usize
            }
        }
    };
}

impl_size2d!(IVec2);
impl_size2d!(UVec2);
impl_size2d!([u32; 2]);
impl_size2d!([i32; 2]);

/// A trait for an arbitrary 2d point.
pub trait Point2d {
    fn x(&self) -> f32;
    fn y(&self) -> f32;

    fn as_ivec2(&self) -> IVec2 {
        self.as_vec2().as_ivec2()
    }
    fn as_uvec2(&self) -> UVec2 {
        self.as_vec2().as_uvec2()
    }
    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x(), self.y())
    }
    fn to_array(&self) -> [f32; 2] {
        self.as_vec2().to_array()
    }
}

impl Point2d for Vec2 {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl Point2d for [f32; 2] {
    fn x(&self) -> f32 {
        self[0]
    }

    fn y(&self) -> f32 {
        self[1]
    }
}

macro_rules! impl_point2d {
    ($type:ty) => {
        impl Point2d for $type {
            fn x(&self) -> f32 {
                self[0] as f32
            }

            fn y(&self) -> f32 {
                self[1] as f32
            }
        }
    };
}

impl_point2d!(IVec2);
impl_point2d!(UVec2);
impl_point2d!([u32; 2]);
impl_point2d!([i32; 2]);

pub struct PivotedPoint {
    point: IVec2,
    pivot: Pivot,
}