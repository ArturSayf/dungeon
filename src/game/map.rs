use crate::game::character::Character;
use crate::game::field::{Cell, FIELD_WIDTH, FIELD_HEIGHT, MapVisibility, SideOfTheWorld};
use crate::game::view::fpv;
use std::io;

pub fn see_map(
    character: &Character, 
    field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT],
    map_visibility: &MapVisibility
) -> bool {
    print!("\x1bc");
    draw_map(character, field, map_visibility);
    println!("v - убрать карту.");
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода!");
        let command = input.trim().to_lowercase();
        
        match command.as_str() {
            "v" => {
                print!("\x1bc");
                fpv(character, field);
                return true;
            }
            _ => {
                println!("Неизвестная команда! Нажмите 'v'!");
            }
        }
    }
}

pub fn draw_map(
    character: &Character, 
    field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT],
    map_visibility: &MapVisibility
) {
    println!("Карта (обнаруженные области):");
    println!();
    
    print!("  ╔");
    for _ in 0..FIELD_WIDTH {
        print!("═");
    }
    println!("╗"); 
    
    for y in 0..FIELD_HEIGHT {
        print!("  ║");
        
        for x in 0..FIELD_WIDTH {
            // Показывает только обнаруженные клетки
            if map_visibility.is_discovered(x, y) {
                if character.x == x && character.y == y {
                    match character.side_of_the_world {
                        SideOfTheWorld::North => print!("^"),
                        SideOfTheWorld::South => print!("v"),
                        SideOfTheWorld::East => print!(">"),
                        SideOfTheWorld::West => print!("<"),
                    }
                } else {
                    match field[y][x] {
                        Cell::Wall { .. } => print!("#"),
                        Cell::Pass { .. } => print!("."),
                        Cell::Door { direction, .. } => {
                            match direction {
                                SideOfTheWorld::South => print!("\u{203E}"),
                                SideOfTheWorld::North => print!("_"),
                                SideOfTheWorld::East => print!("["),
                                SideOfTheWorld::West => print!("]"),
                            }
                        },
                        Cell::Key { .. } => {
                            if let Cell::Key { .. } = field[y][x] {
                                print!("🗝");
                            } else {
                                print!(".");
                            }
                        },
                        Cell::Paper { .. } => {
                            if let Cell::Paper { .. } = field[y][x] {
                                print!("📄");
                            } else {
                                print!(".");
                            }
                        },
                        Cell::Medkit { .. } => {
                            if let Cell::Medkit { .. } = field[y][x] {
                                print!("🚑");
                            } else {
                                print!(".");
                            }
                        },
                        Cell::Toggle { state, .. } => {
                            match state {
                                true => print!("+"),
                                false => print!("-"),
                            }
                        },
                        Cell::LiftingGates { direction, .. } => {
                            match direction {
                                SideOfTheWorld::South => print!("\u{203E}"),
                                SideOfTheWorld::North => print!("_"),
                                SideOfTheWorld::East => print!("["),
                                SideOfTheWorld::West => print!("]"),
                            }
                        },
                        Cell::Box { .. } => print!("❒"),
                        Cell::Safe { .. } => print!("S"),
                        Cell::Exit { .. } => print!("E"),
                    }
                }
            } else {

                print!(" ");
            }
        }
        
        println!("║"); 
    }
    
    print!("  ╚");
    for _ in 0..FIELD_WIDTH {
        print!("═");
    }
    println!("╝");
    println!();
}