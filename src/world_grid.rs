//! A utility for translating to/from grid points and world space.

use itertools::Itertools;

use glam::{IVec2, UVec2, Vec2};

use crate::Pivot;

/// A sized grid with a custom pivot for translating aligned grid points
/// into world space.
///
/// You can specify a world position, size, and pivot for the grid when creating it.
/// These will affect the grid's bounds and tile points.
///
/// For a [Pivot::BottomLeft] grid the
///
/// **IE:**
///
/// |-1,-1| 0, 1| 1, 1|
///
/// |-1, 0| 0, 0| 1, 0|
///
/// |-1,-1| 0,-1| 1,-1|
pub struct WorldGrid {
    pub world_pos: Vec2,
    size: UVec2,
    /// Used when retrieving a tile center - accounts for centered grids.
    center_offset: Vec2,
    /// Used when retrieving a tile position - accounts for centered grids.
    pos_offset: Vec2,
    /// Used when translating from a grid position to an index position.
    pivot_offset: Vec2,
    /// Axis, derived from pivot
    axis: Vec2,
}

impl WorldGrid {
    pub fn new(world_pos: (f32, f32), size: [u32; 2], pivot: Pivot) -> Self {
        let world_pos = Vec2::from(world_pos);
        let size = UVec2::from(size);

        let center_offset = match pivot {
            Pivot::Center => Vec2::select(
                (size % 2).cmpeq(UVec2::ZERO),
                Vec2::new(0.5, 0.5),
                Vec2::ZERO,
            ),
            _ => Vec2::new(0.5, 0.5),
        };

        let pos_offset = center_offset - Vec2::new(0.5, 0.5);

        let axis = pivot.axis().as_vec2();

        let pivot_offset = match pivot {
            Pivot::Center => -size.as_vec2() * Vec2::new(0.5, 0.5),
            _ => Vec2::ZERO,
        };

        //println!("New grid size {}, center_offset {}, axis {}, pivot_offset {}", size, center_offset, axis, pivot_offset);

        WorldGrid {
            world_pos,
            size,
            center_offset,
            pivot_offset,
            pos_offset,
            axis,
        }
    }

    /// Create a grid with it's world position set to the origin.
    pub fn origin(size: [u32; 2], pivot: Pivot) -> Self {
        WorldGrid::new((0.0, 0.0), size, pivot)
    }

    /// Returns the tile position of a given tile.
    ///
    /// A tile's "position" refers to the bottom left point on the tile.
    #[inline]
    pub fn tile_pos(&self, grid_pos: [i32; 2]) -> Vec2 {
        let grid_pos = IVec2::from(grid_pos).as_vec2();
        grid_pos + self.pos_offset
    }

    /// Returns the center point of a given tile.
    #[inline]
    pub fn tile_center(&self, grid_pos: [i32; 2]) -> Vec2 {
        let grid_pos = IVec2::from(grid_pos).as_vec2();
        grid_pos + self.center_offset
    }

    /// Returns the position of the given tile in world space.
    ///
    /// A tile's "position" refers to the bottom left point on the tile.
    #[inline]
    pub fn tile_pos_world(&self, grid_pos: [i32; 2]) -> Vec2 {
        self.tile_pos(grid_pos) + self.world_pos
    }

    /// Return's the center of the given tile in world space.
    #[inline]
    pub fn tile_center_world(&self, grid_pos: [i32; 2]) -> Vec2 {
        self.tile_center(grid_pos) + self.world_pos
    }

    /// Whether or not the given grid position is inside the grid bounds.
    ///
    /// A grid's bounds are determined by it's pivot - a grid's pivot always
    /// sits on the world origin.
    #[inline]
    pub fn grid_pos_in_bounds(&self, grid_pos: [i32; 2]) -> bool {
        self.try_grid_to_index_2d(grid_pos).is_some()
    }
    /// Whether or not the given grid position is inside the grid bounds.
    ///
    /// A grid's bounds are determined by it's pivot - a grid's pivot always
    /// sits on the world origin.
    #[inline]
    pub fn index_2d_in_bounds(&self, index: (u32, u32)) -> bool {
        let index = UVec2::from(index);
        index.cmpge(UVec2::ZERO).all() && index.cmplt(self.size()).all()
    }

    /// Convert a grid point to it's corresponding 2d index.
    ///
    /// Returns none if the given grid point is out of bounds.
    pub fn try_grid_to_index_2d(&self, grid_pos: [i32; 2]) -> Option<UVec2> {
        let center = self.tile_center(grid_pos);
        let index = center * self.axis - self.pivot_offset;

        if index.cmpge(Vec2::ZERO).all() && index.cmplt(self.size.as_vec2()).all() {
            return Some(index.as_uvec2());
        };
        None
    }

    /// Converts from a local grid position to it's corresponding 2d index.
    ///
    /// This function will return invalid values if given out of bounds grid positions.
    /// For a bound-checked version use `try_grid_to_index_2d`
    pub fn grid_to_index_2d(&self, grid_pos: [i32; 2]) -> UVec2 {
        let center = self.tile_center(grid_pos);
        let index = center * self.axis - self.pivot_offset;
        index.as_uvec2()
    }

    /// Convert from a 2d index to it's corresponding grid position.
    pub fn index_2d_to_grid(&self, i: [u32; 2]) -> IVec2 {
        let p = UVec2::from(i).as_vec2();
        let p = (self.pivot_offset - self.pos_offset) + p;
        let p = p + self.center_offset;
        let p = (p * self.axis).floor();
        p.as_ivec2()
    }

    pub fn width(&self) -> usize {
        self.size.x as usize
    }

    pub fn height(&self) -> usize {
        self.size.y as usize
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    /// An iterator over the tile position of every tile in the grid.
    ///
    /// A tile's "position" refers to the bottom left point on the tile.
    pub fn tile_pos_iter(&self) -> impl Iterator<Item = Vec2> {
        self.iter(self.pivot_offset, self.axis)
    }

    /// An iterator over the tile center of every tile in the grid.
    pub fn tile_center_iter(&self) -> impl Iterator<Item = Vec2> {
        self.iter(self.pivot_offset + Vec2::new(0.5, 0.5), self.axis)
    }

    /// An iterator over the tile position of every tile in the grid.
    ///
    /// A tile's "position" refers to the bottom left point on the tile.
    pub fn tile_pos_iter_world(&self) -> impl Iterator<Item = Vec2> {
        let offset = self.world_pos;
        self.iter(self.pivot_offset + offset, self.axis)
    }

    /// Iterate over every tile in the grid, applying the given offset.
    #[inline]
    fn iter(&self, offset: Vec2, axis: Vec2) -> impl Iterator<Item = Vec2> {
        (0..self.height())
            .cartesian_product(0..self.width())
            .map(move |(y, x)| (Vec2::new(x as f32, y as f32) + offset) * axis)
    }
}

#[cfg(test)]
mod test {
    use crate::Pivot;

    use super::WorldGrid;

    #[test]
    fn center_iter_odd() {
        let grid = WorldGrid::origin([3, 3], Pivot::Center);
        let tiles: Vec<_> = grid.tile_center_iter().map(|p| (p.x, p.y)).collect();

        assert_eq!(tiles.len(), 9);
        assert_eq!(tiles[0], (-1.0, -1.0));
        assert_eq!(tiles[1], (0.0, -1.0));
        assert_eq!(tiles[2], (1.0, -1.0));
        assert_eq!(tiles[3], (-1.0, 0.0));
        assert_eq!(tiles[4], (0.0, 0.0));
        assert_eq!(tiles[8], (1.0, 1.0));
    }

    #[test]
    fn center_iter_even() {
        let grid = WorldGrid::origin([10, 10], Pivot::Center);

        let tiles: Vec<_> = grid.tile_center_iter().map(|p| (p.x, p.y)).collect();
        assert_eq!(tiles.len(), 100);
        assert_eq!(tiles[0], (-4.5, -4.5));
        assert_eq!(tiles[1], (-3.5, -4.5));
        assert_eq!(tiles[2], (-2.5, -4.5));
        assert_eq!(tiles[3], (-1.5, -4.5));
        assert_eq!(tiles[99], (4.5, 4.5));

        let tiles: Vec<_> = grid.tile_pos_iter().map(|p| (p.x, p.y)).collect();
        assert_eq!(tiles.len(), 100);
        assert_eq!(tiles[0], (-5.0, -5.0));
        assert_eq!(tiles[1], (-4.0, -5.0));
        assert_eq!(tiles[2], (-3.0, -5.0));
        assert_eq!(tiles[3], (-2.0, -5.0));
        assert_eq!(tiles[99], (4.0, 4.0));
    }

    #[test]
    fn bottom_left_iter_odd() {
        let grid = WorldGrid::origin([5, 5], Pivot::BottomLeft);

        let tiles: Vec<_> = grid.tile_center_iter().map(|p| (p.x, p.y)).collect();
        assert_eq!(tiles.len(), 25);
        assert_eq!(tiles[0], (0.5, 0.5));
        assert_eq!(tiles[1], (1.5, 0.5));
        assert_eq!(tiles[2], (2.5, 0.5));
        assert_eq!(tiles[3], (3.5, 0.5));
        assert_eq!(tiles[24], (4.5, 4.5));

        let tiles: Vec<_> = grid.tile_pos_iter().map(|p| (p.x, p.y)).collect();
        assert_eq!(tiles[0], (0.0, 0.0));
        assert_eq!(tiles[1], (1.0, 0.0));
        assert_eq!(tiles[2], (2.0, 0.0));
        assert_eq!(tiles[3], (3.0, 0.0));
        assert_eq!(tiles[24], (4.0, 4.0));
    }

    #[test]
    fn top_left_iter_odd() {
        let grid = WorldGrid::origin([5, 5], Pivot::TopLeft);

        let tiles: Vec<_> = grid.tile_center_iter().map(|p| (p.x, p.y)).collect();
        assert_eq!(tiles.len(), 25);
        assert_eq!(tiles[0], (0.5, -0.5));
        assert_eq!(tiles[1], (1.5, -0.5));
        assert_eq!(tiles[2], (2.5, -0.5));
        assert_eq!(tiles[3], (3.5, -0.5));
        assert_eq!(tiles[24], (4.5, -4.5));
    }

    #[test]
    fn bottom_right_iter_odd() {
        let grid = WorldGrid::origin([5, 5], Pivot::BottomRight);

        let tiles: Vec<_> = grid.tile_center_iter().map(|p| (p.x, p.y)).collect();
        assert_eq!(tiles.len(), 25);
        assert_eq!(tiles[0], (-0.5, 0.5));
        assert_eq!(tiles[1], (-1.5, 0.5));
        assert_eq!(tiles[2], (-2.5, 0.5));
        assert_eq!(tiles[3], (-3.5, 0.5));
        assert_eq!(tiles[24], (-4.5, 4.5));
    }

    #[test]
    fn center_bounds() {
        let grid = WorldGrid::origin([5, 5], Pivot::Center);

        assert!(!grid.grid_pos_in_bounds([-3, -3]));
        assert!(grid.grid_pos_in_bounds([-2, -2]));
        assert!(grid.grid_pos_in_bounds([-1, -1]));
        assert!(grid.grid_pos_in_bounds([0, 0]));
        assert!(grid.grid_pos_in_bounds([1, 1]));
        assert!(grid.grid_pos_in_bounds([2, 2]));
    }

    #[test]
    fn bottom_left_bounds() {
        let grid = WorldGrid::origin([5, 5], Pivot::BottomLeft);

        assert!(!grid.grid_pos_in_bounds([-1, -1]));
        assert!(grid.grid_pos_in_bounds([0, 0]));
        assert!(grid.grid_pos_in_bounds([1, 1]));
        assert!(grid.grid_pos_in_bounds([2, 2]));
        assert!(grid.grid_pos_in_bounds([3, 3]));
        assert!(grid.grid_pos_in_bounds([4, 4]));
        assert!(!grid.grid_pos_in_bounds([5, 5]));
    }

    #[test]
    fn top_right_bounds() {
        let grid = WorldGrid::origin([5, 5], Pivot::TopRight);

        assert!(!grid.grid_pos_in_bounds([1, 1]));
        assert!(!grid.grid_pos_in_bounds([0, 0]));
        assert!(!grid.grid_pos_in_bounds([0, -1]));
        assert!(grid.grid_pos_in_bounds([-1, -2]));
        assert!(grid.grid_pos_in_bounds([-2, -3]));
        assert!(grid.grid_pos_in_bounds([-4, -4]));
    }

    #[test]
    fn top_right_grid_to_index_2d() {
        let grid = WorldGrid::origin([5, 5], Pivot::TopRight);
        assert_eq!([0, 0], grid.grid_to_index_2d([-1, -1]).to_array());
        assert_eq!([1, 1], grid.grid_to_index_2d([-2, -2]).to_array());
        assert_eq!([2, 2], grid.grid_to_index_2d([-3, -3]).to_array());
    }

    #[test]
    fn top_right_grid_to_grid() {
        let grid = WorldGrid::origin([5, 5], Pivot::TopRight);

        assert_eq!([-1, -1], grid.index_2d_to_grid([0, 0]).to_array());
        assert_eq!([-2, -2], grid.index_2d_to_grid([1, 1]).to_array());
        assert_eq!([-3, -3], grid.index_2d_to_grid([2, 2]).to_array());
    }

    #[test]
    fn top_left_grid_to_grid() {
        let grid = WorldGrid::origin([5, 5], Pivot::TopLeft);

        assert_eq!([0, -1], grid.index_2d_to_grid([0, 0]).to_array());
        assert_eq!([1, -2], grid.index_2d_to_grid([1, 1]).to_array());
        assert_eq!([2, -3], grid.index_2d_to_grid([2, 2]).to_array());
    }

    #[test]
    fn bottom_left_grid_to_grid() {
        let grid = WorldGrid::origin([5, 5], Pivot::BottomLeft);

        assert_eq!([0, 0], grid.index_2d_to_grid([0, 0]).to_array());
    }

    #[test]
    fn top_left_grid_to_index_2d() {
        let grid = WorldGrid::origin([5, 5], Pivot::TopLeft);
        assert_eq!([0, 0], grid.grid_to_index_2d([0, -1]).to_array());
        assert_eq!([1, 1], grid.grid_to_index_2d([1, -2]).to_array());
        assert_eq!([2, 2], grid.grid_to_index_2d([2, -3]).to_array());
    }

    #[test]
    fn bottom_left_grid_to_index_2d() {
        let grid = WorldGrid::origin([5, 5], Pivot::BottomLeft);

        assert_eq!([0, 0], grid.grid_to_index_2d([0, 0]).to_array());
        assert_eq!([1, 1], grid.grid_to_index_2d([1, 1]).to_array());
        assert_eq!([2, 2], grid.grid_to_index_2d([2, 2]).to_array());
    }

    #[test]
    fn center_grid_to_index_2d() {
        let grid = WorldGrid::origin([5, 5], Pivot::Center);
        assert_eq!([0, 0], grid.grid_to_index_2d([-2, -2]).to_array());
        assert_eq!([1, 1], grid.grid_to_index_2d([-1, -1]).to_array());
        assert_eq!([2, 2], grid.grid_to_index_2d([0, 0]).to_array());
        assert_eq!([3, 3], grid.grid_to_index_2d([1, 1]).to_array());
        assert_eq!([4, 4], grid.grid_to_index_2d([2, 2]).to_array());
    }

    #[test]
    fn center_grid_to_grid() {
        let grid = WorldGrid::origin([5, 5], Pivot::Center);

        assert_eq!([-2, -2], grid.index_2d_to_grid([0, 0]).to_array());
        assert_eq!([-1, -1], grid.index_2d_to_grid([1, 1]).to_array());
        assert_eq!([0, 0], grid.index_2d_to_grid([2, 2]).to_array());
        assert_eq!([1, 1], grid.index_2d_to_grid([3, 3]).to_array());
        assert_eq!([2, 2], grid.index_2d_to_grid([4, 4]).to_array());
    }

    #[test]
    fn top_right_tile_center() {
        let grid = WorldGrid::origin([3, 3], Pivot::TopRight);

        assert_eq!([-0.5, -0.5], grid.tile_center([-1, -1]).to_array());
        assert_eq!([-1.5, -1.5], grid.tile_center([-2, -2]).to_array());
    }

    #[test]
    fn top_left_tile_center() {
        let grid = WorldGrid::origin([3, 3], Pivot::TopLeft);

        assert_eq!([0.5, -0.5], grid.tile_center([0, -1]).to_array());
        assert_eq!([1.5, -1.5], grid.tile_center([1, -2]).to_array());
    }

    #[test]
    fn center_tile_center_odd() {
        let grid = WorldGrid::origin([3, 3], Pivot::Center);

        assert_eq!([0.0, 0.0], grid.tile_center([0, 0]).to_array());
        assert_eq!([-1.0, -1.0], grid.tile_center([-1, -1]).to_array());
    }

    #[test]
    fn center_tile_center_even() {
        let grid = WorldGrid::origin([4, 4], Pivot::Center);

        assert_eq!([0.5, 0.5], grid.tile_center([0, 0]).to_array());
        assert_eq!([1.5, 1.5], grid.tile_center([1, 1]).to_array());
    }

    #[test]
    fn center_tile_pos_odd() {
        let grid = WorldGrid::origin([3, 3], Pivot::Center);

        assert_eq!([-0.5, -0.5], grid.tile_pos([0, 0]).to_array());
        assert_eq!([-1.5, -1.5], grid.tile_pos([-1, -1]).to_array());
    }

    #[test]
    fn center_tile_pos_even() {
        let grid = WorldGrid::origin([4, 4], Pivot::Center);

        assert_eq!([0.0, 0.0], grid.tile_pos([0, 0]).to_array());
        assert_eq!([-1.0, -1.0], grid.tile_pos([-1, -1]).to_array());
    }
}
