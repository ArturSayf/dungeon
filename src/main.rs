mod game;
use game::{
    Command, Character, Cell, SideOfTheWorld, Item, Enemy, FIELD_WIDTH, FIELD_HEIGHT, fpv, see_map, MapVisibility, read_input
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

fn check_exit(character: &Character, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
    matches!(field[character.y][character.x], Cell::Exit { state: _, direction: _ })
}

fn victory_screen(){
    print!("\x1bc");
    println!("╔════════════════════════════════⁠╗");
    println!("║             ПОБЕДА!            ║");
    println!("║    Поздравляем вы выбрались    ║");
    println!("║         из подземелья!         ║");
    println!("╠⁠════════════════════════════════╣");
    println!("║      q - Выход из игры.        ║");
    println!("║     m - Вернуться в меню.      ║");
    println!("╚⁠════════════════════════════════⁠╝");
}

fn game_over_screen() {
    print!("\x1bc");
    println!("╔════════════════════════════════⁠╗");
    println!("║            ПРОИГРЫШ!           ║");
    println!("║       Вы были побеждены!       ║");
    println!("╠⁠════════════════════════════════╣");
    println!("║      q - Выход из игры.        ║");
    println!("║     m - Вернуться в меню.      ║");
    println!("╚⁠════════════════════════════════⁠╝");
}

fn main() {
    let paper_note_1 = "Первая цифра кода: 7".to_string();
    let paper_note_2 = "Вторая цифра кода: 1".to_string();
    let paper_note_3 = "Третья цифра кода: 4".to_string();
    let paper_note_4 = "Последняя цифра кода: 8".to_string();
    let paper_note_5 = "Виктор Носань".to_string();

    // Инициализация поля
    let mut field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT] = [
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Safe { state: false, direction: SideOfTheWorld::East, password: 7148, items: vec![Item::Key(4)] }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Exit { state: false, direction: SideOfTheWorld::North }, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Door { direction: SideOfTheWorld::West, number: 4, state: false }, Cell::Pass, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Paper { text: paper_note_5 }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::South, number: 2, state: false }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Toggle { state: false, number: 2, direction: SideOfTheWorld::South }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Box { items: vec![Item::Paper(paper_note_2)] }, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Toggle { state: false, number: 1, direction: SideOfTheWorld::South }, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Box { items: vec![Item::Paper(paper_note_3), Item::Key(2)] }, Cell::Wall],
        [Cell::Wall, Cell::Box { items: vec![Item::Paper(paper_note_1), Item::Key(3)] }, Cell::LiftingGates { state: false, number: 2, direction: SideOfTheWorld::West }, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::LiftingGates { state: false, number: 1, direction: SideOfTheWorld::West }, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Door { direction: SideOfTheWorld::East, number: 1, state: false }, Cell::Pass, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::North, number: 3, state: false }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Medkit {amount: 20}, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Box { items: vec![Item::Paper(paper_note_4)] }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Key { number: 1 }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
    ];

    // Инициализация персонажа
    let mut character = Character::new(12, 5, SideOfTheWorld::South);

    let mut enemy = vec![
        Enemy::new(8, 9, SideOfTheWorld::East),
    ];

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
        println!("Координаты противника: {}.{}, направление на {}",
            enemy[0].x, enemy[0].y, enemy[0].side_of_the_world
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
                if see_map(&character, &mut field, &map_visibility, &enemy) {
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
                for enemy in enemy.iter_mut() {
                    if enemy.is_alive() && enemy.is_adjacent_to_player(&character) {
                        let dx = enemy.x as isize - character.x as isize;
                        let dy = enemy.y as isize - character.y as isize;
                        
                        let is_facing = match character.side_of_the_world {
                            SideOfTheWorld::North => dx == 0 && dy == -1,
                            SideOfTheWorld::South => dx == 0 && dy == 1,
                            SideOfTheWorld::East => dx == 1 && dy == 0,
                            SideOfTheWorld::West => dx == -1 && dy == 0,
                        };
                        
                        if is_facing {
                            let damage = rand::thread_rng().gen_range(5..=15);
                            println!("Вы атакуете противника! Наносите {} урона!", damage);
                            enemy.take_damage(damage);
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
                for enemy in enemy.iter_mut() {
                    if !enemy.is_alive() && enemy.is_adjacent_to_player(&character) {
                        enemy.loot_corpse(&mut character);
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

        if made_action { //при успешном выполнении действия очищается терминал и заново рисуется картинка

            for enemy in enemy.iter_mut() {
                if enemy.is_alive() {
                    enemy.update(&character, &mut field);
                    
                    // Проверяем атаку врага
                    if enemy.is_adjacent_to_player(&character) {
                        enemy.attack_player(&mut character);
                        
                        if !character.is_alive() {
                            game_over_screen();
                            let input = read_input("Введите команду: ");
                            match input.as_str() {
                                "q" => return,
                                "m" => return,
                                _ => println!("Неверная команда."),
                            }
                        }
                    }
                }
            }

            print!("\x1bc");
            fpv(&character, &field, &enemy);
            map_visibility.update_visibility(character.x, character.y);

            if check_exit(&character, &field) {
                victory_screen();
                let input = read_input("Введите команду: ");
                match input.as_str() {
                    "q" => break,
                    "m" => break,
                    _ => println!("Неверная команда."),
                }
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
        }
        else {
            match command {
                Command::Action => continue,
                _ => println!("Нельзя пройти!"),
            };
        }    
    }
}