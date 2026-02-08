// state.rs - Central game state, ported from Go game_state.go

use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::constants::*;
use crate::ghost_state::GhostState;
use crate::location::LocationState;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameState {
    // Header
    pub curr_ticks: u16,
    pub update_period: u8,
    pub mode: u8,
    pub last_unpaused_mode: u8,
    pub pause_on_update: bool,
    pub mode_steps: u8,
    pub level_steps: u16,

    // Game info
    pub curr_score: u16,
    pub curr_level: u8,
    pub curr_lives: u8,

    // Entities
    pub pacman_loc: LocationState,
    pub fruit_loc: LocationState,
    pub fruit_steps: u8,
    pub ghosts: [GhostState; NUM_COLORS],
    pub ghost_combo: u8,

    // Maze
    pub pellets: [u32; MAZE_ROWS as usize],
    pub num_pellets: u16,
    #[serde(skip, default = "default_walls")]
    pub walls: [u32; MAZE_ROWS as usize],

    // RNG
    #[serde(skip, default = "default_rng")]
    pub rng: StdRng,
}

fn default_walls() -> [u32; MAZE_ROWS as usize] {
    INIT_WALLS
}

fn default_rng() -> StdRng {
    StdRng::from_entropy()
}

impl GameState {
    pub fn new() -> Self {
        Self {
            // Header
            curr_ticks: 0,
            update_period: INIT_UPDATE_PERIOD,
            mode: PAUSED,
            last_unpaused_mode: INIT_MODE,
            pause_on_update: false,
            mode_steps: MODE_DURATIONS[INIT_MODE as usize],
            level_steps: LEVEL_DURATION,

            // Game info
            curr_score: 0,
            curr_level: INIT_LEVEL,
            curr_lives: INIT_LIVES,

            // Entities
            pacman_loc: pacman_spawn_loc(),
            fruit_loc: fruit_spawn_loc(),
            fruit_steps: 0,
            ghosts: [
                GhostState::new(RED),
                GhostState::new(PINK),
                GhostState::new(CYAN),
                GhostState::new(ORANGE),
            ],
            ghost_combo: 0,

            // Maze
            pellets: INIT_PELLETS,
            num_pellets: INIT_PELLET_COUNT,
            walls: INIT_WALLS,

            // RNG
            rng: StdRng::from_entropy(),
        }
    }

    /**************************** Tick Functions ****************************/

    pub fn get_curr_ticks(&self) -> u16 {
        self.curr_ticks
    }

    pub fn next_tick(&mut self) {
        if self.curr_ticks == 0xffff {
            return;
        }
        if self.curr_ticks == 0xfffe {
            self.pause();
            warn!("GAME: Max tick limit reached");
        }
        self.curr_ticks += 1;
    }

    /************************ Update Period Functions ************************/

    pub fn get_update_period(&self) -> u8 {
        self.update_period
    }

    pub fn set_update_period(&mut self, period: u8) {
        info!(
            "GAME: Update period changed ({} -> {}) (t = {})",
            self.update_period,
            period,
            self.curr_ticks
        );
        self.update_period = period;
    }

    /************************** Score Functions **************************/

    pub fn get_score(&self) -> u16 {
        self.curr_score
    }

    pub fn increment_score(&mut self, change: u16) {
        let score = self.curr_score as u32;
        self.curr_score = (score + change as u32).min(65535) as u16;
    }

    /************************** Level Functions **************************/

    pub fn get_level(&self) -> u8 {
        self.curr_level
    }

    pub fn set_level(&mut self, level: u8) {
        info!(
            "GAME: Level changed ({} -> {}) (t = {})",
            self.curr_level,
            level,
            self.curr_ticks
        );
        self.curr_level = level;
        let suggested = INIT_UPDATE_PERIOD as i32 - 2 * (level as i32 - 1);
        self.set_update_period(suggested.max(1) as u8);
    }

    pub fn increment_level(&mut self) {
        if self.curr_level == 255 {
            return;
        }
        let level = self.curr_level;
        info!(
            "GAME: Next level ({} -> {}) (t = {})",
            level,
            level + 1,
            self.curr_ticks
        );
        self.curr_level += 1;
        let suggested = INIT_UPDATE_PERIOD as i32 - 2 * level as i32;
        self.set_update_period(suggested.max(1) as u8);
    }

    /************************** Lives Functions **************************/

    pub fn get_lives(&self) -> u8 {
        self.curr_lives
    }

    pub fn set_lives(&mut self, lives: u8) {
        info!("GAME: Lives changed ({} -> {})", self.curr_lives, lives);
        self.curr_lives = lives;
    }

    pub fn decrement_lives(&mut self) {
        if self.curr_lives == 0 {
            return;
        }
        info!(
            "GAME: Pacman lost a life ({} -> {}) (t = {})",
            self.curr_lives,
            self.curr_lives - 1,
            self.curr_ticks
        );
        self.curr_lives -= 1;
    }

    /************************** Pellet Functions **************************/

    pub fn get_num_pellets(&self) -> u16 {
        self.num_pellets
    }

    pub fn decrement_num_pellets(&mut self) {
        if self.num_pellets > 0 {
            self.num_pellets -= 1;
        }
    }

    pub fn reset_pellets(&mut self) {
        self.pellets = INIT_PELLETS;
        self.num_pellets = INIT_PELLET_COUNT;
    }

    /************************** Fruit Functions **************************/

    pub fn get_fruit_steps(&self) -> u8 {
        self.fruit_steps
    }

    pub fn fruit_exists(&self) -> bool {
        self.fruit_steps > 0
    }

    pub fn set_fruit_steps(&mut self, steps: u8) {
        self.fruit_steps = steps;
    }

    pub fn decrement_fruit_steps(&mut self) {
        if self.fruit_steps > 0 {
            self.fruit_steps -= 1;
        }
    }

    /************************ Level Steps Functions ************************/

    pub fn get_level_steps(&self) -> u16 {
        self.level_steps
    }

    pub fn set_level_steps(&mut self, steps: u16) {
        self.level_steps = steps;
    }

    pub fn decrement_level_steps(&mut self) {
        if self.level_steps > 0 {
            self.level_steps -= 1;
        }
    }

    /************************ Step-Related Events ************************/

    pub fn handle_step_events(&mut self) {
        let mode_steps = self.get_mode_steps();
        let level_steps = self.get_level_steps();

        // If mode steps hit 0, change the mode
        if mode_steps == 0 {
            let mode = self.get_mode();
            match mode {
                CHASE => {
                    self.set_mode(SCATTER);
                    self.set_mode_steps(MODE_DURATIONS[SCATTER as usize]);
                }
                SCATTER => {
                    self.set_mode(CHASE);
                    self.set_mode_steps(MODE_DURATIONS[CHASE as usize]);
                }
                PAUSED => {
                    let last_mode = self.get_last_unpaused_mode();
                    match last_mode {
                        CHASE => {
                            self.set_last_unpaused_mode(SCATTER);
                            self.set_mode_steps(MODE_DURATIONS[SCATTER as usize]);
                        }
                        SCATTER => {
                            self.set_last_unpaused_mode(CHASE);
                            self.set_mode_steps(MODE_DURATIONS[CHASE as usize]);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            self.reverse_all_ghosts();
        }

        // If level steps hit 0, apply penalty by speeding up
        if level_steps == 0 {
            warn!("GAME: Long-game penalty applied");
            let new_period = (self.get_update_period() as i32 - 2).max(1) as u8;
            self.set_update_period(new_period);
            self.set_level_steps(LEVEL_PENALTY_DURATION);
        }

        // Decrement mode steps (only if ghosts aren't angry)
        if self.get_num_pellets() >= ANGER_THRESHOLD_1 {
            self.decrement_mode_steps();
        }

        // Decrement level steps and fruit steps
        self.decrement_level_steps();
        self.decrement_fruit_steps();
    }

    /************************ Serialization ************************/

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
