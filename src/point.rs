use glam::{IVec2, UVec2, Vec2};

/// A trait for easier mixing of the different types representing a 2d point.
pub trait Point2d: Clone + Copy {
    fn as_ivec2(&self) -> IVec2;

    fn x(&self) -> i32 {
        self.as_ivec2().x
    }
    fn y(&self) -> i32 {
        self.as_ivec2().y
    }
    fn as_uvec2(&self) -> UVec2 {
        self.as_ivec2().as_uvec2()
    }
    fn as_vec2(&self) -> Vec2 {
        self.as_ivec2().as_vec2()
    }
    fn to_array(&self) -> [i32;2] {
        self.as_ivec2().to_array()
    }
}

impl Point2d for IVec2 {
    fn as_ivec2(&self) -> IVec2 {
        *self
    }
}

impl Point2d for [i32; 2] {
    fn as_ivec2(&self) -> IVec2 {
        IVec2::from(*self)
    }
}

impl Point2d for [u32; 2] {
    fn as_ivec2(&self) -> IVec2 {
        UVec2::from(*self).as_ivec2()
    }
}

#[allow(clippy::len_without_is_empty)]
/// A trait for mixing of the different types representing a 2d size.
pub trait Size2d: Clone + Copy {
    fn as_uvec2(&self) -> UVec2;

    fn width(&self) -> usize {
        self.as_uvec2().x as usize
    }
    fn height(&self) -> usize {
        self.as_uvec2().y as usize
    }
    fn len(&self) -> usize {
        self.width() * self.height()
    }

    fn as_vec2(&self) -> Vec2 {
        self.as_ivec2().as_vec2()
    }

    fn as_ivec2(&self) -> IVec2 {
        self.as_uvec2().as_ivec2()
    }
    fn to_array(&self) -> [usize;2] {
        [self.width(), self.height()]
    }
}

impl Size2d for [u32; 2] {
    fn as_uvec2(&self) -> UVec2 {
        UVec2::new(self[0], self[1])
    }

    fn to_array(&self) -> [usize;2] {
        [self[0] as usize, self[1] as usize]
    }
}

impl Size2d for UVec2 {
    fn as_uvec2(&self) -> UVec2 {
        *self
    }
}

impl Size2d for IVec2 {
    fn as_uvec2(&self) -> UVec2 {
        (*self).as_uvec2()
    }
}

/// A pivot point on a 2d rect.
#[derive(Eq, PartialEq, Clone, Copy)]
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

    /// Transform a point to it's equivalent from the perspective of
    /// a pivot on a 2d rect.
    pub fn pivot_aligned_point(&self, point: impl Point2d, size: impl Size2d) -> IVec2 {
        let point = point.as_ivec2();
        let align_offset = size.as_ivec2().as_vec2() - Vec2::ONE;
        let align_offset = (align_offset * Vec2::from(*self)).as_ivec2();

        point * self.axis() + align_offset
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
