use crate::coords::Pos;
use crate::ghost_state::GameState;
use std::thread;

const MAZE_ROWS: i8 = 31;
const MAZE_COLS: i8 = 28;
const FRUIT_POINTS: i32 = 100;
const FRUIT_THRESHOLD_1: i32 = 70; 
const FRUIT_THRESHOLD_2: i32 = 170; 
const FRUIT_DURATION: u16 = 500;
const ANGER_THRESHOLD_1: i32 = 190; 
const ANGER_THRESHOLD_2: i32 = 210;
const MODE_DURATIONS: [u32; 3] = [200, 800, 1000];
const UP: u8 = 0;
const DOWN: u8 = 0;
const LEFT: u8 = 0;
const RIGHT: u8 = 0;
const NONE: u8 = 0;
const GHOST_FRIGHT_STEPS= 0; 
const LEVEL_DURATION =0;
const RED: u8= 0; 
const PINK: u8 =1; 
const CYAN: u8 =2; 
const ORANGE: u8 =3;
const COMBO_MULTIPLIER: i32=0;
/*
Get a bit within an unsigned integer (treating the integers
in pellets and walls as bit arrays)
*/
pub fn get_bit<N, I>(num: N, bit_idx: I) -> bool 
where
    N: Into<u32> + Copy,
    I: Into<u32> + Copy,
{
    let num_val: u32 = num.into();
    let bit_idx_val: u32 = bit_idx.into();
	/*
		Uses bitwise operation magic (not really, look up how the >> and &
		operators work if you're interested)
	*/
	(num_val >> bit_idx_val) & 1 == 1
}

/*
Get a bit within an unsigned integer (treating the integers in pellets
and walls as bit arrays)
*/
fn modify_bit<N, I>(num: &mut N, bit_idx: I, bit_val: bool)
where
    N: From<u32> + Into<u32> + Copy,
    I: Into<u32> + Copy,
{
    let bit_idx_val: u32 = bit_idx.into();
    let num_val: u32 = (*num).into();
    let mask: u32 = 1 << bit_idx_val;
    // If the bit is true, we should set the bit, otherwise we clear it
    let new_num_val = if bit_val {
        num_val | mask
    } else {
        num_val & (!mask)
    };
    *num = N::from(new_num_val);
}

impl Pos {
    fn get_adjacent(&self) -> [Pos; 4] {
        [
            Pos {
                r: self.r + 1,
                c: self.c,
            },
            Pos {
                r: self.r,
                c: self.c + 1,
            },
            Pos {
                r: self.r - 1,
                c: self.c,
            },
            Pos {
                r: self.r,
                c: self.c - 1,
            },
        ]
    }
}

impl GameState{
/****************************** Timing Functions ******************************/

    // Determines if the game state is ready to update
    pub fn update_ready(&self) -> bool {

        // Get the current ticks value
        let curr_ticks = self.get_curr_tickets();

        // Get the update period (uint16 to match the type of current ticks)
        let update_period = self.get_update_period() as u16;

        // Update if the update period divides the current ticks
        curr_ticks%update_period == 0
    }

    /**************************** Positional fnctions ****************************/

    // Determines if a position is within the bounds of the maze
    pub fn in_bounds(&self, row: i8, col: i8) -> bool {
         (row >= 0 && row < MAZE_ROWS) && (col >= 0 && col < MAZE_COLS)
    }

    // Determines if a pellet is at a given location
    pub fn pellet_at(&self, row: i8, col: i8) -> bool {
        if !self.in_bounds(row, col) {
            return false;
        }

        // (Read) lock the pellets array
        // gs.muPellets.RLock()
        // defer gs.muPellets.RUnlock()

        // Returns the bit of the pellet row corresponding to the column
        get_bit(self.pellets[row as usize], col as u32)
    }

    /*
    Collects a pellet if it is at a given location
    Returns the number of pellets that are left
    */
    pub fn collect_pellet(&mut self, row: i8, col: i8) {

        // Collect fruit, if applicable
        if self.fruit_exists() && self.pacman_loc.collides_with(self.fruit_loc) {
            self.set_fruit_steps(0);
            self.increment_score(FRUIT_POINTS);
        }

        // If there's no pellet, return
        if !self.pellet_at(row, col) {
            return;
        }

        // If we can clear the pellet's bit, decrease the number of pellets
        modify_bit(&mut self.pellets[row as usize], col as u32, false);
        self.decrement_num_pellets();

        // If the we are in particular rows and columns, it is a super pellet
        let super_pellet = ((row == 3) || (row == 23)) && ((col == 1) || (col == 26));

        // Make all the ghosts frightened if a super pellet is collected
        if super_pellet {
            self.frighten_all_ghosts();
        }

        // Update the score, depending on the pellet type
        if super_pellet {
            self.increment_score(super_pellet_points);
        } else {
            self.increment_score(pellet_points);
        }

        // Act depending on the number of pellets left over
        let num_pellets = self.get_num_pellets();

        // Spawn fruit, if applicable
        if (num_pellets == FRUIT_THRESHOLD_1) && !self.fruit_exists() {
            self.set_fruit_steps(FRUIT_DURATION);
        } else if num_pellets == FRUIT_THRESHOLD_2 && !self.fruit_exists() {
            self.set_fruit_steps(FRUIT_DURATION);
        }

        // Other pellet-related events
        if num_pellets == ANGER_THRESHOLD_1 { // Ghosts get angry (speeding up)
            let new_period = std::cmp::max(1, self.get_update_period() as i32 - 2) as u8;
            self.set_update_period(new_period);
            self.set_mode(GameMode::Chase);
            self.set_mode_steps(MODE_DURATIONS[GameMode::Chase as usize])
        } else if num_pellets == ANGER_THRESHOLD_2 { // Ghosts get angrier
            let new_period = std::cmp::max(1, self.get_update_period() as i32 - 2) as u8;
            self.set_update_period(new_period);
            self.set_mode(GameMode::Chase);
            self.set_mode_steps(MODE_DURATIONS[GameMode::Chase as usize]);
        } else if num_pellets == 0 {
            self.level_reset();
            self.increment_level();
        }
    }

    // Determines if a wall is at a given location
    pub fn wall_at(&self, row: i8, col: i8) -> bool {
        if !self.in_bounds(row, col) {
            return true;
        }

        // Returns the bit of the wall row corresponding to the column
        get_bit(self.walls[row as usize], col)
    }

    // Determines if the ghost house is at a given location
    pub fn ghost_spawn_at(&self, row: i8, col: i8) -> bool {
        if !self.in_bounds(row, col) {
            return false
        }

        // Returns the bit of the wall row corresponding to the column
        ((row >= 13) && (row <= 14)) && ((col >= 11) && (col <= 15))
    }

    // Calculates the squared Euclidean distance between two points
    pub fn dist_sq(&self, row1: i8, col1: i8, row2: i8, col2: i8) -> i32 {
        let dx = row2 as i32 - row1 as i32;
        let dy = col2 as i32 - col1 as i32;
        dx*dx + dy*dy
    }

    /***************************** Collision Handling *****************************/

    // Check collisions between Pacman and all the ghosts
    pub fn check_collisions(& mut self) {

        // Flag to decide which ghosts should respawn
        let mut  ghost_respawn_flag: u8 = 0;

        // Keep track of how many ghosts need to respawn
        let mut num_ghost_respawns = 0;

        // Loop over all the ghosts
        for i in 0..self.ghosts.len() {
            let ghost = &mut self.ghosts[i];
            // Check each collision individually
            if self.pacman_loc.collides_with(&ghost.loc) {

                // If the ghost was already eaten, skip it
                if ghost.is_eaten() {
                    continue;
                }

                // If the ghost is frightened, Pacman eats it, otherwise Pacman dies
                if ghost.is_frightened() {
                    modify_bit(&mut ghost_respawn_flag, ghost.color, true);
                    num_ghost_respawns+=1;
                } else {
                    self.death_reset();
                    return;
                }
            }
        }

        // If no ghosts need to respawn, there's no more work to do
        if num_ghost_respawns == 0 {
            return;
        }

        // Lock the motion mutex to synchronize with other ghost update routines
        self.respawn_ghosts(num_ghost_respawns, ghost_respawn_Flag);
    }

    /***************************** Event-Based Resets *****************************/

    // Reset the board (while leaving pellets alone) after Pacman dies
    pub fn death_reset(&mut self) {

        // Acquire the Pacman control lock, to prevent other Pacman movement
        let _pacman_lock = self.mu_pacman.lock().unwrap();

        // Set the game to be paused at the next update
        self.set_pause_on_update(true);

        // Set Pacman to be in an empty state
        self.pacman_loc.copy_from(Pos::EMPTY_LOC);

        // Decrease the number of lives Pacman has left
        self.decrement_lives();

        /*
            If the mode is not the initial mode and the ghosts aren't angry,
            change the mode back to the initial mode
        */
        if self.get_num_pellets() > ANGER_THRESHOLD_1 {
            self.set_mode(GameMode::Init);
            self.set_mode_steps(MODE_DURATIONS[GameMode::Init as usize]);
        }

        // Set the fruit steps back to 0
        self.set_fruit_steps(0);

        // Reset all the ghosts to their original locations
        self.reset_all_ghosts();
    }

    // Reset the board (including pellets) after Pacman clears a level
    pub fn level_reset(& mut self) {

        // Set the game to be paused at the next update
        self.set_pause_on_update(true);

        // Set Pacman to be in an empty state
        self.pacman_loc.copy_from(Pos::EMPTY_LOC);

        // If the mode is not the initial mode, change it
        self.set_mode(GameMode::Init);
        self.set_mode_steps(MODE_DURATIONS[GameMode::Init as usize]);

        // Reset the level penalty
        self.set_level_steps(LEVEL_DURATION);

        // Set the fruit steps back to 0
        self.set_fruit_steps(0);

        // Reset all the ghosts to their original locations
        self.reset_all_ghosts();

        // Reset the pellet bit array and count
        self.reset_pellets();
    }

    /************************** Motion (Pacman Location) **************************/

    // Move Pacman one space in a given direction
    pub fn move_pacman_dir(&mut self, dir: u8) {

        // Acquire the Pacman control lock, to prevent other Pacman movement
        let _pacman_lock = self.mu_pacman.lock().unwrap();

        // Ignore the command if the game is paused
        if self.is_paused() || self.get_pause_on_update() {
            return;
        }

        // Shorthand to make computation simpler
        let p_loc = &mut self.pacman_loc;

        // Calculate the next row and column
        let (next_row, next_col) = p_loc.get_neighbor_coords(dir);

        // Update Pacman's direction
        p_loc.update_dir(dir);

        // Check if there is a wall at the anticipated location, and return if so
        if self.wall_at(next_row, next_col) {
            return;
        }

        // Move Pacman the anticipated spot
        p_loc.update_coords(next_row, next_col);
        self.collect_pellet(next_row, next_col);
        self.check_collisions();
    }

    // Move pacman to destination along shortest path (CV update)
    pub fn move_pacman_absolute(&mut self, new_row: i8, new_col: i8) {
        // Don't update position if we're paused
        if self.is_paused() || self.get_pause_on_update() {
            return;
        }

        // Reject invalid coords
        if self.wall_at(new_row, new_col) {
            return;
        }

        let p_loc = &mut self.pacman_loc;

        // Reject same coords
        if p_loc.row == new_row && p_loc.col == new_col {
            return;
        }

        // Find likely path
        let path = self.find_likely_path(new_row, new_col);

        // This really shouldn't happen but somehow the pathfinding has failed
        let path = match path {
            Some(p) => p,
            None => {
                eprintln!("\x1b[31mERR: Failed to find correct path\x1b[0m");
                return;
            }
        };

        // The new position is far from the old one, let's not traverse the path
        if path.len() > 11 {
            eprintln!(
                "\x1b[35mWARN: Interpolated path too long! \
                 Tracking performance is likely degraded\x1b[0m"
            );

            // Acquire the Pacman control lock, to prevent other Pacman movement
            let _pacman_lock = self.mu_pacman.lock().unwrap();

            // Move Pacman directly to the given position
            self._pacman_lock.update_coords(new_row, new_col);
            self.collect_pellet(new_row, new_col);
            // gs.collect_pellet(newRow, newCol);
            self.check_collisions();
            return;
        }

        let mut prev_pos = Pos {
            r: self.pacman_loc.row,
            c: self.pacman_loc.col,
        };
        // Move Pacman along the detected route
        for next_pos in path.into_iter() {
            let dir = if next_pos.r < prev_pos.r {
                UP
            } else if next_pos.c < prev_pos.c {
                LEFT
            } else if next_pos.r > prev_pos.r {
                DOWN
            } else {
                RIGHT
            };
            self.move_pacman_dir(dir);
            prev_pos = next_pos;
        }
    }

    // Find likely/shortest path to new coords
    // precondition: lock pacman pos
    pub fn find_likely_path(&self, new_row: i8, new_col: i8) -> Option<Vec<Pos>> {
        // Begin breadth-first search
        let start = Pos {
            r: self.pacman_loc.row,
            c: self.pacman_loc.col,
        };
        let mut queue = std::collections::VecDeque::from(vec![start]); // Rust's BFS queue
        let mut parent = std::collections::HashMap::new();
        parent.insert(
            start,
            Pos {r: -1, c: -1,},
        );
        // Define target position
        let target = Pos {
            r: new_row,
            c: new_col,
        };

        let mut found = false;
        // Keep searching until we have exhausted all options or found it
        while let Some(curr) = queue.pop_front() {
            if curr == target {
                found = true;
                break;
            }

            // Find adjacencies/neighbors of current cell
            for adj in curr.get_adjacent().into_iter() {

                // Already searched this one, continue
                if parent.contains_key(&adj) {
                    continue;
                }

                // Skip walls
                if self.wall_at(adj.r, adj.c) {
                    continue;
                }

                // We can validly travel to the destination cell from curr
                parent.insert(adj, curr);

                queue.push_back(adj);
            }
        }
        if !found {
            return None;
        }
        // Backtrack the path
        let mut path: Vec<Pos> = Vec::new();
        let mut last = target;
        loop {
            let &prev = parent.get(&last).unwrap();
            
            if prev.r == -1 && prev.c == -1 {
                break;
            }
            path.push(last);

            if prev == start {
                break;
            }

            last = prev;
        }
        if path.is_empty() {
            return None;
        }
        path.reverse();
            Some(path)
    }

    // Move Pacman back to its spawn point, if necessary
    pub fn try_respawn_pacman(&mut self) {
        // Acquire the Pacman control lock, to prevent other Pacman movement
        let _pacman_lock = self.mu_pacman.lock().unwrap();

        // Set Pacman to be in its original state
        if self.pacman_loc.is_empty() && self.get_lives() > 0 {
            self.pacman_loc.copy_from(Pos::PACMAN_SPAWN_LOC);
        }
    }

}





/******************************* Ghost Movement *******************************/

// Frighten all ghosts at once
pub fn frighten_all_ghosts(&mut self) {

	// Acquire the ghost control lock, to prevent other ghost movement decisions
	let _ghosts_lock = self.mu_ghosts.lock().unwrap();

	// Reset the ghost respawn combo back to 0
	self.ghost_combo = 0;

	// Loop over all the ghosts
	for ghost in self.ghosts.iter_mut() {

		/*
			To frighten a ghost, set its fright steps to a specified value
			and trap it for one step (to force the direction to reverse)
		*/
		ghost.set_fright_steps(GHOST_FRIGHT_STEPS);
            if !ghost.is_trapped() {
                ghost.set_trapped_steps(1);
            }
	}
}

// Reverse all ghosts at once (similar to frightenAllGhosts)
pub fn reverse_all_ghosts(&mut self) {

	// Loop over all the ghosts
	for ghost in self.ghosts.iter_mut() {

		/*
			To change the direction a ghost, trap it for one step
			(to force the direction to reverse)
		*/
		if !ghost.is_trapped() {
            ghost.set_trapped_steps(1);
        }
	}
}

// Reset all ghosts at once
pub fn reset_all_ghosts(&mut self) {

	// Acquire the ghost control lock, to prevent other ghost movement
	let _ghosts_lock = self.mu_ghosts.lock().unwrap();

	// Reset the ghost respawn combo back to 0
	self.ghost_combo = 0;

    
	// Reset each of the ghosts
	// let mut handles = vec![];
    for ghost in self.ghosts.iter() {
        // let handle = thread::spawn(move || {
            ghost.reset();
    //    });
    //    handles.push(handle);
    }
	// Wait for the resets to finish
	// for handle in handles {
    //     handle.join().expect("Ghost reset thread panicked");
    // }

	// If no lives are left, set all ghosts to stare at the player, menacingly
	if self.get_lives() == 0 {
        for ghost in self.ghosts.iter_mut() {
            if ghost.color != ORANGE {
                ghost.next_loc.update_dir(NONE);
            } else {
                // Orange does like making eye contact, unfortunately
                ghost.next_loc.update_dir(LEFT);
            }
        }
    }
}

// Respawn some ghosts, according to a flag
pub fn respawn_ghosts(&mut self, _num_ghost_respawns: usize, ghost_respawn_flag: u8) {

	// Acquire the ghost control lock, to prevent other ghost movement
	
    // let mut handles = vec![];
    let _ghosts_lock = self.mu_ghosts.lock().unwrap();

	// Loop over the ghost colors again, to decide which should respawn
	for i in 0..self.ghosts.len() {
        let ghost = &self.ghosts[i];

		// If the ghost should respawn, do so and increase the score and combo
		if get_bit(ghost_respawn_flag, ghost.color) {
            // Respawn the ghost
            let ghost_respawn_clone = thread::spawn(move || {
                ghost.respawn();
            });
            handles.push(ghost_respawn_clone);

			// Add points corresponding to the current combo length
			let score_points =
                COMBO_MULTIPLIER as i32 * (1 << self.ghost_combo as u16) as i32;
            self.increment_score(score_points);

			// Increment the ghost respawn combo
			self.ghost_combo += 1;
		}
	}

	// Wait for the respawns to finish
	for handle in handles {
        handle.join().expect("Ghost respawn thread panicked");
    }
}

// Update all ghosts at once
pub fn update_all_ghosts(&mut self) {

	// Acquire the ghost control lock, to prevent other ghost movement
	let _ghosts_lock = self.mu_ghosts.lock().unwrap();
    let mut handles = vec![];

	// Loop over the individual ghosts
	for ghost in self.ghosts.iter() {
        let handle = thread::spawn(move || {
            ghost.update();
        });
        handles.push(handle);
    }

	// Wait for the respawns to finish
	for handle in handles {
        handle.join().expect("Ghost update thread panicked");
    }
}

// A game state function to plan all ghosts at once
pub fn plan_all_ghosts(&mut self) {

	// Acquire the ghost control lock, to prevent other ghost movement
	let _ghosts_lock = self.mu_ghosts.lock().unwrap();

    let mut handles = vec![];

	// Plan each ghost's next move concurrently
	for ghost in self.ghosts.iter() {
        let handle = thread::spawn(move || {
            ghost.plan();
        });
        handles.push(handle);
    }

	// Wait until all pending ghost plans are complete
	for handle in handles {
        handle.join().expect("Ghost plan thread panicked");
    }
}

/************************ Ghost Targeting (Chase Mode) ************************/

/*
Returns the chase location of the red ghost
(i.e. Pacman's exact location)
*/
pub fn get_chase_target_red(&self) -> (i8, i8) {

	// Return Pacman's current location
	self.pacman_loc.get_coords()
}

/*
Returns the chase location of the pink ghost
(i.e. 4 spaces ahead of Pacman's location)
*/
pub fn get_chase_target_pink(&self) -> (i8, i8) {

	// Return the red pink's target (4 spaces ahead of Pacman)
	self.pacman_loc.get_ahead_coords(4)
}

/*
Returns the chase location of the cyan ghost
(i.e. The red ghost's location, reflected about 2 spaces ahead of Pacman)
*/
pub fn get_chase_target_cyan(&self) -> (i8, i8) {

	// Get the 'pivot' square, 2 steps ahead of Pacman
	let (pivot_row, pivot_col) = self.pacman_loc.get_ahead_coords(2);

	// Get the current location of the red ghost
	let (red_row, red_col) = self.ghosts[RED as usize].loc.get_coords();

	// Return the pair of coordinates of the calculated target
	(2*pivot_row - red_row), (2*pivot_col - red_col)
}

/*
Returns the chase location of the orange ghost
(i.e. Pacman's exact location, the same as red's target most of the time)
Though, if close enough to Pacman, it should choose its scatter target
*/
pub fn get_chase_target_orange(&self) -> (i8, i8) {

	// Get Pacman's current location
	let (pacman_row, pacman_col) = self.pacman_loc.get_coords();

	// Get the orange ghost's current location
	let (orange_row, orange_col) = self.ghosts[ORANGE as usize].loc.get_coords();

	// If Pacman is far enough from the ghost, return Pacman's location
	if self.dist_sq(orange_row, orange_col, pacman_row, pacman_col) >= 64 {
        return (pacman_row, pacman_col)
    }

	// Otherwise, return the scatter location of orange
	self.ghosts[ORANGE as usize].scatter_target.get_coords()
}

// Returns the chase location of an arbitrary ghost color
pub fn get_chase_target(&self, color: u8) -> (i8, i8) {
	match color {
        RED => self.get_chase_target_red(),
        PINK => self.get_chase_target_pink(),
        CYAN => self.get_chase_target_cyan(),
        ORANGE => self.get_chase_target_orange(),
        _ => Pos::EMPTY_LOC.get_coords(),
    }
}
