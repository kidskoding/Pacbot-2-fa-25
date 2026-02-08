// engine.rs - Game engine loop, ported from Go game_engine.go

use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time;
use tracing::{info, warn};

use crate::commands::interpret_command;
use crate::logging::Logging;
use crate::state::GameState;

pub struct GameEngine {
    pub state: GameState,
    pub logger: Logging,
    tick_duration: Duration,
}

impl GameEngine {
    pub fn new(clock_rate: u32) -> Self {
        let tick_duration = Duration::from_micros(1_000_000 / clock_rate as u64);

        Self {
            state: GameState::new(),
            logger: Logging::new(false),
            tick_duration,
        }
    }

    /// Run the main game loop (async, matching Go's runLoop)
    pub async fn run_loop(
        &mut self,
        output_tx: mpsc::Sender<String>,
        mut input_rx: mpsc::Receiver<Vec<u8>>,
        mut quit_rx: mpsc::Receiver<()>,
    ) {
        info!("LOG: Game engine started");

        let mut interval = time::interval(self.tick_duration);
        let mut just_ticked = true;

        loop {
            // Step 1: Update if ready
            if just_ticked && self.state.update_ready() {
                // Update all ghosts
                self.state.update_all_ghosts();

                // Try to respawn pacman
                self.state.try_respawn_pacman();

                // Pause on update if flagged
                if self.state.get_pause_on_update() {
                    self.state.pause();
                    self.state.set_pause_on_update(false);
                }

                // Check collisions
                self.state.check_collisions();

                // Handle step events (mode changes, penalties, fruit)
                self.state.handle_step_events();

                // Plan next ghost moves
                self.state.plan_all_ghosts();
            }

            // Step 2: Serialize and send state
            let serialized = self.state.serialize();
            if output_tx.send(serialized).await.is_err() {
                warn!("WARN: Output channel closed");
                break;
            }

            // Step 3: Read commands
            loop {
                match input_rx.try_recv() {
                    Ok(msg) => {
                        let rst = interpret_command(&msg, &mut self.state, &self.logger);
                        if rst {
                            self.state = GameState::new();
                            self.state.update_all_ghosts();
                            self.state.handle_step_events();
                            self.state.plan_all_ghosts();
                            just_ticked = true;
                        }
                    }
                    Err(_) => break,
                }
            }

            // Step 4: Tick
            if !self.state.is_paused() {
                just_ticked = true;
                self.state.next_tick();
            } else {
                just_ticked = false;
            }

            // Step 5: Wait for next tick or quit
            tokio::select! {
                _ = interval.tick() => {},
                _ = quit_rx.recv() => {
                    info!("LOG: Game engine quit");
                    return;
                }
            }
        }
    }
}
