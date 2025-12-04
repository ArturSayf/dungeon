use std::fmt::{self};

pub enum Command {
    Forward,
    Back,
    Left,
    Right,
    TurnLeft,
    TurnRight,
    TurnAround,
    Map,
    Action,
    Inventory,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Forward => write!(f, "w"),
            Command::Back => write!(f, "s"),
            Command::Left => write!(f, "a"),
            Command::Right => write!(f, "d"),
            Command::TurnLeft => write!(f, "q"),
            Command::TurnRight => write!(f, "e"),
            Command::TurnAround => write!(f, "z"),
            Command::Map => write!(f, "m"),
            Command::Action => write!(f, "f"),
            Command::Inventory => write!(f, "i"),
        }
    }
}

impl Command {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "forward" | "w" => Some(Command::Forward),
            "back" | "s" => Some(Command::Back),
            "left" | "a" => Some(Command::Left),
            "right" | "d" => Some(Command::Right),
            "turn left" | "q" => Some(Command::TurnLeft),
            "turn right" | "e" => Some(Command::TurnRight),
            "turn around" | "z" => Some(Command::TurnAround),
            "Get Map" | "m" => Some(Command::Map),
            "Action" | "f" => Some(Command::Action),
            "Inventory" | "i" => Some(Command::Inventory),
            _ => None,
        }
    }
}