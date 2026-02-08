// direction.rs - Direction enum for PacBot movement

use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
    None,
}

pub const NUM_DIRS: u8 = 4;

// Direction deltas: [Up, Left, Down, Right, None]
pub const D_ROW: [i8; 5] = [-1, 0, 1, 0, 0];
pub const D_COL: [i8; 5] = [0, -1, 0, 1, 0];

impl Direction {
    pub const DIRS: [(i8, i8); 5] = [
        (-1, 0),  // Up
        (0, -1),  // Left
        (1, 0),   // Down
        (0, 1),   // Right
        (0, 0),   // None
    ];

    /// Get the (drow, dcol) delta for this direction
    pub fn get_dir(&self) -> (i8, i8) {
        Self::DIRS[self.to_index() as usize]
    }

    /// Get the row delta
    pub fn get_drow(&self) -> i8 {
        D_ROW[self.to_index() as usize]
    }

    /// Get the column delta
    pub fn get_dcol(&self) -> i8 {
        D_COL[self.to_index() as usize]
    }

    /// Convert to index matching Go's direction constants
    /// Up=0, Left=1, Down=2, Right=3, None=4
    pub fn to_index(&self) -> u8 {
        match self {
            Direction::Up => 0,
            Direction::Left => 1,
            Direction::Down => 2,
            Direction::Right => 3,
            Direction::None => 4,
        }
    }

    /// Create direction from index
    pub fn from_index(idx: u8) -> Direction {
        match idx {
            0 => Direction::Up,
            1 => Direction::Left,
            2 => Direction::Down,
            3 => Direction::Right,
            _ => Direction::None,
        }
    }

    /// Get the reverse direction
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "up"),
            Direction::Left => write!(f, "left"),
            Direction::Down => write!(f, "down"),
            Direction::Right => write!(f, "right"),
            Direction::None => write!(f, "none"),
        }
    }
}
