// state.rs - controls the state of the game

use crate::direction::Direction;
use serde::{Deserialize, Serialize};

// Position - A simple struct for the position of an object in PacMan
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

// Ghost - Pacmon ghost modeled
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Ghost {
    pub id: u8,
    pub pos: Position,
    pub direction: u32,
    pub is_eaten: bool,
}

// GameMode - Controls the game mode of the game
#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum GameMode {
    Chase,
    Scatter,
    Frightened,
    Paused,
}

// GameState - Controls the gamestate of the game
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    pub pacman: Position,
    pub ghosts: Vec<Ghost>,
    pub score: i32,
    pub level: i32,
    pub lives: i32,
    pub pellets_remaining: i32,
    pub mode: GameMode,
    pub last_unpaused_mode: GameMode
}

impl GameState {
    // Sets up a new game
    pub fn new() -> Self {
        Self {
            pacman: Position { x: 0, y: 0 },
            ghosts: vec![],
            score: 0,
            level: 1,
            lives: 3,
            pellets_remaining: 244,
            mode: GameMode::Paused,
            last_unpaused_mode: GameMode::Scatter,
        }
    }

    // Have PacMan make a move
    pub fn make_move(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.pacman.y -= 1,
            Direction::Down => self.pacman.y += 1,
            Direction::Left => self.pacman.x -= 1,
            Direction::Right => self.pacman.x += 1,
            _ => return,
        }
        
        // Temporarily adds a score of 10, assuming that he eats a pellet everytime he moves
        self.score += 10;
    }

    pub fn make_move_absolute(&mut self, x: i8, y: i8) {
        self.pacman.x = x;
        self.pacman.y = y;
    }

    // Temporarily subtracts score by 10 - WIP (will work on creating a stack of previous moves
    // that were done and then undo the most previous move ASAP)
    pub fn undo_move(&mut self) {
        self.score.saturating_sub(10);
    }

    pub fn pause(&mut self) {
        if self.mode != GameMode::Paused {
            self.last_unpaused_mode = self.mode;
            self.mode = GameMode::Paused;
        }
    }
    
    pub fn is_paused(&self) -> bool {
        self.mode == GameMode::Paused
    }
    
    pub fn play(&mut self) {
        if self.mode == GameMode::Paused {
            self.mode = self.last_unpaused_mode;
        }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
