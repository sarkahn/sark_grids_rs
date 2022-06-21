//! Traits for more easily dealing with the various types to represent 2d points/sizes

use glam::{IVec2, UVec2, Vec2};

use crate::{pivot::PivotedPoint, Pivot};

/// A trait for types representing an integer point on a 2d grid.
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
    fn as_array(&self) -> [i32; 2] {
        self.as_ivec2().to_array()
    }
    #[inline]
    fn as_usize_array(&self) -> [usize; 2] {
        let p = self.as_uvec2();
        [p.x as usize, p.y as usize]
    }

    /// Get the grid point's corresponding 1d index.
    #[inline]
    fn as_index(&self, grid_width: usize) -> usize {
        self.y() as usize * grid_width + self.x() as usize
    }

    /// Return a [PivotedPoint].
    fn pivot(&self, pivot: Pivot) -> PivotedPoint {
        PivotedPoint {
            point: self.as_ivec2(),
            pivot,
        }
    }

    /// Returns the grid point above this one.
    fn up(&self) -> IVec2 {
        IVec2::new(self.x(), self.y() + 1)
    }

    /// Returns the grid point the given number of spaces above this one.
    fn up_by(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x(), self.y() + amount)
    }

    /// Returns the grid point below this one.
    fn down(&self) -> IVec2 {
        IVec2::new(self.x(), self.y() - 1)
    }

    /// Returns the grid point the given number of spaces below this one.
    fn down_by(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x(), self.y() - amount)
    }

    /// Returns the grid point to the right of this one.
    fn right(&self) -> IVec2 {
        IVec2::new(self.x() + 1, self.y())
    }

    /// Returns the grid point the given number of spaces to the right of this one.
    fn right_by(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x() + amount, self.y())
    }

    /// Returns the grid point to the left of this one.
    fn left(&self) -> IVec2 {
        IVec2::new(self.x() - 1, self.y())
    }

    /// Returns the grid point the given number of spaces to the left of this one.
    fn left_by(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x() - amount, self.y())
    }

    /// Retrieve the pivot-aligned point on the grid.
    ///
    /// If no pivot has been applied this will simply return the point
    /// directly.
    fn aligned(&self, size: impl Size2d) -> IVec2;

    #[inline]
    /// The [taxicab distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two grid points.
    fn taxi_dist(self, other: impl GridPoint) -> usize {
        let d = self.as_ivec2() - other.as_ivec2();
        (d.x.abs() + d.y.abs()) as usize
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

            /// Returns the point directly - no pivot has been applied.
            fn aligned(&self, _size: impl Size2d) -> IVec2 {
                IVec2::new(self[0] as i32, self[1] as i32)
            }
        }
    };
}

impl_grid_point!(IVec2);
impl_grid_point!(UVec2);
impl_grid_point!([u32; 2]);
impl_grid_point!([i32; 2]);
impl_grid_point!([usize; 2]);

/// A trait for types representing a 2d size.
#[allow(clippy::len_without_is_empty)]
pub trait Size2d: Clone + Copy {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    #[inline]
    fn as_uvec2(&self) -> UVec2 {
        UVec2::new(self.width() as u32, self.height() as u32)
    }

    #[inline]
    fn len(&self) -> usize {
        self.width() * self.height()
    }

    #[inline]
    fn as_vec2(&self) -> Vec2 {
        self.as_uvec2().as_vec2()
    }

    #[inline]
    fn as_ivec2(&self) -> IVec2 {
        self.as_uvec2().as_ivec2()
    }
    #[inline]
    fn as_array(&self) -> [usize; 2] {
        [self.width(), self.height()]
    }
    #[inline]
    fn as_usize_array(&self) -> [usize; 2] {
        let p = self.as_uvec2();
        [p.x as usize, p.y as usize]
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
impl_size2d!([usize; 2]);

/// A trait for types representing arbitrary 2d point.
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
    fn as_array(&self) -> [f32; 2] {
        self.as_vec2().to_array()
    }
    fn as_usize_array(&self) -> [usize; 2] {
        let p = self.as_uvec2();
        [p.x as usize, p.y as usize]
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

impl_point2d!(Vec2);
impl_point2d!(IVec2);
impl_point2d!(UVec2);
impl_point2d!([u32; 2]);
impl_point2d!([i32; 2]);
impl_point2d!([f32; 2]);
impl_point2d!([usize; 2]);

#[cfg(test)]
mod tests {
    use crate::GridPoint;

    #[test]
    fn taxi() {
        let a = [10, 10];
        let b = [20, 20];

        let dist = GridPoint::taxi_dist(a, b);
        assert_eq!(dist, 20);
    }
}
