//! A utility for translating between grid points and world space.
//!
//! You can specify a world position, size, and pivot for the grid when creating it.
//! These will affect the grid's bounds and tile points.
//!
//! **Example Grids**
//!
//! A 3x3 Pivot::Center grid:
//! ```text
//! |-1,-1| 0, 1| 1, 1|
//! |-1, 0| 0, 0| 1, 0|
//! |-1,-1| 0,-1| 1,-1|
//! ```
//!
//! A 3x3 Pivot::BottomLeft grid:
//! ```text
//! | 0, 2| 1, 2| 2, 2|
//! | 0, 1| 1, 1| 2, 1|
//! | 0, 0| 1, 0| 2, 0|
//! ```
//!
//! A 3x3 Pivot::TopRight grid:
//!
//! ```text
//! |-3,-1|-2,-1|-1,-1|
//! |-3,-2|-2,-2|-1,-2|
//! |-3,-3|-2,-3|-1,-3|
//! ```
//!
//! # Types of Points
//!
//! The api has several different "types" of positions that can be
//! referenced:
//! - **World Position**: The world offset of the grid.
//! - **Grid Position**: The local position of the grid's tiles relative to it's pivot.
//! - **Index**: The 1d index of a tile. (`0`..`width * height`)
//! - **2d Index**: The 2d index of a tile in the grid. `([0,0]`..`[width,height])`.
//! - **Tile Position**: The bottom-left point of a tile in the grid.
//! - **Tile Center**: The center of a tile in the grid.

use itertools::Itertools;

use glam::{IVec2, Vec2, UVec2};

use crate::point::{GridPoint, Size2d, Point2d};

/// A sized grid with a custom pivot for translating between aligned grid points
/// and world space.
#[derive(Default, Debug, Clone)]
pub struct WorldGrid {
    size: UVec2,
    /// Used when retrieving a tile center - accounts for centered/odd size grids.
    center_offset: Vec2,
    /// Used when retrieving a tile position - accounts for centered/odd sized grids.
    pos_offset: Vec2,
    /// Used when translating from a grid position to an index position.
    pivot_offset: Vec2,
    /// Axis, derived from pivot
    axis: Vec2,
}

impl WorldGrid {
    pub fn new(size: impl Size2d, pivot: Pivot) -> Self {
        let size = size.as_ivec2();

        let center_offset = match pivot {
            Pivot::Center => Vec2::select(
                (size % 2).cmpeq(IVec2::ZERO),
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

        WorldGrid {
            size: size.as_uvec2(),
            center_offset,
            pivot_offset,
            pos_offset,
            axis,
        }
    }

    /// Returns the tile position of a given tile from a given grid point.
    ///
    /// A tile's "position" refers to the bottom left point on the tile.
    #[inline]
    pub fn tile_pos_from_index2d(&self, grid_pos: impl GridPoint) -> Vec2 {
        grid_pos.as_vec2() + self.pos_offset
    }

    /// Returns the center point of a given tile.
    #[inline]
    pub fn tile_center_from_index2d(&self, grid_pos: impl GridPoint) -> Vec2 {
        grid_pos.as_vec2() + self.center_offset
    }

    /// Whether or not the given grid position is inside the grid bounds.
    ///
    /// A grid's bounds are determined by it's pivot - a grid's pivot always
    /// sits on the world new.
    #[inline]
    pub fn grid_pos_in_bounds(&self, grid_pos: impl GridPoint) -> bool {
        self.try_grid_to_index_2d(grid_pos).is_some()
    }

    /// Whether or not the given 2d index is inside the grid bounds.
    #[inline]
    pub fn index_2d_in_bounds(&self, index: impl GridPoint) -> bool {
        let index = index.as_ivec2();
        index.cmpge(IVec2::ZERO).all() && index.cmplt(self.size().as_ivec2()).all()
    }

    /// Convert a grid point to it's corresponding 2d index.
    ///
    /// Returns none if the given grid point is out of bounds.
    #[inline]
    pub fn try_grid_to_index_2d(&self, grid_pos: impl GridPoint) -> Option<IVec2> {
        let center = self.tile_center_from_index2d(grid_pos);
        let index = center * self.axis - self.pivot_offset;

        if index.cmpge(Vec2::ZERO).all() && index.cmplt(self.size.as_vec2()).all() {
            return Some(index.as_ivec2());
        };
        None
    }

    /// Converts from a local grid position to it's corresponding 2d index.
    ///
    /// This function will return out of bounds values if given out of bounds grid positions.
    /// For a bound-checked version use `try_grid_to_index_2d`
    #[inline]
    pub fn grid_to_index_2d(&self, grid_pos: impl GridPoint) -> IVec2 {
        let center = self.tile_center_from_index2d(grid_pos);
        let index = center * self.axis - self.pivot_offset;
        index.as_ivec2()
    }

    /// Convert from a 2d index to it's corresponding grid position.
    #[inline]
    pub fn index_2d_to_grid(&self, i: impl GridPoint) -> IVec2 {
        let p = i.as_vec2();
        let p = (self.pivot_offset - self.pos_offset) + p;
        let p = p + self.center_offset;
        let p = (p * self.axis).floor();
        p.as_ivec2()
    }

    /// Convert from a an arbitrary local position to the tile center
    /// at that position's tile.
    #[inline]
    pub fn point_to_tile_center(&self, point: impl Point2d) -> Vec2 {
        let xy = point.as_vec2();
        xy.floor() + self.center_offset
    }

    /// Convert a world position to it's local position on the grid 
    /// (it's position relative to the grid's pivot).
    #[inline]
    pub fn world_to_local(&self, point: impl Point2d) -> Vec2 {
        point.as_vec2() * self.axis
    }

    #[inline]
    pub fn local_to_world(&self, point: impl Point2d) -> Vec2 {
        point.as_vec2() * self.axis
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
    use super::*;

    #[test]
    fn center_iter_odd() {
        let grid = WorldGrid::new([3, 3], Pivot::Center);
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
        let grid = WorldGrid::new([10, 10], Pivot::Center);

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
        let grid = WorldGrid::new([5, 5], Pivot::BottomLeft);

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
        let grid = WorldGrid::new([5, 5], Pivot::TopLeft);

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
        let grid = WorldGrid::new([5, 5], Pivot::BottomRight);

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
        let grid = WorldGrid::new([5, 5], Pivot::Center);

        assert!(!grid.grid_pos_in_bounds([-3, -3]));
        assert!(grid.grid_pos_in_bounds([-2, -2]));
        assert!(grid.grid_pos_in_bounds([-1, -1]));
        assert!(grid.grid_pos_in_bounds([0, 0]));
        assert!(grid.grid_pos_in_bounds([1, 1]));
        assert!(grid.grid_pos_in_bounds([2, 2]));
    }

    #[test]
    fn bottom_left_bounds() {
        let grid = WorldGrid::new([5, 5], Pivot::BottomLeft);

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
        let grid = WorldGrid::new([5, 5], Pivot::TopRight);

        assert!(!grid.grid_pos_in_bounds([1, 1]));
        assert!(!grid.grid_pos_in_bounds([0, 0]));
        assert!(!grid.grid_pos_in_bounds([0, -1]));
        assert!(grid.grid_pos_in_bounds([-1, -2]));
        assert!(grid.grid_pos_in_bounds([-2, -3]));
        assert!(grid.grid_pos_in_bounds([-4, -4]));
    }

    #[test]
    fn top_right_grid_to_index_2d() {
        let grid = WorldGrid::new([5, 5], Pivot::TopRight);
        assert_eq!([0, 0], grid.grid_to_index_2d([-1, -1]).to_array());
        assert_eq!([1, 1], grid.grid_to_index_2d([-2, -2]).to_array());
        assert_eq!([2, 2], grid.grid_to_index_2d([-3, -3]).to_array());
    }

    #[test]
    fn top_right_grid_to_grid() {
        let grid = WorldGrid::new([5, 5], Pivot::TopRight);

        assert_eq!([-1, -1], grid.index_2d_to_grid([0, 0]).to_array());
        assert_eq!([-2, -2], grid.index_2d_to_grid([1, 1]).to_array());
        assert_eq!([-3, -3], grid.index_2d_to_grid([2, 2]).to_array());
    }

    #[test]
    fn top_left_grid_to_grid() {
        let grid = WorldGrid::new([5, 5], Pivot::TopLeft);

        assert_eq!([0, -1], grid.index_2d_to_grid([0, 0]).to_array());
        assert_eq!([1, -2], grid.index_2d_to_grid([1, 1]).to_array());
        assert_eq!([2, -3], grid.index_2d_to_grid([2, 2]).to_array());
    }

    #[test]
    fn bottom_left_grid_to_grid() {
        let grid = WorldGrid::new([5, 5], Pivot::BottomLeft);

        assert_eq!([0, 0], grid.index_2d_to_grid([0, 0]).to_array());
    }

    #[test]
    fn top_left_grid_to_index_2d() {
        let grid = WorldGrid::new([5, 5], Pivot::TopLeft);
        assert_eq!([0, 0], grid.grid_to_index_2d([0, -1]).to_array());
        assert_eq!([1, 1], grid.grid_to_index_2d([1, -2]).to_array());
        assert_eq!([2, 2], grid.grid_to_index_2d([2, -3]).to_array());
    }

    #[test]
    fn bottom_left_grid_to_index_2d() {
        let grid = WorldGrid::new([5, 5], Pivot::BottomLeft);

        assert_eq!([0, 0], grid.grid_to_index_2d([0, 0]).to_array());
        assert_eq!([1, 1], grid.grid_to_index_2d([1, 1]).to_array());
        assert_eq!([2, 2], grid.grid_to_index_2d([2, 2]).to_array());
    }

    #[test]
    fn center_grid_to_index_2d() {
        let grid = WorldGrid::new([5, 5], Pivot::Center);
        assert_eq!([0, 0], grid.grid_to_index_2d([-2, -2]).to_array());
        assert_eq!([1, 1], grid.grid_to_index_2d([-1, -1]).to_array());
        assert_eq!([2, 2], grid.grid_to_index_2d([0, 0]).to_array());
        assert_eq!([3, 3], grid.grid_to_index_2d([1, 1]).to_array());
        assert_eq!([4, 4], grid.grid_to_index_2d([2, 2]).to_array());
    }

    #[test]
    fn center_grid_to_grid() {
        let grid = WorldGrid::new([5, 5], Pivot::Center);

        assert_eq!([-2, -2], grid.index_2d_to_grid([0, 0]).to_array());
        assert_eq!([-1, -1], grid.index_2d_to_grid([1, 1]).to_array());
        assert_eq!([0, 0], grid.index_2d_to_grid([2, 2]).to_array());
        assert_eq!([1, 1], grid.index_2d_to_grid([3, 3]).to_array());
        assert_eq!([2, 2], grid.index_2d_to_grid([4, 4]).to_array());
    }

    #[test]
    fn top_right_tile_center() {
        let grid = WorldGrid::new([3, 3], Pivot::TopRight);

        assert_eq!([-0.5, -0.5], grid.tile_center_from_index2d([-1, -1]).to_array());
        assert_eq!([-1.5, -1.5], grid.tile_center_from_index2d([-2, -2]).to_array());
    }

    #[test]
    fn top_left_tile_center() {
        let grid = WorldGrid::new([3, 3], Pivot::TopLeft);

        assert_eq!([0.5, -0.5], grid.tile_center_from_index2d([0, -1]).to_array());
        assert_eq!([1.5, -1.5], grid.tile_center_from_index2d([1, -2]).to_array());
    }

    #[test]
    fn center_tile_center_odd() {
        let grid = WorldGrid::new([3, 3], Pivot::Center);

        assert_eq!([0.0, 0.0], grid.tile_center_from_index2d([0, 0]).to_array());
        assert_eq!([-1.0, -1.0], grid.tile_center_from_index2d([-1, -1]).to_array());
    }

    #[test]
    fn center_tile_center_even() {
        let grid = WorldGrid::new([4, 4], Pivot::Center);

        assert_eq!([0.5, 0.5], grid.tile_center_from_index2d([0, 0]).to_array());
        assert_eq!([1.5, 1.5], grid.tile_center_from_index2d([1, 1]).to_array());
    }

    #[test]
    fn center_tile_pos_odd() {
        let grid = WorldGrid::new([3, 3], Pivot::Center);

        assert_eq!([-0.5, -0.5], grid.tile_pos_from_index2d([0, 0]).to_array());
        assert_eq!([-1.5, -1.5], grid.tile_pos_from_index2d([-1, -1]).to_array());
    }

    #[test]
    fn center_tile_pos_even() {
        let grid = WorldGrid::new([4, 4], Pivot::Center);

        assert_eq!([0.0, 0.0], grid.tile_pos_from_index2d([0, 0]).to_array());
        assert_eq!([-1.0, -1.0], grid.tile_pos_from_index2d([-1, -1]).to_array());
    }

    #[test]
    fn to_tile_center() {
        let grid = WorldGrid::new([4,4], Pivot::Center);

        assert_eq!([0.5,0.5], grid.point_to_tile_center([0.75,0.75]).to_array());
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
    pub fn pivot_aligned_point(&self, point: impl GridPoint, size: impl Size2d) -> IVec2 {
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
