use pyo3::prelude::*;
use crate::location::Direction;

#[pyclass]
#[derive(Clone)]
pub struct PyDirection {
    inner: Direction
}

#[pymethods]
impl PyDirection {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Direction::None
        }
    }

    #[staticmethod]
    pub fn up() -> Self {
        Self {
            inner: Direction::Up
        }
    }

    #[staticmethod]
    pub fn down() -> Self {
        Self {
            inner: Direction::Down
        }
    }

    #[staticmethod]
    pub fn left() -> Self {
        Self {
            inner: Direction::Left
        }
    }

    #[staticmethod]
    pub fn right() -> Self {
        Self {
            inner: Direction::Right
        }
    }

    #[staticmethod]
    pub fn none() -> Self {
        Self {
            inner: Direction::None
        }
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
        Direction::dirs.to_vec()
    }

    pub fn __str__(&self) -> String {
        match self.inner {
            Direction::Up => "Up".to_string(),
            Direction::Down => "Down".to_string(),
            Direction::Left => "Left".to_string(),
            Direction::Right => "Right".to_string(),
            Direction::None => "None".to_string(),
        }
    }

    pub fn __repr__(&self) -> String {
        format!("Direction.{}", self.__str__())
    }
}
