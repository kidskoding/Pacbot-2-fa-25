use pyo3::prelude::*;

pub mod constants;
pub mod direction;
pub mod location;
pub mod ghost_state;
pub mod ghost_helpers;
pub mod state;
pub mod game_modes;
pub mod game_helpers;
pub mod commands;
pub mod logging;
pub mod engine;
mod pyo3_bindings;

#[pymodule]
pub fn game_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_bindings::register_bindings(m)?;
    Ok(())
}
