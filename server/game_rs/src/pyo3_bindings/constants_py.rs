// pyo3 bindings for constants

use pyo3::prelude::*;
use crate::constants;

pub fn register_constants(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Maze dimensions
    m.add("MAZE_ROWS", constants::MAZE_ROWS)?;
    m.add("MAZE_COLS", constants::MAZE_COLS)?;

    // Game modes
    m.add("PAUSED", constants::PAUSED)?;
    m.add("SCATTER", constants::SCATTER)?;
    m.add("CHASE", constants::CHASE)?;

    // Ghost colors
    m.add("RED", constants::RED)?;
    m.add("PINK", constants::PINK)?;
    m.add("CYAN", constants::CYAN)?;
    m.add("ORANGE", constants::ORANGE)?;
    m.add("NUM_COLORS", constants::NUM_COLORS)?;

    // Scoring
    m.add("PELLET_POINTS", constants::PELLET_POINTS)?;
    m.add("SUPER_PELLET_POINTS", constants::SUPER_PELLET_POINTS)?;
    m.add("FRUIT_POINTS", constants::FRUIT_POINTS)?;
    m.add("COMBO_MULTIPLIER", constants::COMBO_MULTIPLIER)?;

    // Game settings
    m.add("INIT_LIVES", constants::INIT_LIVES)?;
    m.add("INIT_LEVEL", constants::INIT_LEVEL)?;
    m.add("INIT_PELLET_COUNT", constants::INIT_PELLET_COUNT)?;
    m.add("GHOST_FRIGHT_STEPS", constants::GHOST_FRIGHT_STEPS)?;
    m.add("FRUIT_DURATION", constants::FRUIT_DURATION)?;

    Ok(())
}
