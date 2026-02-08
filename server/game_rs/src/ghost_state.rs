// ghost_state.rs - Ghost state management, ported from Go ghost_state.go

use serde::{Deserialize, Serialize};

use crate::constants::*;
use crate::location::LocationState;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GhostState {
    pub loc: LocationState,
    pub next_loc: LocationState,
    pub scatter_target: LocationState,
    pub color: u8,
    pub trapped_steps: u8,
    pub fright_steps: u8,
    pub spawning: bool,
    pub eaten: bool,
}

impl GhostState {
    /// Create a new ghost with the given color, initialized from constants
    pub fn new(color: u8) -> Self {
        let spawn_locs = ghost_spawn_locs();
        let scatter_targets = ghost_scatter_targets();
        let color_idx = color as usize;

        let next_loc = if color >= NUM_ACTIVE_GHOSTS {
            empty_loc()
        } else {
            spawn_locs[color_idx].clone()
        };

        Self {
            loc: empty_loc(),
            next_loc,
            scatter_target: scatter_targets[color_idx].clone(),
            color,
            trapped_steps: GHOST_TRAPPED_STEPS[color_idx],
            fright_steps: 0,
            spawning: true,
            eaten: false,
        }
    }

    /************************ Ghost Frightened State ************************/

    pub fn set_fright_steps(&mut self, steps: u8) {
        self.fright_steps = steps;
    }

    pub fn dec_fright_steps(&mut self) {
        if self.fright_steps > 0 {
            self.fright_steps -= 1;
        }
    }

    pub fn get_fright_steps(&self) -> u8 {
        self.fright_steps
    }

    pub fn is_frightened(&self) -> bool {
        self.fright_steps > 0
    }

    /************************** Ghost Trap State **************************/

    pub fn set_trapped_steps(&mut self, steps: u8) {
        self.trapped_steps = steps;
    }

    pub fn dec_trapped_steps(&mut self) {
        if self.trapped_steps > 0 {
            self.trapped_steps -= 1;
        }
    }

    pub fn is_trapped(&self) -> bool {
        self.trapped_steps > 0
    }

    /************************ Ghost Spawning State ************************/

    pub fn set_spawning(&mut self, spawning: bool) {
        self.spawning = spawning;
    }

    pub fn is_spawning(&self) -> bool {
        self.spawning
    }

    /************************* Ghost Eaten Flag **************************/

    pub fn set_eaten(&mut self, eaten: bool) {
        self.eaten = eaten;
    }

    pub fn is_eaten(&self) -> bool {
        self.eaten
    }
}
