use glam::{IVec2, Vec2};

use crate::{GridPoint, Size2d};

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
    /// Coordinate axis for adjusting an aligned position on a 2d rect.
    pub fn axis(&self) -> IVec2 {
        match self {
            Pivot::TopLeft => IVec2::new(1, -1),
            Pivot::TopRight => IVec2::new(-1, -1),
            Pivot::Center => IVec2::new(1, 1),
            Pivot::BottomLeft => IVec2::new(1, 1),
            Pivot::BottomRight => IVec2::new(-1, 1),
        }
    }

    #[inline]
    /// Transform a point to it's equivalent from the perspective of
    /// this pivot.
    pub fn pivot_aligned_point(&self, point: impl GridPoint, size: impl Size2d) -> IVec2 {
        let axis = Vec2::from(*self);
        let align = (size.as_vec2() - Vec2::ONE) * axis;
        //let align_offset = size.as_vec2() - Vec2::ONE;
        //let align_offset = (align_offset * Vec2::from(*self)).as_ivec2();

        point.as_ivec2() * self.axis() + align.as_ivec2() //align_offset
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
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PivotedPoint {
    pub point: IVec2,
    pub pivot: Pivot,
}

impl PivotedPoint {
    /// Returns the point from the perspective of the pivot.
    pub fn pivot_point(&self) -> IVec2 {
        self.point
    }

    pub fn pivot(&self) -> Pivot {
        self.pivot
    }
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

    /// Retrieve the pivot aligned point.
    #[inline]
    fn get_aligned_point(&self, size: impl Size2d) -> IVec2 {
        self.pivot.pivot_aligned_point(self.point, size)
    }

    fn get_pivot(self) -> PivotedPoint {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pivot_point() {
        let p = [0, 0].pivot(Pivot::TopRight);
        assert_eq!([9, 9], p.get_aligned_point([10, 10]).to_array());

        let p = [3, 3].pivot(Pivot::TopLeft);
        assert_eq!([3, 6], p.get_aligned_point([10, 10]).to_array());
    }
}
