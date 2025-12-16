mod game;
use game::{
    Command, Character, Cell, SideOfTheWorld, Item, fpv, see_map, MapVisibility, read_input,
    Level
};
use rand::Rng;

fn print_available_commands() {
    println!("Доступные команды:");
    println!("  {} - движение вперед", Command::Forward);
    println!("  {} - движение назад", Command::Back);
    println!("  {} - движение влево", Command::Left);
    println!("  {} - движение вправо", Command::Right);
    println!("  {} - поворот налево", Command::TurnLeft);
    println!("  {} - поворот направо", Command::TurnRight);
    println!("  {} - разворот", Command::TurnAround);
    println!("  {} - посмотреть карту", Command::Map);
    println!("  {} - взаимодействие", Command::Action);
    println!("  {} - инвентарь", Command::Inventory);
    println!("  {} - атаковать", Command::Attack);
    println!("  {} - обыскать труп", Command::Loot);
    println!("  {} - использовать аптечку", Command::UseMedkit);
}

fn check_exit(character: &Character, field: &[[Cell; game::FIELD_WIDTH]; game::FIELD_HEIGHT]) -> bool {
    matches!(field[character.y][character.x], Cell::Exit { state: _, direction: _ })
}

fn victory_screen() -> String {
    print!("\x1bc");
    println!("╔════════════════════════════════⁠╗");
    println!("║             ПОБЕДА!            ║");
    println!("║    Поздравляем! Вы выбрались   ║");
    println!("║         из подземелья!         ║");
    println!("╠⁠════════════════════════════════╣");
    println!("║      q - Выход из игры.        ║");
    println!("║     m - Вернуться в меню.      ║");
    println!("╚⁠════════════════════════════════⁠╝");
    
    read_input("Введите команду: ")
}

fn game_over_screen() -> String {
    print!("\x1bc");
    println!("╔════════════════════════════════⁠╗");
    println!("║           ПОРАЖЕНИЕ!           ║");
    println!("║     Вы не смогли выбраться     ║");
    println!("║         из подземелья!         ║");
    println!("╠⁠════════════════════════════════╣");
    println!("║      q - Выход из игры.        ║");
    println!("║     m - Вернуться в меню.      ║");
    println!("╚⁠════════════════════════════════⁠╝");
    
    read_input("Введите команду: ")
}

fn main_menu() {
    loop {
        print!("\x1bc");
        println!("╔══════════════════════════════════════════════════╗");
        println!("║                    ПОДЗЕМЕЛЬЕ                    ║");
        println!("╠══════════════════════════════════════════════════╣");
        println!("║                   ГЛАВНОЕ МЕНЮ                   ║");
        println!("╠══════════════════════════════════════════════════╣");
        println!("║  1 - Выбрать уровень                             ║");
        println!("║  2 - Выход из игры                               ║");
        println!("╚══════════════════════════════════════════════════╝");
        
        let input = read_input("Выберите действие: ");
        
        match input.as_str() {
            "1" => level_selection_menu(),
            "2" => {
                print!("\x1bc");
                println!("Выход из игры");
                return;
            }
            _ => {
                println!("Неверный выбор! Нажмите Enter чтобы продолжить");
                let _ = std::io::stdin().read_line(&mut String::new());
            }
        }
    }
}

fn level_selection_menu() {
    let levels = Level::get_all_levels();
    
    loop {
        print!("\x1bc");
        println!("╔══════════════════════════════════════════════════╗");
        println!("║                   ВЫБОР УРОВНЯ                   ║");
        println!("╠══════════════════════════════════════════════════╣");
        
        for (i, level) in levels.iter().enumerate() {
            println!("║ {}: {:<30}                ║", 
                    i + 1, level.name);
        }
        
        println!("║                                                  ║");
        println!("║  m - Вернуться в главное меню                    ║");
        println!("╚══════════════════════════════════════════════════╝");
        
        let input = read_input("Выберите уровень: ");
        
        if input == "m" {
            return;
        }
        
        if let Ok(choice) = input.parse::<usize>() {
            if choice >= 1 && choice <= levels.len() {
                let selected_level = &levels[choice - 1];
                start_level(selected_level.clone());
                return;
            }
        }
        
        println!("Неверный выбор! Нажмите Enter чтобы продолжить...");
        let _ = std::io::stdin().read_line(&mut String::new());
    }
}

fn start_level(level: Level) {
    let mut field = level.field.clone();
    let mut enemy = level.enemies.clone();

    let mut character = Character::new(
        level.player_start_x,
        level.player_start_y,
        level.player_start_direction
    );

    // Инициализация карты
    let mut map_visibility = MapVisibility::new();
    map_visibility.update_visibility(character.x, character.y);

    // вывод доступных команд
    print_available_commands();
    println!();
    //вывод изображения от 1-го лица
    fpv(&character, &field, &enemy);
    
    //основной цикл игры
    loop {
        println!(
            "Ваши координаты: {}.{}, направление на {}",
            character.x, character.y, character.side_of_the_world
        );

        //ввод команды
        let input = read_input("Введите команду: ");
        let mut made_action = false;

        // вывод доступных команд
        if input == "help" {
            print_available_commands();
            continue;
        }

        //вывод сообщения о вводе неверной команды
        let command = match Command::from_str(&input) {
            Some(cmd) => cmd,
            None => {
                println!("Неверная команда! Введите 'help' для списка команд.");
                continue;
            }
        };

        // выполнение команды
        match command {
            Command::TurnLeft => { //поворот налево
                character.turn_left();
                made_action = true;
            }
            Command::TurnRight => { //поворот направо
                character.turn_right();
                made_action = true;
            }
            Command::TurnAround => { //разворот на 180°
                character.turn_around();
                made_action = true;
            }
            Command::Forward => { //шаг вперёд
                if character.move_forward(&mut field) {
                    made_action = true;
                } 
            },
            Command::Back => { //шаг назад
                if character.move_back(&mut field) {
                    made_action = true;
                } 
            },
            Command::Left => { //шаг влево
                if character.move_left(&mut field) {
                    made_action = true;
                } 
            },
            Command::Right => { //шаг вправо
                if character.move_right(&mut field) {
                    made_action = true;
                } 
            },
            Command::Map => { //посмотреть карту
                if see_map(&character, &field, &map_visibility, &enemy) {
                    made_action = true;
                }
            },
            Command::Action => {
                if character.action(&mut field){
                    made_action = true;
                }
            },
            Command::Inventory => {
                character.manage_inventory();
                made_action = true;
            },
            Command::Attack => {
                let mut enemy_attacked = false;
                for e in enemy.iter_mut() {
                    if e.is_alive() && e.is_adjacent_to_player(&character) {
                        let dx = e.x as isize - character.x as isize;
                        let dy = e.y as isize - character.y as isize;
                        
                        let is_facing = match character.side_of_the_world {
                            SideOfTheWorld::North => dx == 0 && dy == -1,
                            SideOfTheWorld::South => dx == 0 && dy == 1,
                            SideOfTheWorld::East => dx == 1 && dy == 0,
                            SideOfTheWorld::West => dx == -1 && dy == 0,
                        };
                        
                        if is_facing {
                            let damage = rand::thread_rng().gen_range(5..=15);
                            println!("Вы атакуете противника! Наносите {} урона!", damage);
                            e.take_damage(damage);
                            enemy_attacked = true;
                            made_action = true;
                            break;
                        }
                    }
                }
                
                if !enemy_attacked {
                    println!("Нет врагов для атаки! Вы должны смотреть на врага и быть рядом с ним.");
                }
            },
            Command::Loot => {
                let mut corpse_looted = false;
                for e in enemy.iter_mut() {
                    if !e.is_alive() && e.is_adjacent_to_player(&character) {
                        e.loot_corpse(&mut character);
                        corpse_looted = true;
                        made_action = true;
                        break;
                    }
                }
                
                if !corpse_looted {
                    println!("Нет трупов для обыска рядом!");
                }
            },
            Command::UseMedkit => {
                let has_medkit = character.inventory.iter().any(|item| matches!(item, Item::Medkit(_)));
                if has_medkit {
                    character.manage_inventory();
                    made_action = true;
                } else {
                    println!("В инвентаре нет аптечек!");
                }
            },
        }

        if made_action {
            for e in enemy.iter_mut() {
                if e.is_alive() {
                    e.update(&character, &mut field);
                    
                    if e.is_adjacent_to_player(&character) {
                        e.attack_player(&mut character);
                        
                        if !character.is_alive() {
                            let input = game_over_screen();
                            match input.as_str() {
                                "q" => std::process::exit(0),
                                "m" => return,
                                _ => println!("Неверная команда, возвращаюсь в меню..."),
                            }
                            return;
                        }
                    }
                }
            }

            print!("\x1bc");
            fpv(&character, &field, &enemy);
            map_visibility.update_visibility(character.x, character.y);

            if check_exit(&character, &field) {
                let input = victory_screen();
                match input.as_str() {
                    "q" => std::process::exit(0),
                    "m" => return,
                    _ => println!("Неверная команда, возвращаюсь в меню..."),
                }
                return;
            }

            if !character.inventory.is_empty() {
                print!("В инвентаре: ");
                let mut first = true;
                for item in &character.inventory {
                    if !first {
                        print!(", ");
                    }
                    match item {
                        Item::Key(number) => print!("Ключ №{}", number),
                        Item::Paper(..) => print!("Бумага"),
                        Item::Medkit(amount) => print!("Аптечка (+{} HP)", amount),
                    }
                    first = false;
                }   
            }   
            println!();
            println!("Здоровье: {}/{}", character.health, character.max_health);
            println!();   
        } else {
            match command {
                Command::Action => continue,
                _ => println!("Нельзя пройти!"),
            };
        }    
    }
}

fn main() {
    main_menu();
}