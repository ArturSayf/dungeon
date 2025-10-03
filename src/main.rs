use std::io;

#[derive(PartialEq, Debug)]
enum SideOfTheWorld {
    North,
    South,
    West,
    East,
}
struct Character {
    x: i32,
    y: i32,
    side_of_the_world: SideOfTheWorld,
}

#[derive(PartialEq)]
enum Cell {
    Wall,
    Pass,
}

const FIELD_HEIGHT: usize = 5;
const FIELD_WIDTH: usize = 5;

fn valid_move(field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT], x:i32, y:i32) -> bool{
    if x >= 0 && x < FIELD_WIDTH as i32 && y >= 0 && y < FIELD_HEIGHT as i32{
        if field[y as usize][x as usize] == Cell::Pass{
            return true;
        } else {
            println!("Стена!");
            false
        }
    } else {
        println!("Стена!");
        false
    }
}

fn main() {
    let field: [[Cell; 5]; 5] = [
        [Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass],
        [Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass],
        [Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass],
        [Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass],
        [Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass],
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
        use crate::SideOfTheWorld::*;

        match input.as_str() {
            "turn left" => {
                character.side_of_the_world = match character.side_of_the_world {
                    North => West,
                    South => East,
                    East => North,
                    West => South,
                };
            }
            "turn right" => {
                character.side_of_the_world = match character.side_of_the_world {
                    North => East,
                    South => West,
                    East => South,
                    West => North,
                };
            }
            "turn around" => {
                character.side_of_the_world = match character.side_of_the_world {
                    North => South,
                    South => North,
                    East => West,
                    West => East,
                };
            }
            "forward" => match character.side_of_the_world {
                SideOfTheWorld::North => if valid_move(&field, character.x, character.y - 1) {character.y -= 1},
                SideOfTheWorld::South => if valid_move(&field,character.x, character.y + 1) {character.y += 1},
                SideOfTheWorld::East => if valid_move(&field,character.x + 1, character.y) {character.x += 1},
                SideOfTheWorld::West => if valid_move(&field,character.x - 1, character.y) {character.x -= 1},
            },
            "back" => match character.side_of_the_world {
                SideOfTheWorld::North => if valid_move(&field,character.x, character.y + 1) {character.y += 1},
                SideOfTheWorld::South => if valid_move(&field,character.x, character.y - 1) {character.y -= 1},
                SideOfTheWorld::East => if valid_move(&field,character.x - 1, character.y) {character.x -= 1},
                SideOfTheWorld::West => if valid_move(&field,character.x + 1, character.y) {character.x += 1},
            },
            "left" => match character.side_of_the_world {
                SideOfTheWorld::North => if valid_move(&field,character.x - 1, character.y) {character.x -= 1},
                SideOfTheWorld::South => if valid_move(&field,character.x + 1, character.y) {character.x += 1},
                SideOfTheWorld::East => if valid_move(&field,character.x, character.y - 1) {character.y -= 1},
                SideOfTheWorld::West => if valid_move(&field,character.x, character.y + 1) {character.y += 1},
            },
            "right" => match character.side_of_the_world {
                SideOfTheWorld::North => if valid_move(&field,character.x + 1, character.y) {character.x += 1},
                SideOfTheWorld::South => if valid_move(&field,character.x - 1, character.y) {character.x -= 1},
                SideOfTheWorld::East => if valid_move(&field,character.x, character.y + 1) {character.y += 1},
                SideOfTheWorld::West => if valid_move(&field,character.x, character.y - 1) {character.y -= 1},
            },
            _ => println!("Неверная команда!"),
        }
    }
}
