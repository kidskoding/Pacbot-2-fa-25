use pyo3::prelude::*;
use pyo3::types::PyModuleMethods;

use crate::location;

pub mod state_py;
pub mod ghost_state_py;
pub mod logging_py;
pub mod location_py;

pub fn register_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<state_py::PyGameState>()?;
    m.add_class::<ghost_state_py::PyGameGhostState>()?;
    m.add_class::<logging_py::PyLogging>()?;
    m.add_class::<location_py::PyDirection>()?;
    
    Ok(())
}
