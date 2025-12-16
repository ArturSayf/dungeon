use crate::game::character::Character;
use crate::game::field::{Cell, SideOfTheWorld};
use crate::game::view::fpv;
use crate::game::{read_input, Enemy, FIELD_WIDTH, FIELD_HEIGHT};
use crate::game::MapVisibility;

pub fn see_map(
    character: &Character, 
    field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT],
    map_visibility: &MapVisibility,
    enemy: &[Enemy],
) -> bool {
    print!("\x1bc");
    draw_map(character, field, map_visibility, enemy);
    println!("v - убрать карту.");
    
    loop {
        let input = read_input("Введите команду: ");
        
        match input.as_str() {
            "v" => {
                print!("\x1bc");
                fpv(character, field, enemy);
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
    map_visibility: &MapVisibility,
    enemies: &[Enemy],
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
            if map_visibility.is_discovered(x, y) {
                // Проверяем врагов
                let mut enemy_at_pos = false;
                let mut enemy_dead = false;
                
                for enemy in enemies {
                    if enemy.x == x && enemy.y == y {
                        enemy_at_pos = true;
                        enemy_dead = !enemy.is_alive();
                        break;
                    }
                }
                
                if enemy_at_pos && enemy_dead {
                    // Труп врага
                    print!("†");
                } else if character.x == x && character.y == y {
                    // Игрок
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
                                print!("🧰");
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