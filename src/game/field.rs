use std::fmt;

 #[derive(PartialEq, Debug, Clone, Copy)]
pub enum SideOfTheWorld {
    South,
    West,
    North,
    East,
}

impl fmt::Display for SideOfTheWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SideOfTheWorld::North => write!(f, "Север"),
            SideOfTheWorld::South => write!(f, "Юг"),
            SideOfTheWorld::West => write!(f, "Запад"),
            SideOfTheWorld::East => write!(f, "Восток"),
        }
    }
}

pub enum Cell {
    Wall,
    Pass,
    Door { direction: SideOfTheWorld },
    Key,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Wall => write!(f, "Стена"),
            Cell::Pass => write!(f, "Проход"),
            Cell::Door { .. } => write!(f, "Дверь"),
            Cell::Key => write!(f, "Ключ"),
        }
    }
}

pub const FIELD_HEIGHT: usize = 7;
pub const FIELD_WIDTH: usize = 9;

pub struct MapVisibility {
    pub discovered: [[bool; FIELD_WIDTH]; FIELD_HEIGHT],
}

impl MapVisibility {
    pub fn new() -> Self {
        Self {
            discovered: [[false; FIELD_WIDTH]; FIELD_HEIGHT],
        }
    }

    pub fn update_visibility(&mut self, x: usize, y: usize) {
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                
                if nx >= 0 && nx < FIELD_WIDTH as isize && ny >= 0 && ny < FIELD_HEIGHT as isize {
                    self.discovered[ny as usize][nx as usize] = true;
                }
            }
        }
    }

    pub fn is_discovered(&self, x: usize, y: usize) -> bool {
        self.discovered[y][x]
    }
}