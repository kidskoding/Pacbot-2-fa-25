// ghost_helpers.rs - Ghost AI and lifecycle, ported from Go ghost_helpers.go
//
// Note: Most ghost logic (plan, update, reset, respawn) is implemented as
// methods on GameState in game_helpers.rs to avoid borrow checker conflicts
// (ghosts are part of GameState, so ghost methods can't also borrow GameState).
//
// This file provides standalone helper methods on GhostState for operations
// that don't need access to the full game state.

use crate::constants::*;
use crate::ghost_state::GhostState;

impl GhostState {
    /// Reset the ghost to its initial spawn state
    pub fn reset_to_spawn(&mut self) {
        let spawn_locs = ghost_spawn_locs();

        if self.color >= NUM_ACTIVE_GHOSTS {
            return;
        }

        self.set_spawning(true);
        self.set_trapped_steps(GHOST_TRAPPED_STEPS[self.color as usize]);
        self.set_fright_steps(0);
        self.loc.copy_from(&empty_loc());
        self.next_loc.copy_from(&spawn_locs[self.color as usize]);
    }

    /// Respawn ghost after being eaten (goes to ghost house)
    pub fn respawn_eaten(&mut self) {
        let spawn_locs = ghost_spawn_locs();

        if self.color >= NUM_ACTIVE_GHOSTS {
            return;
        }

        self.set_spawning(true);
        self.set_eaten(true);
        self.loc.copy_from(&empty_loc());

        // Red goes to pink's spawn, others go to their own spawn
        if self.color == RED {
            let (pr, pc) = spawn_locs[PINK as usize].get_coords();
            self.next_loc.update_coords(pr, pc);
        } else {
            self.next_loc
                .copy_from(&spawn_locs[self.color as usize]);
        }
        self.next_loc
            .update_dir(crate::direction::Direction::Up);
    }

    /// Get the ghost's current position
    pub fn get_pos(&self) -> (i8, i8) {
        self.loc.get_coords()
    }

    /// Get the ghost's next planned position
    pub fn get_next_pos(&self) -> (i8, i8) {
        self.next_loc.get_coords()
    }
}
