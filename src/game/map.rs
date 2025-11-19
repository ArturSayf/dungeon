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
    
    // Верхняя граница рамки с углами
    print!("  ╔"); // Левый верхний угол
    for _ in 0..FIELD_WIDTH {
        print!("═");
    }
    println!("╗"); // Правый верхний угол
    
    // Содержимое карты с боковыми границами
    for y in 0..FIELD_HEIGHT {
        print!("  ║"); // Левая граница
        
        for x in 0..FIELD_WIDTH {
            // Показываем только обнаруженные клетки
            if map_visibility.is_discovered(x, y) {
                if character.x == x && character.y == y {
                    // Рисуем персонажа
                    match character.side_of_the_world {
                        SideOfTheWorld::North => print!("^"),
                        SideOfTheWorld::South => print!("v"),
                        SideOfTheWorld::East => print!(">"),
                        SideOfTheWorld::West => print!("<"),
                    }
                } else {
                    // Рисуем содержимое клетки
                    match field[y][x] {
                        Cell::Wall => print!("#"),
                        Cell::Pass => print!("."),
                        Cell::Door { direction } => {
                            match direction {
                                SideOfTheWorld::South => print!("\u{203E}"), // ‾
                                SideOfTheWorld::North => print!("_"),
                                SideOfTheWorld::East => print!("["),
                                SideOfTheWorld::West => print!("]"),
                            }
                        },
                        Cell::Key => {
                            // Показываем ключ только если он еще не собран
                            if let Cell::Key = field[y][x] {
                                print!("K");
                            } else {
                                print!(".");
                            }
                        },
                    }
                }
            } else {
                // Необнаруженные клетки
                print!(" ");
            }
        }
        
        println!("║"); // Правая граница
    }
    
    // Нижняя граница рамки с углами
    print!("  ╚"); // Левый нижний угол
    for _ in 0..FIELD_WIDTH {
        print!("═");
    }
    println!("╝"); // Правый нижний угол
    println!();

    // Информация о ключе (остается без изменений)
    if character.has_key {
        println!("У вас есть ключ!");
    } else {
        let mut key_found = false;
        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                if map_visibility.is_discovered(x, y) {
                    if let Cell::Key = field[y][x] {
                        key_found = true;
                        break;
                    }
                }
            }
            if key_found { break; }
        }
        
        if key_found {
            println!("Вы видели ключ на карте!");
        } else {
            println!("Ключ не найден.");
        }
    }
}