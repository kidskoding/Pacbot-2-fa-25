use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoggingState {
    command_log_enabled: bool
}

pub struct Logging {
    mutex_lock: Arc<RwLock<LoggingState>>
}

impl Logging {
    pub fn new(enabled: bool) -> Self {
        Logging {
            mutex_lock: Arc::new(RwLock::new(LoggingState {
                command_log_enabled: enabled,
            })),
        }
    }

    pub fn get_command_log_enabled(&self) -> bool {
        let guard = self.mutex_lock.read();
        guard.command_log_enabled
    }

    pub fn set_command_log_enabled(&self, enabled: bool) {
        let mut guard = self.mutex_lock.write();
        guard.command_log_enabled = enabled
    }
}
