use std::io;
mod game;
use game::{
    Command, Character, Cell, SideOfTheWorld, Item, 
    FIELD_WIDTH, FIELD_HEIGHT, fpv, see_map, MapVisibility,
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
    println!("  {} - взаимодействие", Command::Action);
    println!("  {} - инвентарь", Command::Inventory);
}

fn check_exit(character: &Character, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
    matches!(field[character.y][character.x], Cell::Exit)
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

fn main() {
    let paper_note_1 = "Первая цифра кода: 7".to_string();
    let paper_note_2 = "Вторая цифра кода: 1".to_string();
    let paper_note_3 = "Третья цифра кода: 4".to_string();
    let paper_note_4 = "Последняя цифра кода: 8".to_string();

    // Инициализация поля
    let mut field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT] = [
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Safe { state: false, direction: SideOfTheWorld::East, password: 7148, items: vec![Item::Key(4)] }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Exit, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Door { direction: SideOfTheWorld::West, number: 4, state: false }, Cell::Pass, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::South, number: 2, state: false }, Cell::Wall, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Toggle { state: false, number: 2, direction: SideOfTheWorld::South }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Box { items: vec![Item::Paper(paper_note_2)] }, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Toggle { state: false, number: 1, direction: SideOfTheWorld::South }, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Box { items: vec![Item::Paper(paper_note_3), Item::Key(2)] }],
        [Cell::Box { items: vec![Item::Paper(paper_note_1), Item::Key(3)] }, Cell::LiftingGates { state: false, number: 2, direction: SideOfTheWorld::West }, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::LiftingGates { state: false, number: 1, direction: SideOfTheWorld::West }, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Door { direction: SideOfTheWorld::East, number: 1, state: false }, Cell::Pass, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::North, number: 3, state: false }, Cell::Wall, Cell::Wall, Cell::Wall],
        [Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass],
        [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Box { items: vec![Item::Paper(paper_note_4)] }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Key { number: 1 }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
    ];

    // Инициализация персонажа
    let mut character = Character::new(11, 4, SideOfTheWorld::South);
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
                if see_map(&character, &mut field, &map_visibility) {
                    made_action = true;
                }
            },
            Command::Action => {
                if character.action(&mut field){
                    map_visibility.update_visibility(character.x, character.y);
                    made_action = true;
                }
                else {
                    continue;
                }
            },
            Command::Inventory => {
                character.manage_inventory();
                made_action = true;
            },
        }

        if made_action { //при успешном выполнении действия очищается терминал и заново рисуется картинка
            print!("\x1bc");
            fpv(&character, &field);

            if check_exit(&character, &field) {
                victory_screen();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Ошибка ввода!");
                let command_str = input.trim().to_lowercase();
                match command_str.as_str() {
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
                }
                first = false;
            }   
            }   
            println!();
            println!();
        }

           
    }
}