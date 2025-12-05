use std::fmt::{self};
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

#[derive(PartialEq, Debug, Clone)]
pub enum Item {
    Key(u8),
    Paper(String),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Key(number) => write!(f, "Ключ №{}", number),
            Item::Paper(..) => write!(f, "Записка"),
        }
    }
}

pub enum Cell {
    Wall,
    Pass,
    Door { direction: SideOfTheWorld, number: u8, state: bool },
    Key { number: u8 },
    Toggle { state: bool, number: u8, direction: SideOfTheWorld },
    LiftingGates { state: bool, number: u8, direction: SideOfTheWorld },
    Box {items: Vec<Item>},
    Safe {state: bool, direction: SideOfTheWorld, password: u16, items: Vec<Item> },
    Exit,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Wall => write!(f, "Стена"),
            Cell::Pass => write!(f, "Проход"),
            Cell::Door { number, .. } => write!(f, "Дверь №{}", number),
            Cell::Key { number } => write!(f, "Ключ №{}", number),
            Cell::Toggle { state, .. } => write!(f, "Рубильник {}", state),
            Cell::LiftingGates { state, number, .. } => write!(f, "Ворота №{} {}", number, if *state { "открыты" } else { "закрыты" }),
            Cell::Box { .. } => write!(f, "Ящик"),
            Cell::Safe { .. } => write!(f, "Сейф"),
            Cell::Exit => write!(f, "Выход")
        }
    }
}

pub const FIELD_HEIGHT: usize = 10;
pub const FIELD_WIDTH: usize = 20;

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