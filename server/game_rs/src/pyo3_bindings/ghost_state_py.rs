// pyo3 bindings for ghost_state.rs

use pyo3::prelude::*;
use crate::ghost_state::GhostState;

#[pyclass]
pub struct PyGhostState {
    inner: GhostState,
}

#[pymethods]
impl PyGhostState {
    #[new]
    pub fn new(color: u8) -> Self {
        Self {
            inner: GhostState::new(color),
        }
    }

    pub fn get_pos(&self) -> (i8, i8) {
        self.inner.loc.get_coords()
    }

    pub fn get_next_pos(&self) -> (i8, i8) {
        self.inner.next_loc.get_coords()
    }

    pub fn get_color(&self) -> u8 {
        self.inner.color
    }

    pub fn set_fright_steps(&mut self, steps: u8) {
        self.inner.set_fright_steps(steps);
    }

    pub fn dec_fright_steps(&mut self) {
        self.inner.dec_fright_steps();
    }

    pub fn get_fright_steps(&self) -> u8 {
        self.inner.get_fright_steps()
    }

    pub fn is_frightened(&self) -> bool {
        self.inner.is_frightened()
    }

    pub fn set_trapped_steps(&mut self, steps: u8) {
        self.inner.set_trapped_steps(steps);
    }

    pub fn dec_trapped_steps(&mut self) {
        self.inner.dec_trapped_steps();
    }

    pub fn is_trapped(&self) -> bool {
        self.inner.is_trapped()
    }

    pub fn set_spawning(&mut self, spawning: bool) {
        self.inner.set_spawning(spawning);
    }

    pub fn is_spawning(&self) -> bool {
        self.inner.is_spawning()
    }

    pub fn set_eaten(&mut self, eaten: bool) {
        self.inner.set_eaten(eaten);
    }

    pub fn is_eaten(&self) -> bool {
        self.inner.is_eaten()
    }

    pub fn __repr__(&self) -> String {
        let (row, col) = self.inner.loc.get_coords();
        format!(
            "PyGhostState(color={}, pos=({}, {}), fright={}, trapped={}, spawning={}, eaten={})",
            self.inner.color,
            row,
            col,
            self.inner.fright_steps,
            self.inner.trapped_steps,
            self.inner.spawning,
            self.inner.eaten,
        )
    }
}
