// pyo3 bindings for logging.rs

use pyo3::prelude::*;
use crate::logging::Logging;

#[pyclass]
pub struct PyLogging {
    logging: Logging,
}

#[pymethods]
impl PyLogging {
    #[new]
    pub fn new(enabled: Option<bool>) -> Self {
        Self {
            logging: Logging::new(enabled.unwrap_or(false)),
        }
    }

    pub fn get_command_log_enabled(&self) -> bool {
        self.logging.get_command_log_enabled()
    }

    pub fn set_command_log_enabled(&self, enabled: bool) {
        self.logging.set_command_log_enabled(enabled);
    }
}
