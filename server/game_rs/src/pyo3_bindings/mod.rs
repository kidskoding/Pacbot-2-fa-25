use pyo3::prelude::*;
use pyo3::types::PyModuleMethods;

pub mod state_py;
pub mod ghost_state_py;

pub fn register_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<state_py::PyGameState>()?;
    m.add_class::<ghost_state_py::PyGameGhostState>()?;
    Ok(())
}
