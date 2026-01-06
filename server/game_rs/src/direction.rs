use std::fmt;

use serde::{Deserialize, Serialize};

// Direction - A simple struct for all the directions PacBot can move 
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub enum DirectionError {
    InvalidMove,
    NoneDirection,
}

impl fmt::Display for DirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DirectionError::InvalidMove => write!(f, "the requested move is blocked or invalid"),
            DirectionError::NoneDirection => write!(f, "cannot perform a move with Direction::None"),
        }
    }
}

impl Direction {
    pub const DIRS: [(i8, i8); 5] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (0, 0)
    ];

    pub fn get_dir(&self) -> (i8, i8) {
        match self {
            Direction::Up => Self::DIRS[0],
            Direction::Down => Self::DIRS[1],
            Direction::Left => Self::DIRS[2],
            Direction::Right => Self::DIRS[3],
            Direction::None => Self::DIRS[4],
        }
    }

    pub fn get_drow(&self) -> i8 {
        self.get_dir().0
    }

    pub fn get_dcol(&self) -> i8 {
        self.get_dir().1
    }
}
