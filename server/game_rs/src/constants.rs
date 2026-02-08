// constants.rs - All game constants and maze data, ported from Go variables.go

use crate::direction::Direction;
use crate::location::LocationState;

// Maze dimensions
pub const MAZE_ROWS: i8 = 31;
pub const MAZE_COLS: i8 = 28;

// Timing
pub const INIT_UPDATE_PERIOD: u8 = 12;
pub const LEVEL_DURATION: u16 = 960; // 8 minutes at 24 fps, update period = 12
pub const LEVEL_PENALTY_DURATION: u16 = 240; // 2 min (24fps, update period = 12)

// Game modes
pub const PAUSED: u8 = 0;
pub const SCATTER: u8 = 1;
pub const CHASE: u8 = 2;
pub const NUM_MODES: usize = 3;

pub const INIT_MODE: u8 = SCATTER;

// Mode durations in steps (update periods)
pub const MODE_DURATIONS: [u8; NUM_MODES] = [
    255, // paused
    60,  // scatter - 30 seconds at 24 fps
    180, // chase   - 90 seconds at 24 fps
];

// Initial game values
pub const INIT_LEVEL: u8 = 1;
pub const INIT_LIVES: u8 = 3;

// Ghost colors
pub const RED: u8 = 0;
pub const PINK: u8 = 1;
pub const CYAN: u8 = 2;
pub const ORANGE: u8 = 3;
pub const NUM_COLORS: usize = 4;

pub const NUM_ACTIVE_GHOSTS: u8 = 4;

// Ghost house exit location
pub const GHOST_HOUSE_EXIT_ROW: i8 = 12;
pub const GHOST_HOUSE_EXIT_COL: i8 = 13;

// Ghost names (for debugging)
pub const GHOST_NAMES: [&str; NUM_COLORS] = ["red", "pink", "cyan", "orange"];

// Spawn locations
pub fn pacman_spawn_loc() -> LocationState {
    LocationState::new(23, 13, Direction::Right)
}

pub fn fruit_spawn_loc() -> LocationState {
    LocationState::new(17, 13, Direction::None)
}

pub fn empty_loc() -> LocationState {
    LocationState::new(32, 32, Direction::None)
}

// Ghost spawn locations
pub fn ghost_spawn_locs() -> [LocationState; NUM_COLORS] {
    [
        LocationState::new(11, 13, Direction::Left), // red
        LocationState::new(13, 13, Direction::Down), // pink
        LocationState::new(14, 11, Direction::Up),   // cyan
        LocationState::new(14, 15, Direction::Up),   // orange
    ]
}

// Ghost scatter targets (fixed)
pub fn ghost_scatter_targets() -> [LocationState; NUM_COLORS] {
    [
        LocationState::new(-3, 25, Direction::None), // red
        LocationState::new(-3, 2, Direction::None),  // pink
        LocationState::new(31, 27, Direction::None), // cyan
        LocationState::new(31, 0, Direction::None),  // orange
    ]
}

// Ghost trapped steps (how long each ghost stays trapped at start)
pub const GHOST_TRAPPED_STEPS: [u8; NUM_COLORS] = [
    0,  // red
    5,  // pink
    16, // cyan
    32, // orange
];

// Ghost fright duration
pub const GHOST_FRIGHT_STEPS: u8 = 40;

// Fruit
pub const FRUIT_DURATION: u8 = 30;
pub const FRUIT_POINTS: u16 = 100;

// Pellets
pub const INIT_PELLET_COUNT: u16 = 244;
pub const PELLET_POINTS: u16 = 10;
pub const SUPER_PELLET_POINTS: u16 = 50;

// Ghost combo multiplier
pub const COMBO_MULTIPLIER: u16 = 200;

// Fruit spawn thresholds (pellet count remaining)
pub const FRUIT_THRESHOLD_1: u16 = 174;
pub const FRUIT_THRESHOLD_2: u16 = 74;

// Anger thresholds (pellet count remaining)
pub const ANGER_THRESHOLD_1: u16 = 20;
pub const ANGER_THRESHOLD_2: u16 = 10;

// Initial pellet layout as bit arrays (column 0 = bit 0 on the right)
pub const INIT_PELLETS: [u32; MAZE_ROWS as usize] = [
    0b0000_0000000000000000000000000000, // row 0
    0b0000_0111111111111001111111111110, // row 1
    0b0000_0100001000001001000001000010, // row 2
    0b0000_0100001000001001000001000010, // row 3
    0b0000_0100001000001001000001000010, // row 4
    0b0000_0111111111111111111111111110, // row 5
    0b0000_0100001001000000001001000010, // row 6
    0b0000_0100001001000000001001000010, // row 7
    0b0000_0111111001111001111001111110, // row 8
    0b0000_0000001000000000000001000000, // row 9
    0b0000_0000001000000000000001000000, // row 10
    0b0000_0000001000000000000001000000, // row 11
    0b0000_0000001000000000000001000000, // row 12
    0b0000_0000001000000000000001000000, // row 13
    0b0000_0000001000000000000001000000, // row 14
    0b0000_0000001000000000000001000000, // row 15
    0b0000_0000001000000000000001000000, // row 16
    0b0000_0000001000000000000001000000, // row 17
    0b0000_0000001000000000000001000000, // row 18
    0b0000_0000001000000000000001000000, // row 19
    0b0000_0111111111111001111111111110, // row 20
    0b0000_0100001000001001000001000010, // row 21
    0b0000_0100001000001001000001000010, // row 22
    0b0000_0111001111111001111111001110, // row 23
    0b0000_0001001001000000001001001000, // row 24
    0b0000_0001001001000000001001001000, // row 25
    0b0000_0111111001111001111001111110, // row 26
    0b0000_0100000000001001000000000010, // row 27
    0b0000_0100000000001001000000000010, // row 28
    0b0000_0111111111111111111111111110, // row 29
    0b0000_0000000000000000000000000000, // row 30
];

// Wall layout as bit arrays (column 0 = bit 0 on the right)
pub const INIT_WALLS: [u32; MAZE_ROWS as usize] = [
    0b0000_1111111111111111111111111111, // row 0
    0b0000_1000000000000110000000000001, // row 1
    0b0000_1011110111110110111110111101, // row 2
    0b0000_1011110111110110111110111101, // row 3
    0b0000_1011110111110110111110111101, // row 4
    0b0000_1000000000000000000000000001, // row 5
    0b0000_1011110110111111110110111101, // row 6
    0b0000_1011110110111111110110111101, // row 7
    0b0000_1000000110000110000110000001, // row 8
    0b0000_1111110111110110111110111111, // row 9
    0b0000_1111110111110110111110111111, // row 10
    0b0000_1111110110000000000110111111, // row 11
    0b0000_1111110110111111110110111111, // row 12
    0b0000_1111110110111111110110111111, // row 13
    0b0000_1111110000111111110000111111, // row 14
    0b0000_1111110110111111110110111111, // row 15
    0b0000_1111110110111111110110111111, // row 16
    0b0000_1111110110000000000110111111, // row 17
    0b0000_1111110110111111110110111111, // row 18
    0b0000_1111110110111111110110111111, // row 19
    0b0000_1000000000000110000000000001, // row 20
    0b0000_1011110111110110111110111101, // row 21
    0b0000_1011110111110110111110111101, // row 22
    0b0000_1000110000000000000000110001, // row 23
    0b0000_1110110110111111110110110111, // row 24
    0b0000_1110110110111111110110110111, // row 25
    0b0000_1000000110000110000110000001, // row 26
    0b0000_1011111111110110111111111101, // row 27
    0b0000_1011111111110110111111111101, // row 28
    0b0000_1000000000000000000000000001, // row 29
    0b0000_1111111111111111111111111111, // row 30
];
