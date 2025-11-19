use crate::game::field::{Cell, FIELD_WIDTH, FIELD_HEIGHT};
use crate::game::field::SideOfTheWorld;

 #[derive(PartialEq, Debug, Clone, Copy)]
pub struct Character {
    pub x: usize,
    pub y: usize,
    pub side_of_the_world: SideOfTheWorld,
    pub has_key: bool,
}

impl Character {
    pub fn valid_move(&mut self, dx: isize, dy: isize, field: &mut[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool{

    let nx = match self.x.checked_add_signed(dx) {
      Some(v) => v,
      None => return false,  
    };
    let ny = match self.y.checked_add_signed(dy) {
      Some(v) => v,
      None => return false,  
    };

    if nx < FIELD_WIDTH && ny < FIELD_HEIGHT{
        match field[ny][nx] {
            Cell::Wall => false,
            Cell::Pass => {
                self.x = nx;
                self.y = ny;
                true
            }
            Cell::Door  {direction: _ }=> {
                if self.has_key {
                        self.x = nx;
                        self.y = ny;
                        true
                    } else {
                        false
                    }
            }
            Cell::Key => {
                self.x = nx;
                    self.y = ny;
                    if !self.has_key {
                        self.has_key = true;
                        field[ny][nx] = Cell::Pass;
                    }
                    true
            }
        }
    } else {
        false
    }

}

    pub fn move_forward(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(0,-1, field),
            SideOfTheWorld::South => self.valid_move(0, 1, field),
            SideOfTheWorld::East => self.valid_move(1, 0, field),
            SideOfTheWorld::West => self.valid_move(-1, 0, field),
        }

    }

    pub fn move_back(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(0,1, field),
            SideOfTheWorld::South => self.valid_move(0, -1, field),
            SideOfTheWorld::East => self.valid_move(-1, 0, field),
            SideOfTheWorld::West => self.valid_move(1, 0, field),
        }
    }

    pub fn move_left(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(-1,0, field),
            SideOfTheWorld::South => self.valid_move(1, 0, field),
            SideOfTheWorld::East => self.valid_move(0, -1, field),
            SideOfTheWorld::West => self.valid_move(0, 1, field),
        }
    }

    pub fn move_right(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(1,0, field),
            SideOfTheWorld::South => self.valid_move(-1, 0, field),
            SideOfTheWorld::East => self.valid_move(0, 1, field),
            SideOfTheWorld::West => self.valid_move(0, -1, field),
        }
    }

    pub fn turn_left(&mut self) {
        self.side_of_the_world = match self.side_of_the_world {
            SideOfTheWorld::North => SideOfTheWorld::West,
            SideOfTheWorld::South => SideOfTheWorld::East,
            SideOfTheWorld::East => SideOfTheWorld::North,
            SideOfTheWorld::West => SideOfTheWorld::South,
        }
    }

    pub fn turn_right(&mut self) {
        self.side_of_the_world = match self.side_of_the_world {
            SideOfTheWorld::North => SideOfTheWorld::East,
            SideOfTheWorld::South => SideOfTheWorld::West,
            SideOfTheWorld::East => SideOfTheWorld::South,
            SideOfTheWorld::West => SideOfTheWorld::North,
        }
    }

    pub fn turn_around(&mut self) {
        self.side_of_the_world = match self.side_of_the_world {
            SideOfTheWorld::North => SideOfTheWorld::South,
            SideOfTheWorld::South => SideOfTheWorld::North,
            SideOfTheWorld::East => SideOfTheWorld::West,
            SideOfTheWorld::West => SideOfTheWorld::East,
        }
    }
}