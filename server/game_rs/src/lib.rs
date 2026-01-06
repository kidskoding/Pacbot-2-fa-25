use pyo3::prelude::*;

mod state;
mod ghost_state;
mod pyo3_bindings;
mod commands;
mod location;
mod logging;
mod engine;
mod direction;

#[pymodule]
pub fn game_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_bindings::register_bindings(m)?;
    Ok(())
}
