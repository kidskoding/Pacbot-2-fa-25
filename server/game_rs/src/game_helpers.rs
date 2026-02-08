// game_helpers.rs - Core game logic, ported from Go game_helpers.go

use std::collections::{HashMap, VecDeque};

use rand::Rng;
use tracing::warn;

use crate::constants::*;
use crate::direction::{Direction, NUM_DIRS};
use crate::location::LocationState;
use crate::state::GameState;

/***************************** Bitwise Operations *****************************/

pub fn get_bit(num: u32, bit_idx: i8) -> bool {
    ((num >> bit_idx as u32) & 1) == 1
}

pub fn modify_bit(num: &mut u32, bit_idx: i8, val: bool) {
    if val {
        *num |= 1 << bit_idx as u32;
    } else {
        *num &= !(1 << bit_idx as u32);
    }
}

impl GameState {
    /****************************** Timing Functions ******************************/

    pub fn update_ready(&self) -> bool {
        let update_period = self.get_update_period() as u16;
        if update_period == 0 {
            return false;
        }
        self.curr_ticks % update_period == 0
    }

    /**************************** Positional Functions ****************************/

    pub fn in_bounds(&self, row: i8, col: i8) -> bool {
        row >= 0 && row < MAZE_ROWS && col >= 0 && col < MAZE_COLS
    }

    pub fn pellet_at(&self, row: i8, col: i8) -> bool {
        if !self.in_bounds(row, col) {
            return false;
        }
        get_bit(self.pellets[row as usize], col)
    }

    pub fn collect_pellet(&mut self, row: i8, col: i8) {
        // Collect fruit if applicable
        if self.fruit_exists() && self.pacman_loc.collides_with(&self.fruit_loc) {
            self.set_fruit_steps(0);
            self.increment_score(FRUIT_POINTS);
        }

        // If there's no pellet, return
        if !self.pellet_at(row, col) {
            return;
        }

        // Clear the pellet bit and decrement count
        modify_bit(&mut self.pellets[row as usize], col, false);
        self.decrement_num_pellets();

        // Check for super pellet
        let super_pellet = (row == 3 || row == 23) && (col == 1 || col == 26);

        // Frighten all ghosts if super pellet
        if super_pellet {
            self.frighten_all_ghosts();
        }

        // Update score
        if super_pellet {
            self.increment_score(SUPER_PELLET_POINTS);
        } else {
            self.increment_score(PELLET_POINTS);
        }

        let num_pellets = self.get_num_pellets();

        // Spawn fruit at thresholds
        if (num_pellets == FRUIT_THRESHOLD_1 || num_pellets == FRUIT_THRESHOLD_2)
            && !self.fruit_exists()
        {
            self.set_fruit_steps(FRUIT_DURATION);
        }

        // Pellet-related events
        if num_pellets == ANGER_THRESHOLD_1 || num_pellets == ANGER_THRESHOLD_2 {
            let new_period = (self.get_update_period() as i32 - 2).max(1) as u8;
            self.set_update_period(new_period);
            self.set_mode(CHASE);
            self.set_mode_steps(MODE_DURATIONS[CHASE as usize]);
        } else if num_pellets == 0 {
            self.level_reset();
            self.increment_level();
        }
    }

    pub fn wall_at(&self, row: i8, col: i8) -> bool {
        if !self.in_bounds(row, col) {
            return true;
        }
        get_bit(self.walls[row as usize], col)
    }

    pub fn ghost_spawn_at(&self, row: i8, col: i8) -> bool {
        if !self.in_bounds(row, col) {
            return false;
        }
        (row >= 13 && row <= 14) && (col >= 11 && col <= 15)
    }

    pub fn dist_sq(&self, row1: i8, col1: i8, row2: i8, col2: i8) -> i32 {
        let dr = (row2 - row1) as i32;
        let dc = (col2 - col1) as i32;
        dr * dr + dc * dc
    }

    /***************************** Collision Handling *****************************/

    pub fn check_collisions(&mut self) {
        let mut ghost_respawn_flag: u8 = 0;
        let mut num_ghost_respawns = 0;

        let pacman_loc = self.pacman_loc.clone();

        for i in 0..NUM_COLORS {
            let ghost = &self.ghosts[i];

            if pacman_loc.collides_with(&ghost.loc) {
                if ghost.is_eaten() {
                    continue;
                }

                if ghost.is_frightened() {
                    modify_bit_u8(&mut ghost_respawn_flag, ghost.color, true);
                    num_ghost_respawns += 1;
                } else {
                    self.death_reset();
                    return;
                }
            }
        }

        if num_ghost_respawns == 0 {
            return;
        }

        self.respawn_ghosts(ghost_respawn_flag);
    }

    /***************************** Event-Based Resets *****************************/

    pub fn death_reset(&mut self) {
        // Pause on next update
        self.set_pause_on_update(true);

        // Set pacman to empty
        self.pacman_loc.copy_from(&empty_loc());

        // Lose a life
        self.decrement_lives();

        // Reset mode if ghosts aren't angry
        if self.get_num_pellets() > ANGER_THRESHOLD_1 {
            self.set_mode(INIT_MODE);
            self.set_mode_steps(MODE_DURATIONS[INIT_MODE as usize]);
        }

        // Clear fruit
        self.set_fruit_steps(0);

        // Reset all ghosts
        self.reset_all_ghosts();
    }

    pub fn level_reset(&mut self) {
        // Pause on next update
        self.set_pause_on_update(true);

        // Set pacman to empty
        self.pacman_loc.copy_from(&empty_loc());

        // Reset mode
        self.set_mode(INIT_MODE);
        self.set_mode_steps(MODE_DURATIONS[INIT_MODE as usize]);

        // Reset level penalty
        self.set_level_steps(LEVEL_DURATION);

        // Clear fruit
        self.set_fruit_steps(0);

        // Reset ghosts
        self.reset_all_ghosts();

        // Reset pellets
        self.reset_pellets();
    }

    /************************** Motion (Pacman Location) **************************/

    pub fn move_pacman_dir(&mut self, dir: Direction) {
        // Ignore if paused
        if self.is_paused() || self.get_pause_on_update() {
            return;
        }

        // Calculate next position
        let (next_row, next_col) = self.pacman_loc.get_neighbor_coords(dir);

        // Update direction
        self.pacman_loc.update_dir(dir);

        // Check wall
        if self.wall_at(next_row, next_col) {
            return;
        }

        // Move pacman
        self.pacman_loc.update_coords(next_row, next_col);
        self.collect_pellet(next_row, next_col);

        // Check collisions
        self.check_collisions();
    }

    pub fn move_pacman_absolute(&mut self, new_row: i8, new_col: i8) {
        if self.is_paused() || self.get_pause_on_update() {
            return;
        }

        if self.wall_at(new_row, new_col) {
            return;
        }

        let (cur_row, cur_col) = self.pacman_loc.get_coords();
        if cur_row == new_row && cur_col == new_col {
            return;
        }

        // Find likely path via BFS
        let path = self.find_likely_path(new_row, new_col);

        let path = match path {
            Some(p) => p,
            None => {
                warn!("ERR: Failed to find correct path");
                return;
            }
        };

        // If path too long, move directly
        if path.len() > 11 {
            warn!(
                "WARN: Interpolated path too long! Tracking performance is likely degraded"
            );
            self.pacman_loc.update_coords(new_row, new_col);
            self.collect_pellet(new_row, new_col);
            self.check_collisions();
            return;
        }

        // Move along path
        let mut prev = (cur_row, cur_col);
        for &(next_r, next_c) in &path {
            let dir = if next_r < prev.0 {
                Direction::Up
            } else if next_c < prev.1 {
                Direction::Left
            } else if next_r > prev.0 {
                Direction::Down
            } else {
                Direction::Right
            };
            self.move_pacman_dir(dir);
            prev = (next_r, next_c);
        }
    }

    pub fn find_likely_path(&self, new_row: i8, new_col: i8) -> Option<Vec<(i8, i8)>> {
        let start = (self.pacman_loc.row, self.pacman_loc.col);
        let target = (new_row, new_col);

        let mut queue: VecDeque<(i8, i8)> = VecDeque::new();
        let mut parent: HashMap<(i8, i8), (i8, i8)> = HashMap::new();

        queue.push_back(start);
        parent.insert(start, (-1, -1));

        let mut found = false;

        while let Some(curr) = queue.pop_front() {
            let neighbors = [
                (curr.0 + 1, curr.1),
                (curr.0, curr.1 + 1),
                (curr.0 - 1, curr.1),
                (curr.0, curr.1 - 1),
            ];

            for adj in neighbors {
                if parent.contains_key(&adj) {
                    continue;
                }
                if self.wall_at(adj.0, adj.1) {
                    continue;
                }
                parent.insert(adj, curr);
                if adj == target {
                    found = true;
                    break;
                }
                queue.push_back(adj);
            }

            if found {
                break;
            }
        }

        if !found {
            return None;
        }

        // Backtrack path
        let mut path = Vec::new();
        let mut current = target;
        while current != start {
            path.push(current);
            current = match parent.get(&current) {
                Some(&p) => p,
                None => return None,
            };
        }
        path.reverse();

        // Verify path connects to start
        if let Some(&first) = path.first() {
            if parent.get(&first) != Some(&start) && first != start {
                // Check the parent of first element is start
                let first_parent = parent.get(&first);
                if first_parent != Some(&start) {
                    return None;
                }
            }
        }

        Some(path)
    }

    pub fn try_respawn_pacman(&mut self) {
        if self.pacman_loc.is_empty() && self.get_lives() > 0 {
            self.pacman_loc.copy_from(&pacman_spawn_loc());
        }
    }

    /******************************* Ghost Movement *******************************/

    pub fn frighten_all_ghosts(&mut self) {
        self.ghost_combo = 0;
        for i in 0..NUM_COLORS {
            self.ghosts[i].set_fright_steps(GHOST_FRIGHT_STEPS);
            if !self.ghosts[i].is_trapped() {
                self.ghosts[i].set_trapped_steps(1);
            }
        }
    }

    pub fn reverse_all_ghosts(&mut self) {
        for i in 0..NUM_COLORS {
            if !self.ghosts[i].is_trapped() {
                self.ghosts[i].set_trapped_steps(1);
            }
        }
    }

    pub fn reset_all_ghosts(&mut self) {
        self.ghost_combo = 0;
        let spawn_locs = ghost_spawn_locs();

        for i in 0..NUM_COLORS {
            let color = i as u8;
            if color >= NUM_ACTIVE_GHOSTS {
                continue;
            }

            self.ghosts[i].set_spawning(true);
            self.ghosts[i].set_trapped_steps(GHOST_TRAPPED_STEPS[i]);
            self.ghosts[i].set_fright_steps(0);
            self.ghosts[i].loc.copy_from(&empty_loc());
            self.ghosts[i].next_loc.copy_from(&spawn_locs[i]);
        }

        // If no lives left, ghosts stare at player menacingly
        if self.get_lives() == 0 {
            for i in 0..NUM_COLORS {
                if self.ghosts[i].color != ORANGE {
                    self.ghosts[i].next_loc.update_dir(Direction::None);
                } else {
                    self.ghosts[i].next_loc.update_dir(Direction::Left);
                }
            }
        }
    }

    pub fn respawn_ghosts(&mut self, ghost_respawn_flag: u8) {
        let spawn_locs = ghost_spawn_locs();

        for i in 0..NUM_COLORS {
            let color = self.ghosts[i].color;
            if !get_bit_u8(ghost_respawn_flag, color) {
                continue;
            }

            if color >= NUM_ACTIVE_GHOSTS {
                continue;
            }

            // Set ghost as eaten and spawning
            self.ghosts[i].set_spawning(true);
            self.ghosts[i].set_eaten(true);

            // Move to empty location
            self.ghosts[i].loc.copy_from(&empty_loc());

            // Set next location to spawn point (red goes to pink's spawn)
            if color == RED {
                let (pr, pc) = spawn_locs[PINK as usize].get_coords();
                self.ghosts[i].next_loc.update_coords(pr, pc);
            } else {
                self.ghosts[i]
                    .next_loc
                    .copy_from(&spawn_locs[color as usize]);
            }
            self.ghosts[i].next_loc.update_dir(Direction::Up);

            // Score combo
            self.increment_score(COMBO_MULTIPLIER << self.ghost_combo as u16);
            self.ghost_combo += 1;
        }
    }

    pub fn update_all_ghosts(&mut self) {
        let spawn_locs = ghost_spawn_locs();

        for i in 0..NUM_COLORS {
            if self.ghosts[i].color >= NUM_ACTIVE_GHOSTS {
                continue;
            }

            // Check if ghost reached red spawn and is done spawning
            if self.ghosts[i].loc.collides_with(&spawn_locs[RED as usize])
                && self.ghosts[i].loc.dir != Direction::Down
            {
                self.ghosts[i].set_spawning(false);
            }

            // Clear eaten flag
            if self.ghosts[i].is_eaten() {
                self.ghosts[i].set_eaten(false);
                self.ghosts[i].set_fright_steps(0);
            }

            // Decrement fright steps
            if self.ghosts[i].is_frightened() {
                self.ghosts[i].dec_fright_steps();
            }

            // Copy next_loc into loc
            let next = self.ghosts[i].next_loc.clone();
            self.ghosts[i].loc.copy_from(&next);
        }
    }

    pub fn plan_all_ghosts(&mut self) {
        // We need to plan each ghost using game state info
        // Since ghost.plan() needs &GameState but ghosts are part of GameState,
        // we clone relevant data to avoid borrow conflicts
        let pacman_loc = self.pacman_loc.clone();
        let red_loc = self.ghosts[RED as usize].loc.clone();
        let last_unpaused_mode = self.get_last_unpaused_mode();

        for i in 0..NUM_COLORS {
            if self.ghosts[i].color >= NUM_ACTIVE_GHOSTS {
                continue;
            }

            // Skip if location is empty (after reset/respawn)
            if self.ghosts[i].loc.is_empty() {
                continue;
            }

            // Advance next_loc from current loc
            let loc_clone = self.ghosts[i].loc.clone();
            self.ghosts[i].next_loc.advance_from(&loc_clone);

            // If trapped, reverse direction and return
            if self.ghosts[i].is_trapped() {
                let rev_dir = self.ghosts[i].next_loc.get_reversed_dir();
                self.ghosts[i].next_loc.update_dir(rev_dir);
                self.ghosts[i].dec_trapped_steps();
                continue;
            }

            let fright_steps = self.ghosts[i].get_fright_steps();
            let spawning = self.ghosts[i].is_spawning();
            let color = self.ghosts[i].color;

            // Decide target based on mode
            let spawn_locs = ghost_spawn_locs();
            let (target_row, target_col) = if spawning
                && !self.ghosts[i].loc.collides_with(&spawn_locs[RED as usize])
                && !self.ghosts[i].next_loc.collides_with(&spawn_locs[RED as usize])
            {
                spawn_locs[RED as usize].get_coords()
            } else if last_unpaused_mode == CHASE {
                self.get_chase_target_with_data(color, &pacman_loc, &red_loc)
            } else {
                self.ghosts[i].scatter_target.get_coords()
            };

            // Validate each direction
            let mut num_valid = 0;
            let mut move_valid = [false; NUM_DIRS as usize];
            let mut move_dist_sq = [0i32; NUM_DIRS as usize];

            for dir_idx in 0..NUM_DIRS {
                let dir = Direction::from_index(dir_idx);
                let (row, col) = self.ghosts[i].next_loc.get_neighbor_coords(dir);

                move_dist_sq[dir_idx as usize] = self.dist_sq(row, col, target_row, target_col);

                let mut valid = !self.wall_at(row, col);

                if spawning {
                    if self.ghost_spawn_at(row, col) {
                        valid = true;
                    }
                    if row == GHOST_HOUSE_EXIT_ROW && col == GHOST_HOUSE_EXIT_COL {
                        valid = true;
                    }
                }

                // Can't reverse direction
                if dir == self.ghosts[i].next_loc.get_reversed_dir() {
                    valid = false;
                }

                move_valid[dir_idx as usize] = valid;
                if valid {
                    num_valid += 1;
                }
            }

            if num_valid == 0 {
                let (row, col) = self.ghosts[i].next_loc.get_coords();
                warn!(
                    "WARN: {} has nowhere to go (row = {}, col = {}, dir = {}, spawning = {})",
                    GHOST_NAMES[color as usize],
                    row,
                    col,
                    self.ghosts[i].next_loc.dir,
                    spawning
                );
                continue;
            }

            // If frightened, choose random valid direction
            if fright_steps > 1 {
                let random_num = self.rng.gen_range(0..num_valid);
                let mut count = 0;
                for dir_idx in 0..NUM_DIRS {
                    if !move_valid[dir_idx as usize] {
                        continue;
                    }
                    if count == random_num {
                        self.ghosts[i]
                            .next_loc
                            .update_dir(Direction::from_index(dir_idx));
                        break;
                    }
                    count += 1;
                }
                continue;
            }

            // Choose direction that minimizes distance to target
            let mut best_dir = Direction::Up;
            let mut best_dist = i32::MAX;
            for dir_idx in 0..NUM_DIRS {
                if !move_valid[dir_idx as usize] {
                    continue;
                }
                if move_dist_sq[dir_idx as usize] < best_dist {
                    best_dir = Direction::from_index(dir_idx);
                    best_dist = move_dist_sq[dir_idx as usize];
                }
            }
            self.ghosts[i].next_loc.update_dir(best_dir);
        }
    }

    /************************ Ghost Targeting (Chase Mode) ************************/

    fn get_chase_target_with_data(
        &self,
        color: u8,
        pacman_loc: &LocationState,
        red_loc: &LocationState,
    ) -> (i8, i8) {
        match color {
            RED => pacman_loc.get_coords(),
            PINK => pacman_loc.get_ahead_coords(4),
            CYAN => {
                let (pivot_row, pivot_col) = pacman_loc.get_ahead_coords(2);
                let (red_row, red_col) = red_loc.get_coords();
                (2 * pivot_row - red_row, 2 * pivot_col - red_col)
            }
            ORANGE => {
                let (pac_row, pac_col) = pacman_loc.get_coords();
                let (orange_row, orange_col) = self.ghosts[ORANGE as usize].loc.get_coords();
                if self.dist_sq(orange_row, orange_col, pac_row, pac_col) >= 64 {
                    (pac_row, pac_col)
                } else {
                    self.ghosts[ORANGE as usize].scatter_target.get_coords()
                }
            }
            _ => empty_loc().get_coords(),
        }
    }

    pub fn get_chase_target(&self, color: u8) -> (i8, i8) {
        self.get_chase_target_with_data(color, &self.pacman_loc.clone(), &self.ghosts[RED as usize].loc.clone())
    }
}

// Helper for u8 bit operations (used for ghost respawn flag)
fn get_bit_u8(num: u8, bit_idx: u8) -> bool {
    ((num >> bit_idx) & 1) == 1
}

fn modify_bit_u8(num: &mut u8, bit_idx: u8, val: bool) {
    if val {
        *num |= 1 << bit_idx;
    } else {
        *num &= !(1 << bit_idx);
    }
}
