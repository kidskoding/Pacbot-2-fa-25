pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    pub const dirs: [(i8, i8); 5] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (0, 0)
    ];

    pub fn get_dir(&self) -> (i8, i8) {
        match self {
            Direction::Up => Self::dirs[0],
            Direction::Down => Self::dirs[1],
            Direction::Left => Self::dirs[2],
            Direction::Right => Self::dirs[3],
            Direction::None => Self::dirs[4],
        }
    }

    pub fn get_drow(&self) -> i8 {
        self.get_dir().0
    }

    pub fn get_dcol(&self) -> i8 {
        self.get_dir().1
    }
}