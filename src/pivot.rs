use std::ops::{Mul, Sub};

use glam::{IVec2, Vec2};

use crate::GridPoint;

/// A pivot point on a 2d rect.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Pivot {
    /// +X Right, +Y Down.
    TopLeft,
    /// +X Left, +Y Down.
    TopRight,
    /// +X Right, +Y Up.
    Center,
    /// +X Right, +Y Up.
    BottomLeft,
    /// +X Left, +Y Up
    BottomRight,
}

impl Pivot {
    /// Coordinate axis for each pivot, used when transforming a point into
    /// the pivot's coordinate space
    pub fn axis(&self) -> IVec2 {
        match self {
            Pivot::TopLeft => IVec2::new(1, -1),
            Pivot::TopRight => IVec2::new(-1, -1),
            Pivot::Center => IVec2::new(1, 1),
            Pivot::BottomLeft => IVec2::new(1, 1),
            Pivot::BottomRight => IVec2::new(-1, 1),
        }
    }

    // #[inline]
    // fn transform_point(&self, point: impl GridPoint) -> IVec2 {
    //     point.as_ivec2() * self.axis()
    // }

    /// Transform a point to it's equivalent position from the perspective
    /// of this pivot
    #[inline]
    pub fn transform_point(&self, point: impl GridPoint, size: impl GridPoint) -> IVec2 {
        let origin = size.as_vec2().sub(1.0).mul(Vec2::from(*self));
        let point = point.as_ivec2() * self.axis();
        origin.round().as_ivec2() + point
    }
}

impl From<Pivot> for Vec2 {
    fn from(p: Pivot) -> Self {
        match p {
            Pivot::TopLeft => Vec2::new(0.0, 1.0),
            Pivot::TopRight => Vec2::new(1.0, 1.0),
            Pivot::Center => Vec2::new(0.5, 0.5),
            Pivot::BottomLeft => Vec2::new(0.0, 0.0),
            Pivot::BottomRight => Vec2::new(1.0, 0.0),
        }
    }
}

/// A 2d point on a rect aligned to a certain [Pivot].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PivotedPoint {
    pub point: IVec2,
    pub pivot: Pivot,
}

impl GridPoint for PivotedPoint {
    #[inline]
    fn x(&self) -> i32 {
        self.point.x
    }

    #[inline]
    fn y(&self) -> i32 {
        self.point.y
    }

    // /// Retrieve the pivot aligned point.
    // #[inline]
    // fn get_aligned_point(&self, size: impl Size2d) -> IVec2 {
    //     if let Some(pivot) = self.pivot {
    //         todo!()
    //     } else {
    //         self.point
    //     }
    // }

    fn get_pivot(self) -> Option<Pivot> {
        Some(self.pivot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn pivot_point() {
    //     let p = [0, 0].pivot(Pivot::TopRight);
    //     assert_eq!([9, 9], p.get_aligned_point([10, 10]).to_array());

    //     let p = [3, 3].pivot(Pivot::TopLeft);
    //     assert_eq!([3, 6], p.get_aligned_point([10, 10]).to_array());
    // }

    #[test]
    fn transform_point() {
        let pivot = Pivot::TopRight;
        assert_eq!([8, 8], pivot.transform_point([1, 1], [10, 10]).to_array());
        assert_eq!([9, 9], pivot.transform_point([0, 0], [10, 10]).to_array());
        let pivot = Pivot::TopLeft;
        assert_eq!([1, 8], pivot.transform_point([1, 1], [10, 10]).to_array());
        let pivot = Pivot::BottomLeft;
        assert_eq!([1, 1], pivot.transform_point([1, 1], [10, 10]).to_array());
        let pivot = Pivot::BottomRight;
        assert_eq!([8, 1], pivot.transform_point([1, 1], [10, 10]).to_array());
    }
}
