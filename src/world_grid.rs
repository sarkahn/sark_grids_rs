use glam::{IVec2, UVec2, Vec2};
use itertools::Itertools;

use crate::{point::Point2d, GridPoint, Pivot};

/// A sized grid which can be used to translate world positions to
/// tile positions based on [`WorldSpace`] and the size of the grid.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WorldGrid {
    /// The [`WorldSpace`] for this grid.
    pub world_space: WorldSpace,
    /// How many pixels constitute a "tile" in the grid.
    pub pixels_per_tile: UVec2,
    /// How many tiles the grid has.
    pub tile_count: UVec2,
}

impl WorldGrid {
    /// Create a [`WorldGrid`] set to [`WorldSpace::Units`].
    pub fn unit_grid(tile_count: impl GridPoint, pixels_per_tile: impl GridPoint) -> Self {
        Self {
            world_space: WorldSpace::Units,
            pixels_per_tile: pixels_per_tile.as_uvec2(),
            tile_count: tile_count.as_uvec2(),
        }
    }

    /// Create a [`WorldGrid`] set to [`WorldSpace::Pixels`].
    pub fn pixel_grid(tile_count: impl GridPoint, pixels_per_tile: impl GridPoint) -> Self {
        Self {
            world_space: WorldSpace::Pixels,
            pixels_per_tile: pixels_per_tile.as_uvec2(),
            tile_count: tile_count.as_uvec2(),
        }
    }

    /// The size of a tile in world space, given `pixels_per_tile` and `world_space`.
    pub fn tile_size_world(&self) -> Vec2 {
        let [x, y] = self.pixels_per_tile.as_vec2().to_array();
        match self.world_space {
            WorldSpace::Units => Vec2::new(x / y, 1.0),
            WorldSpace::Pixels => Vec2::new(x, y),
        }
    }

    /// Convert a position to it's corresponding tile index.
    #[inline]
    pub fn pos_to_index(&self, pos: impl Point2d) -> IVec2 {
        let pos = pos.as_vec2();
        let pos = pos / self.tile_size_world();
        (pos + self.center_offset()).floor().as_ivec2()
    }

    /// Try to get the corresponding tile index of a given position.
    ///
    /// Returns none if the position is out of grid bounds.
    #[inline]
    pub fn get_pos_to_index(&self, pos: impl Point2d) -> Option<IVec2> {
        let i = self.pos_to_index(pos);
        match self.index_in_bounds(i) {
            true => Some(i),
            false => None,
        }
    }

    /// Convert a tile index to it's corresponding position.
    ///
    /// The returned position is the bottom left of the tile.
    #[inline]
    pub fn index_to_pos(&self, pos: impl GridPoint) -> Vec2 {
        match self.world_space {
            WorldSpace::Units => pos.as_vec2() - self.center_offset(),
            WorldSpace::Pixels => {
                let offset = self.center_offset() * self.pixels_per_tile.as_vec2();
                let pos = pos.as_vec2() * self.pixels_per_tile.as_vec2();
                pos - offset
            }
        }
    }

    /// How large the grid is in world space, given the [`WorldSpace`] of the grid.
    pub fn world_size(&self) -> Vec2 {
        self.tile_count.as_vec2() * self.tile_size_world()
    }

    /// Convert a position to it's tile position in the grid.
    pub fn pos_to_tile_pos(&self, pos: impl Point2d) -> Vec2 {
        pos.as_vec2() + self.center_offset()
    }

    /// Return the world center of the tile at the given index.
    #[inline]
    pub fn index_to_tile_center(&self, index: impl GridPoint) -> Vec2 {
        let pos = index.as_vec2() + self.center_offset();
        pos * self.tile_size_world()
    }

    /// Return the tile position of a pivot point given the size of the grid.
    #[inline]
    pub fn pivot_pos(&self, pivot: Pivot) -> Vec2 {
        let pivot = Vec2::from(pivot) - Vec2::splat(0.5);
        self.tile_count.as_vec2() * pivot
    }

    #[inline]
    pub fn pivot_pos_world(&self, pivot: Pivot) -> Vec2 {
        self.pivot_pos(pivot) * self.tile_size_world()
    }

    #[inline]
    pub fn index_in_bounds(&self, index: impl GridPoint) -> bool {
        let size = self.tile_count.as_ivec2();
        let i = index.as_ivec2() + size / 2;
        i.cmpge(IVec2::ZERO).all() && i.cmplt(size).all()
    }

    #[inline]
    pub fn pos_in_bounds(&self, pos: impl Point2d) -> bool {
        self.index_in_bounds(self.pos_to_index(pos))
    }

    /// An iterator over the position of every tile in the grid.
    ///
    /// A tile's "position" refers to the bottom left point on the tile.
    pub fn tile_pos_iter(&self) -> impl Iterator<Item = Vec2> {
        let start = self.pivot_pos(Pivot::BottomLeft);
        let tile_size = self.tile_size_world();
        (0..self.tile_count.y)
            .cartesian_product(0..self.tile_count.x)
            .map(move |(y, x)| {
                let xy = Vec2::new(x as f32, y as f32);
                (start + xy) * tile_size
            })
    }

    /// An iterator over the center of every tile in the grid.
    pub fn tile_center_iter(&self) -> impl Iterator<Item = Vec2> {
        let start = self.pivot_pos(Pivot::BottomLeft) + 0.5;
        let tile_size = self.tile_size_world();
        (0..self.tile_count.y)
            .cartesian_product(0..self.tile_count.x)
            .map(move |(y, x)| {
                let xy = Vec2::new(x as f32, y as f32);
                (start + xy) * tile_size
            })
        ////let pivot_offset = -self.tile_count.as_vec2() / 2.0 + (0.5 - self.center_offset());
        // let start = self.pivot_pos(Pivot::BottomLeft) + (self.tile_size_world() * 0.5);
        // println!("START {}", start);
        // let world_space_scale = self.tile_size_world();
        // (0..self.tile_count.y)
        //     .cartesian_product(0..self.tile_count.x)
        //     .map(move |(y, x)| {
        //         let xy = Vec2::new(x as f32,y as f32);
        //         (start + xy) * world_space_scale
        //     })
    }

    /// The center of a world grid always sits on origin, so odd sized
    /// grids will have all their tile centers offset by 0.5.
    #[inline]
    fn center_offset(&self) -> Vec2 {
        let axis_even = (self.tile_count.as_ivec2() % 2).cmpeq(IVec2::ZERO);
        Vec2::select(axis_even, Vec2::ZERO, Vec2::splat(0.5))
    }
}

/// How world space is defined.
///
/// This is used when converting between positions and tile indices.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WorldSpace {
    /// With this setting the size of all tiles is exactly one world unit.
    ///
    /// When world space is defined by world units, `pixels_per_tile` determines
    /// how many pixels fit vertically in a single world unit.
    Units,
    /// With this setting the size of all tiles in world units is equal to
    /// `pixels_per_tile`.
    ///
    /// This matches the defaults for bevy's built in orthographic camera
    /// where one world unit == one pixel.
    Pixels,
}

impl WorldSpace {
    /// Return the opposite of this [`WorldSpace`].
    pub fn other(&self) -> Self {
        match self {
            WorldSpace::Units => WorldSpace::Pixels,
            WorldSpace::Pixels => WorldSpace::Units,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Pivot;

    use super::WorldGrid;

    #[test]
    fn pixel_iter() {
        let grid = WorldGrid::pixel_grid([3, 3], [8, 8]);

        let mut iter = grid.tile_pos_iter();
        assert_eq!([-12.0, -12.0], iter.next().unwrap().to_array());
        assert_eq!([4.0, 4.0], iter.last().unwrap().to_array());

        let mut iter = grid.tile_center_iter();
        assert_eq!([-8.0, -8.0], iter.next().unwrap().to_array());
        assert_eq!([8.0, 8.0], iter.last().unwrap().to_array());
    }

    #[test]
    fn bounds() {
        let grid = WorldGrid::unit_grid([3, 3], [8, 8]);
        assert_eq!(true, grid.pos_in_bounds([-1.5, 0.0]));
        assert_eq!(false, grid.pos_in_bounds([-1.6, 0.0]));

        let grid = WorldGrid::unit_grid([2, 2], [8, 8]);
        assert_eq!(true, grid.pos_in_bounds([-1.0, 0.0]));
        assert_eq!(false, grid.pos_in_bounds([-1.1, 0.0]));
    }

    #[test]
    fn corners() {
        let grid = WorldGrid::unit_grid([4, 4], [8, 8]);
        let bl = grid.pivot_pos(Pivot::BottomLeft).to_array();
        let tr = grid.pivot_pos(Pivot::TopRight).to_array();
        assert_eq!([-2.0, -2.0], bl);
        assert_eq!([2.0, 2.0], tr);

        let grid = WorldGrid::unit_grid([3, 3], [8, 8]);
        let bl = grid.pivot_pos(Pivot::BottomLeft).to_array();
        let tr = grid.pivot_pos(Pivot::TopRight).to_array();
        assert_eq!([-1.5, -1.5], bl);
        assert_eq!([1.5, 1.5], tr);

        let grid = WorldGrid::pixel_grid([3, 3], [8, 8]);
        let bl = grid.pivot_pos_world(Pivot::BottomLeft).to_array();
        let tr = grid.pivot_pos_world(Pivot::TopRight).to_array();
        assert_eq!([-12.0, -12.0], bl);
        assert_eq!([12.0, 12.0], tr);

        let grid = WorldGrid::pixel_grid([4, 4], [8, 8]);
        let bl = grid.pivot_pos_world(Pivot::BottomLeft).to_array();
        let tr = grid.pivot_pos_world(Pivot::TopRight).to_array();
        assert_eq!([-16.0, -16.0], bl);
        assert_eq!([16.0, 16.0], tr);
    }

    #[test]
    fn pos_to_tile_pos() {
        let grid = WorldGrid::unit_grid([5, 5], [8, 8]);
        let p = grid.pos_to_tile_pos([0.0, 0.0]);
        assert_eq!([0.5, 0.5], p.to_array());

        let grid = WorldGrid::unit_grid([4, 4], [8, 8]);
        let p = grid.pos_to_tile_pos([0.0, 0.0]);
        assert_eq!([0.0, 0.0], p.to_array());
    }

    #[test]
    fn world_iter_center_odd() {
        let grid = WorldGrid::unit_grid([3, 3], [8, 8]);

        let points: Vec<_> = grid.tile_center_iter().map(|p| p.to_array()).collect();

        assert_eq!([-1.0, -1.0], points[0]);
        assert_eq!([0.0, -1.0], points[1]);
        assert_eq!([1.0, -1.0], points[2]);
        assert_eq!([-1.0, 0.0], points[3]);
        assert_eq!([0.0, 0.0], points[4]);
        assert_eq!([1.0, 0.0], points[5]);
        assert_eq!([-1.0, 1.0], points[6]);
        assert_eq!([0.0, 1.0], points[7]);
        assert_eq!([1.0, 1.0], points[8]);
    }

    #[test]
    fn world_iter_center_even() {
        let grid = WorldGrid::unit_grid([2, 2], [8, 8]);

        let points: Vec<_> = grid.tile_center_iter().map(|p| p.to_array()).collect();

        assert_eq!([-0.5, -0.5], points[0]);
        assert_eq!([0.5, -0.5], points[1]);
        assert_eq!([-0.5, 0.5], points[2]);
        assert_eq!([0.5, 0.5], points[3]);
    }

    #[test]
    fn rect_tiles() {
        let grid = WorldGrid::unit_grid([2, 2], [4, 8]);

        let points: Vec<_> = grid.tile_center_iter().map(|p| p.to_array()).collect();

        assert_eq!([-0.25, -0.5], points[0]);
        assert_eq!([0.25, -0.5], points[1]);
        assert_eq!([-0.25, 0.5], points[2]);
        assert_eq!([0.25, 0.5], points[3]);
    }
}
