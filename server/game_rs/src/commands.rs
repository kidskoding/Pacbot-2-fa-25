use serde::{Deserialize, Serialize};

use crate::logging::Logging;
use crate::state::GameState;

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    id: u32,
    action: String,
    target: String,
}

pub fn interpret_command(msg: &[u8], gs: &mut GameState, logger: &Logging) -> bool {
    if logger.get_command_log_enabled() {
        if msg.is_empty() {
            tracing::warn!("received empty command message. ignoring!");
            return false;
        }
    }
} 
