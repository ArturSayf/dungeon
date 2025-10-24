use std::{io, usize};

#[derive(PartialEq, Debug)]
enum SideOfTheWorld {
    North,
    South,
    West,
    East,
}
struct Character {
    x: usize,
    y: usize,
    side_of_the_world: SideOfTheWorld,
}

impl Character {
    fn valid_move(&mut self, dx: isize, dy: isize, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool{
    /*let nx = self.x as isize;
    let ny = self.y as isize;

    if nx < 0 || ny < 0 {
        false;
    }

    let  nx = nx as usize;
    let  ny = ny as usize;*/

    let nx = match self.x.checked_add_signed(dx) {
      Some(v) => v,
      None => return false,  
    };
    let ny = match self.y.checked_add_signed(dy) {
      Some(v) => v,
      None => return false,  
    };

    if nx < FIELD_WIDTH && ny < FIELD_HEIGHT{
        if field[ny][nx] == Cell::Pass{
            self.x = nx;
            self.y = ny;
            true
        } else {
            false
        }
    } else {
        false
    }

}

    fn move_forward(&mut self, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(0,-1, field),
            SideOfTheWorld::South => self.valid_move(0, 1, field),
            SideOfTheWorld::East => self.valid_move(1, 0, field),
            SideOfTheWorld::West => self.valid_move(-1, 0, field),
        }

    }

    fn move_back(&mut self, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(0,1, field),
            SideOfTheWorld::South => self.valid_move(0, -1, field),
            SideOfTheWorld::East => self.valid_move(-1, 0, field),
            SideOfTheWorld::West => self.valid_move(1, 0, field),
        }
    }

    fn move_left(&mut self, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(-1,0, field),
            SideOfTheWorld::South => self.valid_move(1, 0, field),
            SideOfTheWorld::East => self.valid_move(0, -1, field),
            SideOfTheWorld::West => self.valid_move(0, 1, field),
        }
    }

    fn move_right(&mut self, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(1,0, field),
            SideOfTheWorld::South => self.valid_move(-1, 0, field),
            SideOfTheWorld::East => self.valid_move(0, 1, field),
            SideOfTheWorld::West => self.valid_move(0, -1, field),
        }
    }

    fn turn_left(&mut self) {
        self.side_of_the_world = match self.side_of_the_world {
            SideOfTheWorld::North => SideOfTheWorld::West,
            SideOfTheWorld::South => SideOfTheWorld::East,
            SideOfTheWorld::East => SideOfTheWorld::North,
            SideOfTheWorld::West => SideOfTheWorld::South,
        }
    }

    fn turn_right(&mut self) {
        self.side_of_the_world = match self.side_of_the_world {
            SideOfTheWorld::North => SideOfTheWorld::East,
            SideOfTheWorld::South => SideOfTheWorld::West,
            SideOfTheWorld::East => SideOfTheWorld::South,
            SideOfTheWorld::West => SideOfTheWorld::North,
        }
    }

    fn turn_around(&mut self) {
        self.side_of_the_world = match self.side_of_the_world {
            SideOfTheWorld::North => SideOfTheWorld::South,
            SideOfTheWorld::South => SideOfTheWorld::North,
            SideOfTheWorld::East => SideOfTheWorld::West,
            SideOfTheWorld::West => SideOfTheWorld::East,
        }
    }
}

#[derive(PartialEq)]
enum Cell {
    Wall,
    Pass,
    Door,
    Key,
}

const FIELD_HEIGHT: usize = 9;
const FIELD_WIDTH: usize = 6;

fn main() {
    let field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT] = [
        [Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall],
        [Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Key],
        [Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Wall],
        [Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall],
        [Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass],
        [Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall],
        [Cell::Door, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass],
        [Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass],
        [Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass],
    ];

    let mut character: Character = Character {
        x: 0,
        y: 0,
        side_of_the_world: SideOfTheWorld::North,
    };

    println!("Двигайтесь! (left, right, forward, back, turn left, turn right, turn around)");
    loop {
        println!(
            "Ваши координаты: {}.{}, направление на {:?}",
            character.x, character.y, character.side_of_the_world
        );

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Ошибка!");
        input = input.trim().to_lowercase();

        match input.as_str() {
            "turn left" => {
                character.turn_left();
            }
            "turn right" => {
                character.turn_right();
            }
            "turn around" => {
                character.turn_around();
            }
            "forward" => {
                if !character.move_forward(&field) {
                    println!("Стена!")
                }
            },
            "back" => {
                if !character.move_back(&field) {
                    println!("Стена!")
                }
            },
            "left" => {
                if !character.move_left(&field) {
                    println!("Стена!")
                }
            },
            "right" => {
                if !character.move_right(&field) {
                    println!("Стена!")
                }
            },
            _ => println!("Неверная команда!"),
        }
    }
}
