use pyo3::prelude::*;

pub mod state_py;
pub mod ghost_state_py;
pub mod logging_py;
pub mod location_py;
pub mod constants_py;
pub mod engine_py;

pub fn register_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<state_py::PyGameState>()?;
    m.add_class::<ghost_state_py::PyGhostState>()?;
    m.add_class::<logging_py::PyLogging>()?;
    m.add_class::<location_py::PyDirection>()?;
    m.add_class::<location_py::PyLocationState>()?;
    m.add_class::<engine_py::PyGameEngine>()?;

    // Register constants
    constants_py::register_constants(m)?;

    Ok(())
}
