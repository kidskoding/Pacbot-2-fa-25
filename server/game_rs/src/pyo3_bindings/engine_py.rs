// pyo3 bindings for engine.rs

use pyo3::prelude::*;
use crate::commands::interpret_command;
use crate::engine::GameEngine;
use crate::state::GameState;

#[pyclass]
pub struct PyGameEngine {
    engine: GameEngine,
}

#[pymethods]
impl PyGameEngine {
    #[new]
    pub fn new(clock_rate: Option<u32>) -> Self {
        Self {
            engine: GameEngine::new(clock_rate.unwrap_or(24)),
        }
    }

    /// Run one tick of the game loop (for Python-driven game loops)
    pub fn step(&mut self) {
        if self.engine.state.update_ready() {
            self.engine.state.update_all_ghosts();
            self.engine.state.try_respawn_pacman();

            if self.engine.state.get_pause_on_update() {
                self.engine.state.pause();
                self.engine.state.set_pause_on_update(false);
            }

            self.engine.state.check_collisions();
            self.engine.state.handle_step_events();
            self.engine.state.plan_all_ghosts();
        }

        if !self.engine.state.is_paused() {
            self.engine.state.next_tick();
        }
    }

    pub fn get_state_json(&self) -> String {
        self.engine.state.serialize()
    }

    pub fn send_command(&mut self, msg: Vec<u8>) -> bool {
        interpret_command(&msg, &mut self.engine.state, &self.engine.logger)
    }

    pub fn get_score(&self) -> u16 {
        self.engine.state.get_score()
    }

    pub fn get_lives(&self) -> u8 {
        self.engine.state.get_lives()
    }

    pub fn get_level(&self) -> u8 {
        self.engine.state.get_level()
    }

    pub fn is_paused(&self) -> bool {
        self.engine.state.is_paused()
    }

    pub fn reset(&mut self) {
        self.engine.state = GameState::new();
    }

    pub fn __repr__(&self) -> String {
        format!(
            "PyGameEngine(score={}, lives={}, level={}, ticks={})",
            self.engine.state.curr_score,
            self.engine.state.curr_lives,
            self.engine.state.curr_level,
            self.engine.state.curr_ticks,
        )
    }
}
