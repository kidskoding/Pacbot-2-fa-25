// pyo3 bindings for direction.rs and location.rs

use pyo3::prelude::*;
use crate::direction::Direction;
use crate::location::LocationState;

#[pyclass]
#[derive(Clone)]
pub struct PyDirection {
    inner: Direction,
}

#[pymethods]
impl PyDirection {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Direction::None,
        }
    }

    #[staticmethod]
    pub fn up() -> Self {
        Self { inner: Direction::Up }
    }

    #[staticmethod]
    pub fn down() -> Self {
        Self { inner: Direction::Down }
    }

    #[staticmethod]
    pub fn left() -> Self {
        Self { inner: Direction::Left }
    }

    #[staticmethod]
    pub fn right() -> Self {
        Self { inner: Direction::Right }
    }

    #[staticmethod]
    pub fn none() -> Self {
        Self { inner: Direction::None }
    }

    pub fn get_dir(&self) -> (i8, i8) {
        self.inner.get_dir()
    }

    pub fn get_drow(&self) -> i8 {
        self.inner.get_drow()
    }

    pub fn get_dcol(&self) -> i8 {
        self.inner.get_dcol()
    }

    #[staticmethod]
    pub fn get_all_dirs() -> Vec<(i8, i8)> {
        Direction::DIRS.to_vec()
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    pub fn __repr__(&self) -> String {
        format!("Direction.{}", self.inner)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyLocationState {
    inner: LocationState,
}

#[pymethods]
impl PyLocationState {
    #[new]
    pub fn new(row: i8, col: i8, dir: Option<&str>) -> Self {
        let direction = match dir.unwrap_or("none") {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => Direction::None,
        };
        Self {
            inner: LocationState::new(row, col, direction),
        }
    }

    pub fn get_coords(&self) -> (i8, i8) {
        self.inner.get_coords()
    }

    pub fn get_row(&self) -> i8 {
        self.inner.row
    }

    pub fn get_col(&self) -> i8 {
        self.inner.col
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "LocationState(row={}, col={}, dir={})",
            self.inner.row, self.inner.col, self.inner.dir
        )
    }
}
