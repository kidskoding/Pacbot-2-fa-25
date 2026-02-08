// location.rs - Position and direction tracking for entities

use serde::{Deserialize, Serialize};
use crate::direction::Direction;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LocationState {
    pub row: i8,
    pub col: i8,
    pub dir: Direction,
}

impl LocationState {
    pub fn new(row: i8, col: i8, dir: Direction) -> Self {
        Self { row, col, dir }
    }

    /// Copy all values from another location
    pub fn copy_from(&mut self, other: &LocationState) {
        self.row = other.row;
        self.col = other.col;
        self.dir = other.dir;
    }

    /// Check if this location collides with another
    /// Locations with row/col >= 32 are considered "empty" and don't collide
    pub fn collides_with(&self, other: &LocationState) -> bool {
        if self.row >= 32 || self.col >= 32 || other.row >= 32 || other.col >= 32 {
            return false;
        }
        self.row == other.row && self.col == other.col
    }

    /// Check if this location is the empty/invalid location (32, 32)
    pub fn is_empty(&self) -> bool {
        self.row == 32 && self.col == 32
    }

    /// Get coordinates as a tuple
    pub fn get_coords(&self) -> (i8, i8) {
        (self.row, self.col)
    }

    /// Get the coordinates of a neighboring cell in the given direction
    pub fn get_neighbor_coords(&self, dir: Direction) -> (i8, i8) {
        (self.row + dir.get_drow(), self.col + dir.get_dcol())
    }

    /// Get coordinates a number of steps ahead in the current facing direction
    pub fn get_ahead_coords(&self, spaces: i8) -> (i8, i8) {
        (
            self.row + self.dir.get_drow() * spaces,
            self.col + self.dir.get_dcol() * spaces,
        )
    }

    /// Set this location to one step ahead of another location
    pub fn advance_from(&mut self, other: &LocationState) {
        let (row, col) = other.get_ahead_coords(1);
        self.row = row;
        self.col = col;
        self.dir = other.dir;
    }

    /// Get the reversed direction of this location
    pub fn get_reversed_dir(&self) -> Direction {
        self.dir.reverse()
    }

    /// Update just the coordinates
    pub fn update_coords(&mut self, row: i8, col: i8) {
        self.row = row;
        self.col = col;
    }

    /// Update just the direction
    pub fn update_dir(&mut self, dir: Direction) {
        self.dir = dir;
    }
}
