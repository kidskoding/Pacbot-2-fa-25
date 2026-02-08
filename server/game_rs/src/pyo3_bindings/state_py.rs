// pyo3 bindings for state.rs

use pyo3::prelude::*;
use crate::commands::interpret_command;
use crate::direction::Direction;
use crate::logging::Logging;
use crate::state::GameState;

#[pyclass]
pub struct PyGameState {
    pub inner: GameState,
    logger: Logging,
}

#[pymethods]
impl PyGameState {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: GameState::new(),
            logger: Logging::new(false),
        }
    }

    pub fn make_move(&mut self, dir: &str) {
        let direction = match dir {
            "up" | "w" => Direction::Up,
            "down" | "s" => Direction::Down,
            "left" | "a" => Direction::Left,
            "right" | "d" => Direction::Right,
            _ => return,
        };
        self.inner.move_pacman_dir(direction);
    }

    pub fn make_move_absolute(&mut self, row: i8, col: i8) {
        self.inner.move_pacman_absolute(row, col);
    }

    pub fn pause(&mut self) {
        self.inner.pause();
    }

    pub fn play(&mut self) {
        self.inner.play();
    }

    pub fn is_paused(&self) -> bool {
        self.inner.is_paused()
    }

    pub fn get_score(&self) -> u16 {
        self.inner.get_score()
    }

    pub fn get_level(&self) -> u8 {
        self.inner.get_level()
    }

    pub fn get_lives(&self) -> u8 {
        self.inner.get_lives()
    }

    pub fn get_num_pellets(&self) -> u16 {
        self.inner.get_num_pellets()
    }

    pub fn get_mode(&self) -> u8 {
        self.inner.get_mode()
    }

    pub fn get_pacman_pos(&self) -> (i8, i8) {
        self.inner.pacman_loc.get_coords()
    }

    pub fn get_ghost_pos(&self, color: u8) -> (i8, i8) {
        if (color as usize) < crate::constants::NUM_COLORS {
            self.inner.ghosts[color as usize].loc.get_coords()
        } else {
            (32, 32)
        }
    }

    pub fn get_curr_ticks(&self) -> u16 {
        self.inner.get_curr_ticks()
    }

    pub fn serialize(&self) -> String {
        self.inner.serialize()
    }

    pub fn reset(&mut self) {
        self.inner = GameState::new();
    }

    pub fn update(&mut self) {
        if self.inner.update_ready() {
            self.inner.update_all_ghosts();
            self.inner.try_respawn_pacman();
            if self.inner.get_pause_on_update() {
                self.inner.pause();
                self.inner.set_pause_on_update(false);
            }
            self.inner.check_collisions();
            self.inner.handle_step_events();
            self.inner.plan_all_ghosts();
        }
    }

    pub fn interpret_command(&mut self, msg: Vec<u8>) -> bool {
        interpret_command(&msg, &mut self.inner, &self.logger)
    }

    pub fn __repr__(&self) -> String {
        format!(
            "PyGameState(score={}, pacman=({}, {}), lives={}, level={}, pellets={})",
            self.inner.curr_score,
            self.inner.pacman_loc.row,
            self.inner.pacman_loc.col,
            self.inner.curr_lives,
            self.inner.curr_level,
            self.inner.num_pellets,
        )
    }
}
