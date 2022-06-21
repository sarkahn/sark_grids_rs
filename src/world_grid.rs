use glam::{IVec2, UVec2, Vec2};
use itertools::Itertools;

use crate::{point::Point2d, GridPoint, Pivot, Size2d};

/// A sized grid which can be used to translate world positions to
/// tile positions based on [`WorldSpace`] and the size of the grid.
pub struct WorldGrid {
    /// The [`WorldSpace`] for this grid.
    pub world_space: WorldSpace,
    /// How many pixels constitute a "tile" in the grid.
    pub pixels_per_tile: u32,
    /// How many tiles the grid has.
    pub tile_count: UVec2,
}

impl WorldGrid {
    /// Create a [`WorldGrid`] set to [`WorldSpace::Units`].
    pub fn unit_grid(tile_count: impl Size2d, pixels_per_tile: u32) -> Self {
        Self {
            world_space: WorldSpace::Units,
            pixels_per_tile,
            tile_count: tile_count.as_uvec2(),
        }
    }

    /// Create a [`WorldGrid`] set to [`WorldSpace::Pixels`].
    pub fn pixel_grid(tile_count: impl Size2d, pixels_per_tile: u32) -> Self {
        Self {
            world_space: WorldSpace::Pixels,
            pixels_per_tile,
            tile_count: tile_count.as_uvec2(),
        }
    }

    /// Convert a position to it's corresponding tile index.
    #[inline]
    pub fn pos_to_index(&self, pos: impl Point2d) -> IVec2 {
        let pos = pos.as_vec2();
        let pos = pos / self.world_space_scale();
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
                let offset = self.center_offset() * self.pixels_per_tile as f32;
                let pos = pos.as_vec2() * self.pixels_per_tile as f32;
                pos - offset
            }
        }
    }

    /// How large the grid is in world space, given the [`WorldSpace`] of the grid.
    pub fn world_size(&self) -> UVec2 {
        self.tile_count * self.world_space_scale() as u32
    }

    /// Return the world center of the tile at the given index.
    #[inline]
    pub fn index_to_tile_center(&self, index: impl GridPoint) -> Vec2 {
        let pos = index.as_vec2() + self.center_offset();
        pos * self.world_space_scale()
    }

    /// Return the position of a pivot point on the grid.
    #[inline]
    pub fn pivot_pos(&self, pivot: Pivot) -> Vec2 {
        let pivot = Vec2::from(pivot) - Vec2::splat(0.5);
        let pos = self.tile_count.as_vec2() * pivot;
        pos * self.world_space_scale()
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
        let pivot_offset = -self.tile_count.as_vec2() / 2.0 + (0.5 - self.center_offset());
        let world_space_scale = self.world_space_scale();
        (0..self.tile_count.x)
            .cartesian_product(0..self.tile_count.y)
            .map(move |(x, y)| world_space_scale * (Vec2::new(x as f32, y as f32) + pivot_offset))
    }

    /// An iterator over the center of every tile in the grid.
    pub fn tile_center_iter(&self) -> impl Iterator<Item = Vec2> {
        let pivot_offset = -self.tile_count.as_vec2() / 2.0 + self.center_offset();
        let world_space_scale = self.world_space_scale();
        (0..self.tile_count.x)
            .cartesian_product(0..self.tile_count.y)
            .map(move |(x, y)| world_space_scale * (Vec2::new(x as f32, y as f32) + pivot_offset))
    }

    /// The center of a world grid always sits on origin, so odd sized
    /// grids will have all their tile centers offset by 0.5.
    #[inline]
    fn center_offset(&self) -> Vec2 {
        let axis_even = (self.tile_count % 2).cmpeq(UVec2::ZERO);
        Vec2::select(axis_even, Vec2::ZERO, Vec2::splat(0.5))
    }

    /// Returns a value that can be used to convert indices to positions
    /// according to [`WorldSpace`].
    #[inline]
    fn world_space_scale(&self) -> f32 {
        match self.world_space {
            WorldSpace::Units => 1.0,
            WorldSpace::Pixels => self.pixels_per_tile as f32,
        }
    }
}

/// How world space is defined.
///
/// This is used by [`WorldGrid`] when converting between points and
/// tile indices.
pub enum WorldSpace {
    /// World space is defined by world units. `pixels_per_tile` determines
    /// how many pixels fit vertically in a single world unit.
    ///
    /// With this setting the size of all tiles is exactly one world unit.
    Units,
    /// World space is defined in pixels. All position->index conversions
    /// will be scaled by `pixels_per_unit`.
    ///
    /// With this setting the size of all tiles in world units is equal to
    /// the `pixels_per_tile` of the [`WorldGrid`].
    ///
    /// This matches the defaults for bevy's built in orthographic camera.
    Pixels,
}

#[cfg(test)]
mod tests {
    use crate::Pivot;

    use super::WorldGrid;

    #[test]
    fn pixel_iter() {
        let grid = WorldGrid::pixel_grid([3, 3], 8);

        let mut iter = grid.tile_pos_iter();
        assert_eq!([-12.0, -12.0], iter.next().unwrap().to_array());
        assert_eq!([4.0, 4.0], iter.last().unwrap().to_array());

        let mut iter = grid.tile_center_iter();
        assert_eq!([-8.0, -8.0], iter.next().unwrap().to_array());
        assert_eq!([8.0, 8.0], iter.last().unwrap().to_array());
    }

    #[test]
    fn bounds() {
        let grid = WorldGrid::unit_grid([3, 3], 8);
        assert_eq!(true, grid.pos_in_bounds([-1.5, 0.0]));
        assert_eq!(false, grid.pos_in_bounds([-1.6, 0.0]));

        let grid = WorldGrid::unit_grid([2, 2], 8);
        assert_eq!(true, grid.pos_in_bounds([-1.0, 0.0]));
        assert_eq!(false, grid.pos_in_bounds([-1.1, 0.0]));
    }

    #[test]
    fn corners() {
        let grid = WorldGrid::unit_grid([4, 4], 8);
        let bl = grid.pivot_pos(Pivot::BottomLeft).to_array();
        let tr = grid.pivot_pos(Pivot::TopRight).to_array();
        assert_eq!([-2.0, -2.0], bl);
        assert_eq!([2.0, 2.0], tr);

        let grid = WorldGrid::unit_grid([3, 3], 8);
        let bl = grid.pivot_pos(Pivot::BottomLeft).to_array();
        let tr = grid.pivot_pos(Pivot::TopRight).to_array();
        assert_eq!([-1.5, -1.5], bl);
        assert_eq!([1.5, 1.5], tr);

        let grid = WorldGrid::pixel_grid([3, 3], 8);
        let bl = grid.pivot_pos(Pivot::BottomLeft).to_array();
        let tr = grid.pivot_pos(Pivot::TopRight).to_array();
        assert_eq!([-12.0, -12.0], bl);
        assert_eq!([12.0, 12.0], tr);

        let grid = WorldGrid::pixel_grid([4, 4], 8);
        let bl = grid.pivot_pos(Pivot::BottomLeft).to_array();
        let tr = grid.pivot_pos(Pivot::TopRight).to_array();
        assert_eq!([-16.0, -16.0], bl);
        assert_eq!([16.0, 16.0], tr);
    }
}
