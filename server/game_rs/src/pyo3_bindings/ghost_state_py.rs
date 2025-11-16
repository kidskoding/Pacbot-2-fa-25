use pyo3::prelude::*;
use crate::ghost_state::{self, GhostState};

#[pyclass]
pub struct PyGameGhostState {
    ghost_state: GhostState
}

#[pymethods]
impl PyGameGhostState {
    #[new]
    pub fn new() -> Self {
        Self {
            ghost_state: GhostState::new()
        } 
    }

    pub fn setfright_steps(&mut self, steps: u8) {
        self.ghost_state.setfright_steps(steps) 
    }
    
    // Decrement the fright steps of a ghost
    pub fn decfright_steps(&mut self) {
        self.ghost_state.decfright_steps()
    }

    // Get the fright steps of a ghost
    pub fn getfright_steps(&self) -> u8 {
        self.ghost_state.getfright_steps()
    }

    // Check if a ghost is frightened
    pub fn isFrightened(&self) -> bool {
        self.ghost_state.isFrightened()
    }

    /****************************** Ghost Trap State ******************************/

    // Set the trapped steps of a ghost
    pub fn settrapped_steps(&mut self, steps: u8) {
        self.ghost_state.settrapped_steps(steps)
    }

    // Decrement the trapped steps of a ghost
    pub fn dectrapped_steps(&mut self) {
        self.ghost_state.dectrapped_steps()
    }

    // Check if a ghost is trapped
    pub fn isTrapped(&self) -> bool {
        self.ghost_state.isTrapped()
    }

    /**************************** Ghost Spawning State ****************************/

    // Set the ghost spawning flag
    pub fn setSpawning(&mut self, spawning: bool) {
        self.ghost_state.setSpawning(spawning)
    }

    // Check if a ghost is spawning
    pub fn isSpawning(&self) -> bool {
        self.ghost_state.isSpawning()
    }

    /****************************** Ghost Eaten Flag ******************************/

    // Set the ghost eaten flag
    pub fn setEaten(&mut self, eaten: bool) {
        self.ghost_state.setEaten(eaten)
    }

    // Check if a ghost is eaten
    pub fn isEaten(&self) -> bool {
        self.ghost_state.eaten
    }
}
