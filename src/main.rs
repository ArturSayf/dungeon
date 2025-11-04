use std::{io, usize};

 #[derive(PartialEq, Debug, Clone, Copy)]
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
    has_key: bool,
}

impl Character {
    fn valid_move(&mut self, dx: isize, dy: isize, field: &mut[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool{

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
            Cell::Door => {
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

    fn move_forward(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(0,-1, field),
            SideOfTheWorld::South => self.valid_move(0, 1, field),
            SideOfTheWorld::East => self.valid_move(1, 0, field),
            SideOfTheWorld::West => self.valid_move(-1, 0, field),
        }

    }

    fn move_back(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(0,1, field),
            SideOfTheWorld::South => self.valid_move(0, -1, field),
            SideOfTheWorld::East => self.valid_move(-1, 0, field),
            SideOfTheWorld::West => self.valid_move(1, 0, field),
        }
    }

    fn move_left(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(-1,0, field),
            SideOfTheWorld::South => self.valid_move(1, 0, field),
            SideOfTheWorld::East => self.valid_move(0, -1, field),
            SideOfTheWorld::West => self.valid_move(0, 1, field),
        }
    }

    fn move_right(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
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

    /*fn open_door(&mut self, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool{
        if self.x = 
    }*/

}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Cell {
    Wall,
    Pass,
    Door,
    Key,
}

const FIELD_HEIGHT: usize = 9;
const FIELD_WIDTH: usize = 6;

fn draw_field(character: &Character, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) {
    print!("\x1bc");

    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if character.x == x && character.y == y {
                match character.side_of_the_world {
                    SideOfTheWorld::North => print!("^"),
                    SideOfTheWorld::South => print!("v"),
                    SideOfTheWorld::East => print!(">"),
                    SideOfTheWorld::West => print!("<"),
                }
            } else {
                match field[y][x] {
                    Cell::Wall => print!("#"),
                    Cell::Pass => print!(" "),
                    Cell::Door => print!("D"),
                    Cell::Key => print!("K"),
                }
            }
        }
        println!();
    }
    println!();

    if character.has_key {
        println!("У вас есть ключ!");
    } else {
        println!("Ключ не найден.");
    }
}

fn main() {
    let mut field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT] = [
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
    /*
    ________
	|^ ## #|
	|# ## K|
	|# #  #|
	|  ## #|
	|#     |
	|  ## #|
	|D#    |
	| ## # |
	| ## # |
    ////////
     */

    let mut character: Character = Character {
        x: 0,
        y: 0,
        side_of_the_world: SideOfTheWorld::North,
        has_key: false,
    };

    println!("Двигайтесь! (left, right, forward, back, turn left, turn right, turn around)");
    draw_field(&character, &field);
    loop {
        println!(
            "Ваши координаты: {}.{}, направление на {:?}",
            character.x, character.y, character.side_of_the_world
        );

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода!");
        let command = input.trim().to_lowercase();
        let mut made_action = false;

        match command.as_str() {
            "turn left" => {
                character.turn_left();
                made_action = true;
            }
            "turn right" => {
                character.turn_right();
                made_action = true;
            }
            "turn around" => {
                character.turn_around();
                made_action = true;
            }
            "forward" => {
                if character.move_forward(&mut field) {
                    made_action = true;
                } else {
                    println!("Нельзя пройти!");
                }
            },
            "back" => {
                if character.move_back(&mut field) {
                    made_action = true;
                } else {
                    println!("Нельзя пройти!");
                }
            },
            "left" => {
                if character.move_left(&mut field) {
                    made_action = true;
                } else {
                    println!("Нельзя пройти!");
                }
            },
            "right" => {
                if character.move_right(&mut field) {
                    made_action = true;
                } else {
                    println!("Нельзя пройти!");
                }
            },
            _ => println!("Неверная команда!"),
        }
        if made_action {
            draw_field(&character, &field);
        }
    }
}
