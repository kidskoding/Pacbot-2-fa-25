use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::direction::Direction;
use crate::logging::Logging;
use crate::state::GameState;

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    id: u32,
    action: String,
    target: String,
}

pub fn interpret_command(msg: &[u8], gs: &mut GameState, logger: &Logging) -> bool {
    if msg.is_empty() {
        return false;
    }

    if logger.get_command_log_enabled() {
        let cmd_char = msg[0] as char;
        if msg.len() > 1 {
            info!("COMM: {} {:?}", cmd_char, &msg[1..]);
        } else {
            info!("COMM: {}", cmd_char);
        }
    }

    match msg[0] {
        b'p' => { gs.pause(); }
        b'P' => { gs.play(); }
        b'r' | b'R' => return true,
        b'w' => gs.make_move(Direction::Up),
        b'a' => gs.make_move(Direction::Left),
        b's' => gs.make_move(Direction::Down),
        b'd' => gs.make_move(Direction::Right),
        b'x' => {
            if msg.len() != 3 {
                warn!("ERR: Invalid position update (type 'x'). Ignoring...");
                return false;
            }

            gs.make_move_absolute(msg[1] as i8, msg[2] as i8);
        }
        _ => {
            warn!("Received unknown command byte: {}", msg[0]);
        }
    }   

    true
} 
