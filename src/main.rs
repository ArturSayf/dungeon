use std::io;
mod game;

use game::{
    Command, Character, Cell, SideOfTheWorld, 
    FIELD_WIDTH, FIELD_HEIGHT, fpv, see_map, MapVisibility
};

fn print_available_commands() { // вывод доступных команд
    println!("Доступные команды:");
    println!("  {} - движение вперед", Command::Forward);
    println!("  {} - движение назад", Command::Back);
    println!("  {} - движение влево", Command::Left);
    println!("  {} - движение вправо", Command::Right);
    println!("  {} - поворот налево", Command::TurnLeft);
    println!("  {} - поворот направо", Command::TurnRight);
    println!("  {} - разворот", Command::TurnAround);
    println!("  {} - посмотреть карту", Command::Map);
}

fn main() {
    // Инициализация поля
    let mut field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT] = [
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::North }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall],
        [Cell::Pass, Cell::Pass, Cell::Door { direction: SideOfTheWorld::West }, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Door { direction: SideOfTheWorld::East }, Cell::Pass, Cell::Pass],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Key, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::South }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
    ];

    // Инициализация персонажа
    let mut character = Character {
        x: 4,
        y: 3,
        side_of_the_world: SideOfTheWorld::North,
        has_key: false,
    };

    // Инициализация карты
    let mut map_visibility = MapVisibility::new();
    map_visibility.update_visibility(character.x, character.y);

    // вывод доступных команд
    print_available_commands();
    println!();
    //вывод изображения от 1-го лица
    fpv(&character, &field);
    
    //основной цикл игры
    loop {
        println!(
            "Ваши координаты: {}.{}, направление на {}",
            character.x, character.y, character.side_of_the_world
        );

        //ввод команды
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода!");
        let command_str = input.trim().to_lowercase();
        let mut made_action = false;

        // вывод доступных команд
        if command_str == "help" {
            print_available_commands();
            continue;
        }

        //вывод сообщения о вводе неверной команды
        let command = match Command::from_str(&command_str) {
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
                    map_visibility.update_visibility(character.x, character.y);
                    made_action = true;
                } else {
                    println!("Нельзя пройти!");
                }
            },
            Command::Back => { //шаг назад
                if character.move_back(&mut field) {
                    map_visibility.update_visibility(character.x, character.y);
                    made_action = true;
                } else {
                    println!("Нельзя пройти!");
                }
            },
            Command::Left => { //шаг влево
                if character.move_left(&mut field) {
                    map_visibility.update_visibility(character.x, character.y);
                    made_action = true;
                } else {
                    println!("Нельзя пройти!");
                }
            },
            Command::Right => { //шаг вправо
                if character.move_right(&mut field) {
                    map_visibility.update_visibility(character.x, character.y);
                    made_action = true;
                } else {
                    println!("Нельзя пройти!");
                }
            },
            Command::Map => { //посмотреть карту
                if see_map(&character, &field, &map_visibility) {
                    made_action = true;
                }
            },
        }

        if made_action { //при успешном выполнении действия очищается терминал и заново рисуется картинка
            print!("\x1bc");
            fpv(&character, &field);
            
            if character.has_key {
                println!("У вас есть ключ!"); //сообщении о подобраном ключе
            }
        }
    }
}