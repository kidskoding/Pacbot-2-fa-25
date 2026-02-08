// game_modes.rs - Game mode management, ported from Go game_modes.go

use tracing::info;

use crate::constants::*;
use crate::state::GameState;

// Mode name strings for logging
const MODE_NAMES: [&str; NUM_MODES] = ["paused", "scatter", "chase"];

impl GameState {
    /****************************** Current Mode ******************************/

    pub fn get_mode(&self) -> u8 {
        self.mode
    }

    pub fn set_mode(&mut self, mode: u8) {
        if self.mode != PAUSED && mode != PAUSED && self.mode != mode {
            info!(
                "GAME: Mode changed ({} -> {}) (t = {})",
                MODE_NAMES[self.mode as usize],
                MODE_NAMES[mode as usize],
                self.curr_ticks
            );
        }
        self.mode = mode;
    }

    /**************************** Last Unpaused Mode ****************************/

    pub fn get_last_unpaused_mode(&self) -> u8 {
        if self.mode != PAUSED {
            self.mode
        } else {
            self.last_unpaused_mode
        }
    }

    pub fn set_last_unpaused_mode(&mut self, mode: u8) {
        let unpaused_mode = self.get_last_unpaused_mode();
        if self.mode == PAUSED && unpaused_mode != mode {
            info!(
                "GAME: Mode changed while paused ({} -> {}) (t = {})",
                MODE_NAMES[unpaused_mode as usize],
                MODE_NAMES[mode as usize],
                self.curr_ticks
            );
        }
        self.last_unpaused_mode = mode;
    }

    /******************************* Pause / Play *******************************/

    pub fn is_paused(&self) -> bool {
        self.mode == PAUSED
    }

    pub fn pause(&mut self) {
        if self.is_paused() {
            return;
        }
        self.last_unpaused_mode = self.mode;
        self.mode = PAUSED;
        info!("GAME: Paused (t = {})", self.curr_ticks);
    }

    pub fn play(&mut self) {
        if !self.is_paused() || self.curr_lives == 0 || self.curr_ticks == 0xffff {
            return;
        }
        self.mode = self.last_unpaused_mode;
        info!("GAME: Resumed (t = {})", self.curr_ticks);
    }

    /************************* Pause on Next Update *************************/

    pub fn get_pause_on_update(&self) -> bool {
        self.pause_on_update
    }

    pub fn set_pause_on_update(&mut self, flag: bool) {
        self.pause_on_update = flag;
    }

    /****************************** Mode Steps ******************************/

    pub fn get_mode_steps(&self) -> u8 {
        self.mode_steps
    }

    pub fn set_mode_steps(&mut self, steps: u8) {
        self.mode_steps = steps;
    }

    pub fn decrement_mode_steps(&mut self) {
        if self.mode_steps > 0 {
            self.mode_steps -= 1;
        }
    }
}
