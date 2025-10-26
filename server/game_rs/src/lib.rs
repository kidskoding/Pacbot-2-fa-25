use pyo3::prelude::*;

mod state;
mod ghost_state;
mod pyo3_bindings;

#[pymodule]
pub fn game_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_bindings::register_bindings(m)?;
    Ok(())
}
