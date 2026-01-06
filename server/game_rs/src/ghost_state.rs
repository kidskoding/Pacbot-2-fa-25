// ghost_state.rs - controls the state of the ghost

use serde::{Deserialize, Serialize};

use crate::state::GameState;

// Position - A simple struct for the position of an object in PacMan
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LocationState {
    pub x: i32,
    pub y: i32,
}

// Ghost - Pacmon ghost modeled
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GhostState {
  pub loc: LocationState, // Current location
	pub next_loc: LocationState, // Planned location (for next update)
	pub scatter_target: LocationState, // Position of (fixed) scatter target
	pub game: GameState,     // The game state tied to the ghost
	pub color: u8,
	pub trapped_steps: u8,
	pub fright_steps: u8,
	pub spawning: bool,        // Flag set when spawning
	pub eaten: bool,        // Flag set when eaten and returning to ghost house
}

impl GhostState {
    // Sets up a new ghost
    pub fn new() -> Self {
        Self {
            loc: LocationState {x:0, y:0},
            next_loc: LocationState {x:0, y:0},
            scatter_target : LocationState {x:0, y:0},
            game: GameState::new(),
            color: 0,
            trapped_steps: 0,
            fright_steps: 0,
            spawning: true,
            eaten: false,
        }
    }
    // Set the fright steps of a ghost
    pub fn setfright_steps(&mut self, steps: u8) {
        self.fright_steps = steps; 
    }

    // Decrement the fright steps of a ghost
    pub fn decfright_steps(&mut self) {
        self.fright_steps-=1;
    }

    // Get the fright steps of a ghost
    pub fn getfright_steps(&self) -> u8 {
        self.fright_steps
    }

    // Check if a ghost is frightened
    pub fn isFrightened(&self) -> bool {
        self.fright_steps > 0
    }

    /****************************** Ghost Trap State ******************************/

    // Set the trapped steps of a ghost
    pub fn settrapped_steps(&mut self, steps: u8) {
        self.trapped_steps = steps;
    }

    // Decrement the trapped steps of a ghost
    pub fn dectrapped_steps(&mut self) {
        self.trapped_steps-=1;
    }

    // Check if a ghost is trapped
    pub fn isTrapped(&self) -> bool {
        self.trapped_steps > 0
    }

    /**************************** Ghost Spawning State ****************************/

    // Set the ghost spawning flag
    pub fn setSpawning(&mut self, spawning: bool) {
        self.spawning = spawning;
    }

    // Check if a ghost is spawning
    pub fn isSpawning(&self) -> bool {
        self.spawning
    }

    /****************************** Ghost Eaten Flag ******************************/

    // Set the ghost eaten flag
    pub fn setEaten(&mut self, eaten: bool) {
        self.eaten = eaten;
    }

    // Check if a ghost is eaten
    pub fn isEaten(&self) -> bool {
        self.eaten
    }
}
