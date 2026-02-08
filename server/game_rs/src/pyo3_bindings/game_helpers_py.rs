use pyo3::prelude::*;
use crate::game_helpers::{self, GameHelpers};

#[pyclass]
pub struct PyGameGameHelpers {
    game_helpers: GameHelpers
}

#[pymethods]
impl PyGameGameHelpers {
    #[new]
    pub fn new() -> Self {
        Self {
            game_helpers: GameHelpers::new()
        } 
    }

    pub fn get_bit<N, I>(num: N, bit_idx: I) -> bool 
    where
        N: Into<u32> + Copy,
        I: Into<u32> + Copy,
    {
        self.game_helpers.get_bit(num, bit_idx) 
    }

    // Decrement the fright steps of a ghost
    pub fn modify_bit<N, I>(num: &mut N, bit_idx: I, bit_val: bool)
    where
        N: From<u32> + Into<u32> + Copy,
        I: Into<u32> + Copy, 
    {
        self.game_helpers.modify_bit(num, bit_idx, bit_val)
    }

    // Get the fright steps of a ghost
    pub fn getfright_steps(&self) -> u8 {
        self.game_helpers.getfright_steps()
    }

    // Check if a ghost is frightened
    pub fn isFrightened(&self) -> bool {
        self.game_helpers.isFrightened()
    }

    /****************************** Ghost Trap State ******************************/

    // Set the trapped steps of a ghost
    pub fn settrapped_steps(&mut self, steps: u8) {
        self.game_helpers.settrapped_steps(steps)
    }

    // Decrement the trapped steps of a ghost
    pub fn dectrapped_steps(&mut self) {
        self.game_helpers.dectrapped_steps()
    }

    // Check if a ghost is trapped
    pub fn isTrapped(&self) -> bool {
        self.game_helpers.isTrapped()
    }

    /**************************** Ghost Spawning State ****************************/

    // Set the ghost spawning flag
    pub fn setSpawning(&mut self, spawning: bool) {
        self.game_helpers.setSpawning(spawning)
    }

    // Check if a ghost is spawning
    pub fn isSpawning(&self) -> bool {
        self.game_helpers.isSpawning()
    }

    /****************************** Ghost Eaten Flag ******************************/

    // Set the ghost eaten flag
    pub fn setEaten(&mut self, eaten: bool) {
        self.game_helpers.setEaten(eaten)
    }

    // Check if a ghost is eaten
    pub fn isEaten(&self) -> bool {
        self.game_helpers.eaten
    }
}
