use std::io;
#[derive(PartialEq,Debug)]
enum Side_of_the_world{
    North,
    South,
    West,
    East,
}
struct Character {
    x: i32,
    y: i32,
    side_of_the_world: Side_of_the_world
}


fn main() {
    let mut character: Character = Character {
        x: 0,
        y: 0,
        side_of_the_world: Side_of_the_world::North,
    };

    println!(
        "Двигайтесь! (left, right, forward, back, turn left, turn right, turn around)"
    );
    loop {
        println!(
            "Ваши координаты: {}.{}, направление на {:?}",
            character.x, character.y, character.side_of_the_world
        );

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Ошибка!");
        input = input.trim().to_lowercase();
use crate::Side_of_the_world::*;

        match input.as_str() {
            "turn left" => {
                character.side_of_the_world = match character.side_of_the_world {
                    North => West,
                    South => East,
                    East => North,
                    West => South,
                };
            },
            "turn right" => {
                character.side_of_the_world = match character.side_of_the_world {
                    North => East,
                    South => West,
                    East => South,
                    West => North,
                };
            },
            "turn around" => {
                character.side_of_the_world = match character.side_of_the_world {
                    North => South,
                    South => North,
                    East => West,
                    West => East,
                };
            },
            "forward" => {
                match character.side_of_the_world {
                    Side_of_the_world::North => character.y += 1,
                    Side_of_the_world::South => character.y -= 1,
                    Side_of_the_world::East => character.x += 1,
                    Side_of_the_world::West => character.x -= 1,
                }
            },
            "back" => {
                match character.side_of_the_world {
                    Side_of_the_world::North => character.y -= 1,
                    Side_of_the_world::South => character.y += 1,
                    Side_of_the_world::East => character.x -= 1,
                    Side_of_the_world::West => character.x += 1,
                }
            },
            "left" => {
                match character.side_of_the_world {
                    Side_of_the_world::North => character.x -= 1,
                    Side_of_the_world::South => character.x += 1,
                    Side_of_the_world::East => character.y += 1,
                    Side_of_the_world::West => character.y -= 1,
                }
            },
            "right" => {
                match character.side_of_the_world {
                    Side_of_the_world::North => character.x += 1,
                    Side_of_the_world::South => character.x -= 1,
                    Side_of_the_world::East => character.y -= 1,
                    Side_of_the_world::West => character.y += 1,
                }
            },
            _ => println!("Неверная команда!"),
        }
    }
}
