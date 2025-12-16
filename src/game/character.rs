use crate::game::field::{Cell, Item, SideOfTheWorld};
use crate::game::{read_input, FIELD_WIDTH, FIELD_HEIGHT};

use std::io;

#[derive(PartialEq, Debug, Clone)]
pub struct Character {
    pub x: usize,
    pub y: usize,
    pub side_of_the_world: SideOfTheWorld,
    pub inventory: Vec<Item>,
    pub health: u8,
    pub max_health: u8,
}

impl Character {
    pub fn new(x: usize, y:usize, direction:SideOfTheWorld) -> Self {
        Self { 
            x, 
            y, 
            side_of_the_world: direction, 
            inventory: Vec::new(),
            health: 80,
            max_health: 100,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn take_damage(&mut self, amount: u8) {
        if amount >= self.health {
            self.health = 0;
            println!("Персонаж погиб!");
        } else {
            self.health -= amount;
            println!("Получено {} урона! Осталось здоровья: {}/{}", 
                     amount, self.health, self.max_health);
        }
    }

    pub fn has_key(&self, door_number: u8) -> bool {
        self.inventory.iter().any(|item| {
            if let Item::Key(number) = item {
                *number == door_number
            } else {
                false
            }
        })
    }

    pub fn remove_key(&mut self, door_number: u8) -> bool {
        if let Some(pos) = self.inventory.iter().position(|item| {
            if let Item::Key(number) = item {
                *number == door_number
            } else {
                false
            }
        }) {
            self.inventory.remove(pos);
            true
        } else {
            false
        }
    }   
    
    pub fn interaction(&mut self, dx: isize, dy: isize, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        let nx = match self.x.checked_add_signed(dx) {
            Some(v) => v,
            None => return false,  
        };
        let ny = match self.y.checked_add_signed(dy) {
            Some(v) => v,
            None => return false,  
        };

        if nx >= FIELD_WIDTH || ny >= FIELD_HEIGHT {
            return false;
        }

        match &mut field[ny][nx] {
            Cell::Toggle { state, number, direction,.. } => {
                let correct_side = match (self.side_of_the_world, direction) {
                    (SideOfTheWorld::North, SideOfTheWorld::South) => true,
                    (SideOfTheWorld::South, SideOfTheWorld::North) => true,
                    (SideOfTheWorld::East, SideOfTheWorld::West) => true,
                    (SideOfTheWorld::West, SideOfTheWorld::East) => true,
                    _ => false,
                };

                if !correct_side {
                    return false;
                }
                *state = !*state; 
                let toggle_state = *state;
                let toggle_number = *number;                
                for y in 0..FIELD_HEIGHT {
                    for x in 0..FIELD_WIDTH {
                        if let Cell::LiftingGates { state: gate_state, number: gate_number, .. } = &mut field[y][x] {
                            if *gate_number == toggle_number {
                                *gate_state = toggle_state;
                            }
                        }
                    }
                }
                true
            },

            Cell::Door {number, state, ..} if !*state => {
                if self.has_key(*number) {
                    *state = true;
                    self.remove_key(*number);
                    true
                } else {
                    println!("Нужен ключ №{}", number);
                    false
                }
            },

            Cell::Box { items } => {
                self.manage_box(items);
                true
            },
            Cell::Safe { state, password, direction, .. } if !*state => {
                let correct_side = match (self.side_of_the_world, direction) {
                    (SideOfTheWorld::North, SideOfTheWorld::North) => true,
                    (SideOfTheWorld::South, SideOfTheWorld::South) => true,
                    (SideOfTheWorld::East, SideOfTheWorld::East) => true,
                    (SideOfTheWorld::West, SideOfTheWorld::West) => true,
                    _ => false,
                };

                if !correct_side {
                    return false;
                }

                loop {
                    println!("Введите пароль (4 цифры) или 'x' для отмены:");
                    let input = read_input("Введите команду: ");
                
                    if input == "x" {
                        return false;
                    }
                
                    if input.len() == 4 && input.chars().all(|c| c.is_digit(10)) {
                        let entered_password: u16 = match input.parse() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Некорректный пароль!");
                                continue;
                            }
                        };
                    
                        if entered_password == *password {
                            *state = true;
                            return true;
                        } else {
                            println!("Неверный пароль! Попробуйте еще раз.");
                            continue;
                        }
                    } else {
                        println!("Пароль должен содержать ровно 4 цифры!");
                    }
                }
            },

            Cell::Safe { state, items, direction, .. } if *state => {
                let correct_side = match (self.side_of_the_world, direction) {
                    (SideOfTheWorld::North, SideOfTheWorld::North) => true,
                    (SideOfTheWorld::South, SideOfTheWorld::South) => true,
                    (SideOfTheWorld::East, SideOfTheWorld::East) => true,
                    (SideOfTheWorld::West, SideOfTheWorld::West) => true,
                    _ => false,
                };

                if !correct_side {
                    return false;
                }

                self.manage_box(items);
                true
            },

            Cell::Exit { state, direction } => {
                let correct_side = match (self.side_of_the_world, direction) {
                    (SideOfTheWorld::North, SideOfTheWorld::North) => true,
                    (SideOfTheWorld::South, SideOfTheWorld::South) => true,
                    (SideOfTheWorld::East, SideOfTheWorld::East) => true,
                    (SideOfTheWorld::West, SideOfTheWorld::West) => true,
                    _ => false,
                };

                if !correct_side {
                    return false;
                }
                *state = !*state;
                true
            } ,
            _ => false,
        }
    }

    pub fn manage_inventory(&mut self) {
        loop {
            print!("\x1bc");
            println!("⁠┌──────⁠┤ИНВЕНТАРЬ├──────⁠┐");

            if self.inventory.is_empty() {
                println!(" │    Инвентарь пуст     │");
                println!("⁠└⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─┘");
            } else {
                println!(" │ Предметы в инвентаре: │");
                for (i, item) in self.inventory.iter().enumerate() {
                    match item {
                        Item::Key(number) => println!(" │ {} - Ключ №{:<11} │", i + 1, number),
                        Item::Paper(_) => println!(" │ {} - Бумага{:11} │", i + 1, ""),
                        Item::Medkit(amount) => println!(" │ {} - Аптечка (+{:<2} HP) {:0} │", i +1, amount, ""),
                    }
                }
                println!("⁠└⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─┘");
            }

            println!(" Здоровье: {:<3}/{}", self.health, self.max_health);
            println!(" КОМАНДЫ:");
            println!(" 1-5 - использовать аптечку (если есть)");
            println!(" r - прочитать бумагу (если есть)");
            println!(" x - закрыть инвентарь");
            println!();

            let input = read_input("Введите команду: ");

            match input.as_str() {
                "x" => break,
                "r" => {
                    self.read_papers();
                },
                "1" | "2" | "3" | "4" | "5" => {
                    let index = input.parse::<usize>().unwrap() - 1;
                    if index < self.inventory.len() {
                        if let Item::Medkit(amount) = self.inventory[index].clone() {
                            self.use_medkit(index, amount);
                        } else {
                            println!("Это не аптечка!");
                            println!("Нажмите Enter чтобы продолжить...");
                            let _ = io::stdin().read_line(&mut String::new());
                        }
                    } else {
                        println!("Неверный номер предмета!");
                        println!("Нажмите Enter чтобы продолжить...");
                        let _ = io::stdin().read_line(&mut String::new());
                    }
                },
                _ => {
                    if !input.is_empty() {
                        println!("Неизвестная команда! Используйте '1-5', 'r' или 'x'");
                        println!("Нажмите Enter чтобы продолжить...");
                        let _ = io::stdin().read_line(&mut String::new());
                    }
                }
            }
        }
    }

    pub fn use_medkit(&mut self, index: usize, amount: u8) {
        if self.health >= self.max_health {
            println!("Здоровье уже максимальное!");
            println!("Нажмите Enter чтобы продолжить...");
            let _ = io::stdin().read_line(&mut String::new());
            return;
        }

        self.inventory.remove(index);
        let new_health = self.health as u16 + amount as u16;
        self.health = new_health.min(self.max_health as u16) as u8;
        
        println!("Использована аптечка! +{} HP. Текущее здоровье: {}/{}", 
                 amount, self.health, self.max_health);
        println!("Нажмите Enter чтобы продолжить...");
        let _ = io::stdin().read_line(&mut String::new());
    }

    pub fn read_papers(&self) {
        let papers: Vec<&String> = self.inventory.iter()
            .filter_map(|item| {
                if let Item::Paper(text) = item {
                    Some(text)
                } else {
                    None
                }
            })
            .collect();
        
        if papers.is_empty() {
            return;
        }

        loop {
            print!("\x1bc");
            println!("┌─────────────────────────────────────────┐");
            println!("│            СОДЕРЖАНИЕ БУМАГ             │");
            println!("├─────────────────────────────────────────┤");

            const MAX_LINE_WIDTH: usize = 68;

            for (i, text) in papers.iter().enumerate() {
                println!("│ Записка {}:                              │", i + 1);

                let lines = wrap_text_without_word_break(text, MAX_LINE_WIDTH);

                for line in &lines {
                    println!("│ {:<39} │", line);
                }

                if lines.is_empty() {
                    println!("│ (пусто)                                 │");
                }

                if i < papers.len() - 1 {
                    println!("│                                         │");
                }
            }

            println!("├─────────────────────────────────────────┤");
            println!("│ r - вернуться в инвентарь               │");
            println!("└─────────────────────────────────────────┘");
            println!();

            let input = read_input("Введите команду: ");

            if input == "r" {
                break;
            }
        }
            fn wrap_text_without_word_break(text: &str, max_width: usize) -> Vec<String> {
            let mut result = Vec::new();
            
            let words: Vec<&str> = text.split_whitespace().collect();
            
            let mut current_line = String::new();
            
            for word in words {
                let potential_length = if current_line.is_empty() {
                    word.len()
                } else {
                    current_line.len() + 1 + word.len()
                };

                if potential_length <= max_width {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                } else {
                    if !current_line.is_empty() {
                        result.push(current_line.clone());
                        current_line.clear();
                    }
                    if word.len() > max_width {
                        let mut chars = word.chars();
                        let mut long_word_line = String::new();

                        while let Some(ch) = chars.next() {
                            if long_word_line.len() < max_width {
                                long_word_line.push(ch);
                            } else {
                                result.push(long_word_line.clone());
                                long_word_line.clear();
                                long_word_line.push(ch);
                            }
                        }

                        if !long_word_line.is_empty() {
                            current_line = long_word_line;
                        }
                    } else {
                        current_line.push_str(word);
                    }
                }
            }

            if !current_line.is_empty() {
                result.push(current_line);
            }

            result
        }   
    }

    pub fn add_to_inventory(&mut self, item: Item) -> bool {
        if self.inventory.len() < 5 {
            self.inventory.push(item.clone());
            println!("Подобран предмет: {}", item);
            true
        } else {
            println!("В инвентаре нет места.");
            false
        }
    }

    pub fn manage_box(&mut self, box_items: &mut Vec<Item>) {
        loop {  
        print!("\x1bc");
        
        let col_width = 20;
        let total_width = col_width * 2 + 3; 

        println!("┌{:─<width$}┐", "", width = total_width - 2);
        
        let title = " КОНТЕЙНЕР ";
        let title_padding = (total_width - title.chars().count() - 2) / 2;
        println!("│{: <left$}{}{: <right$}│", 
                 "", 
                 title, 
                 "",
                 left = title_padding,
                 right = total_width - title.chars().count() - 2 - title_padding);
        
        println!("├{:─<left$}┼{:─<right$}┤", 
                 "", "", 
                 left = col_width,
                 right = col_width);
        
        println!("│{: ^left$}│{: ^right$}│", 
                 "ИНВЕНТАРЬ", 
                 "КОНТЕЙНЕР",
                 left = col_width,
                 right = col_width);
        
        println!("├{:─<left$}┼{:─<right$}┤", 
                 "", "", 
                 left = col_width,
                 right = col_width);
        
        let max_rows = 3.max(self.inventory.len()).max(box_items.len());
        
        for i in 0..max_rows {
            let left_item = if i < self.inventory.len() {
                match &self.inventory[i] {
                    Item::Key(num) => format!("{} - Ключ №{}", i + 1, num),
                    Item::Paper(..) => format!("{} - Бумага", i + 1),
                    Item::Medkit(amount) => format!("{} - Аптечка (+{} HP)", i + 1, amount),
                }
            } else {
                "".to_string()
            };
            
            let right_item = if i < box_items.len() {
                match &box_items[i] {
                    Item::Key(num) => format!("{} - Ключ №{}", i + 1, num),
                    Item::Paper(..) => format!("{} - Бумага", i + 1),
                    Item::Medkit(amount) => format!("{} - Аптечка (+{} HP)", i + 1, amount),
                }
            } else {
                "".to_string()
            };
            
            println!("│{:<left$}│{:<right$}│", 
                     left_item, 
                     right_item,
                     left = col_width,
                     right = col_width);
        }
        
        println!("└{:─<left$}┴{:─<right$}┘", 
                 "", "", 
                 left = col_width,
                 right = col_width);
        
        println!();
        println!(" КОМАНДЫ:");
        println!(" 1-5 - взять предмет из контейнера");
        println!(" a-e - положить предмет в контейнер");
        println!(" x   - закрыть контейнер");
        println!();
        
        let input = read_input("Введите команду: ");
        
        let message;
        
        match input.as_str() {
            "x" => {
                message = "Контейнер закрыт.".to_string();
                println!("{}", message);
                break;
            },
            "1" | "2" | "3" | "4" | "5" => {
                let index = input.parse::<usize>().unwrap() - 1;
                if index < box_items.len() {
                    if self.inventory.len() < 5 {
                        let item = box_items.remove(index);
                        let item_name = match &item {
                            Item::Key(num) => format!("Ключ №{}", num),
                            Item::Paper(..) => "Бумага".to_string(),
                            Item::Medkit(amount) => format!("Аптечка (+{} HP)", amount),
                        };
                        self.inventory.push(item);
                        message = format!("Предмет '{}' будет перемещен в инвентарь.", item_name);
                    } else {
                        message = "Инвентарь заполнен! Максимум 5 предметов.".to_string();
                    }
                } else {
                    message = "Неверный номер предмета в контейнере!".to_string();
                }
            },
            "a" => {
                if self.inventory.len() >= 1 {
                    if box_items.len() < 5 {
                        let item = self.inventory.remove(0);
                        let item_name = match &item {
                            Item::Key(num) => format!("Ключ №{}", num),
                            Item::Paper(..) => "Бумага".to_string(),
                            Item::Medkit(amount) => format!("Аптечка (+{} HP)", amount),
                        };
                        box_items.push(item);
                        message = format!("Предмет '{}' будет перемещен в контейнер.", item_name);
                    } else {
                        message = "Контейнер заполнен! Максимум 5 предметов.".to_string();
                    }
                } else {
                    message = "В инвентаре нет предмета под номером 1!".to_string();
                }
            },
            "b" => {
                if self.inventory.len() >= 2 {
                    if box_items.len() < 5 {
                        let item = self.inventory.remove(1);
                        let item_name = match &item {
                            Item::Key(num) => format!("Ключ №{}", num),
                            Item::Paper(..) => "Бумага".to_string(),
                            Item::Medkit(amount) => format!("Аптечка (+{} HP)", amount),
                        };
                        box_items.push(item);
                        message = format!("Предмет '{}' будет перемещен в контейнер.", item_name);
                    } else {
                        message = "Контейнер заполнен! Максимум 5 предметов.".to_string();
                    }
                } else {
                    message = "В инвентаре нет предмета под номером 2!".to_string();
                }
            },
            "c" => {
                if self.inventory.len() >= 3 {
                    if box_items.len() < 5 {
                        let item = self.inventory.remove(2);
                        let item_name = match &item {
                            Item::Key(num) => format!("Ключ №{}", num),
                            Item::Paper(..) => "Бумага".to_string(),
                            Item::Medkit(amount) => format!("Аптечка (+{} HP)", amount),
                        };
                        box_items.push(item);
                        message = format!("Предмет '{}' будет перемещен в контейнер.", item_name);
                    } else {
                        message = "Контейнер заполнен! Максимум 5 предметов.".to_string();
                    }
                } else {
                    message = "В инвентаре нет предмета под номером 3!".to_string();
                }
            },
            "d" => {
                if self.inventory.len() >= 4 {
                    if box_items.len() < 5 {
                        let item = self.inventory.remove(3);
                        let item_name = match &item {
                            Item::Key(num) => format!("Ключ №{}", num),
                            Item::Paper(..) => "Бумага".to_string(),
                            Item::Medkit(amount) => format!("Аптечка (+{} HP)", amount),
                        };
                        box_items.push(item);
                        message = format!("Предмет '{}' будет перемещен в контейнер.", item_name);
                    } else {
                        message = "Контейнер заполнен! Максимум 5 предметов.".to_string();
                    }
                } else {
                    message = "В инвентаре нет предмета под номером 4!".to_string();
                }
            },
            "e" => {
                if self.inventory.len() >= 5 {
                    if box_items.len() < 5 {
                        let item = self.inventory.remove(4);
                        let item_name = match &item {
                            Item::Key(num) => format!("Ключ №{}", num),
                            Item::Paper(..) => "Бумага".to_string(),
                            Item::Medkit(amount) => format!("Аптечка (+{} HP)", amount),
                        };
                        box_items.push(item);
                        message = format!("Предмет '{}' будет перемещен в контейнер.", item_name);
                    } else {
                        message = "Контейнер заполнен! Максимум 5 предметов.".to_string();
                    }
                } else {
                    message = "В инвентаре нет предмета под номером 5!".to_string();
                }
            },
            _ => {
                message = "Неверная команда! Используйте 1-5, a/b/c/d/e или x.".to_string();
            }
        }
        
        if !message.is_empty() {
            println!("{}", message);
        }
        }
    }

    pub fn valid_move(&mut self, dx: isize, dy: isize, field: &mut[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        let nx = match self.x.checked_add_signed(dx) {
          Some(v) => v,
          None => return false,  
        };
        let ny = match self.y.checked_add_signed(dy) {
          Some(v) => v,
          None => return false,  
        };

        if nx < FIELD_WIDTH && ny < FIELD_HEIGHT {
            match &field[ny][nx] {
                Cell::Wall => false,
                Cell::Pass => {
                    self.x = nx;
                    self.y = ny;
                    true
                },
                Cell::Door { state, number, .. } => {
                    if *state {
                        self.x = nx;
                        self.y = ny;
                        true
                    } else {
                        println!("Дверь закрыта! Нужен ключ № {}.", number);
                        false
                    }
                },
                Cell::Key {number} => {
                    self.x = nx;
                    self.y = ny;
                    let picked_up = self.add_to_inventory(Item::Key(*number));
                    if picked_up {
                        field[ny][nx] = Cell::Pass;
                    }
                    true
                },
                Cell::Paper { text } => {
                    self.x = nx;
                    self.y = ny;
                    let picked_up = self.add_to_inventory(Item::Paper(text.clone()));
                    if picked_up {
                        field[ny][nx] = Cell::Pass;
                    }
                    true
                },
                Cell::Medkit { amount } => {
                    self.x = nx;
                    self.y = ny;
                    let picked_up = self.add_to_inventory(Item::Medkit(*amount));
                    if picked_up {
                        field[ny][nx] = Cell::Pass;
                    }
                    true
                },
                Cell::Toggle { .. } => false,
                Cell::LiftingGates { state, number, .. } => {
                    if *state {
                        self.x = nx;
                        self.y = ny;
                        true
                    } else {
                        println!("Дверь закрыта! Найдите переключатель №{}", number);
                        false
                    }
                },
                Cell::Box { .. } => false,
                Cell::Safe { .. } => false,
                Cell::Exit { state, .. } => {
                    if *state {
                        self.x = nx;
                        self.y = ny;
                        true
                    } else {
                        println!("Лифт закрыт! Нажмите кнопку вызова лифта.");
                        false
                    }
                },
            }
        } else {
            false
        }
    }

    pub fn move_forward(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(0,-1, field),
            SideOfTheWorld::South => self.valid_move(0, 1, field),
            SideOfTheWorld::East => self.valid_move(1, 0, field),
            SideOfTheWorld::West => self.valid_move(-1, 0, field),
        }
    }

    pub fn move_back(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(0,1, field),
            SideOfTheWorld::South => self.valid_move(0, -1, field),
            SideOfTheWorld::East => self.valid_move(-1, 0, field),
            SideOfTheWorld::West => self.valid_move(1, 0, field),
        }
    }

    pub fn move_left(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(-1,0, field),
            SideOfTheWorld::South => self.valid_move(1, 0, field),
            SideOfTheWorld::East => self.valid_move(0, -1, field),
            SideOfTheWorld::West => self.valid_move(0, 1, field),
        }
    }

    pub fn move_right(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        match self.side_of_the_world {
            SideOfTheWorld::North => self.valid_move(1,0, field),
            SideOfTheWorld::South => self.valid_move(-1, 0, field),
            SideOfTheWorld::East => self.valid_move(0, 1, field),
            SideOfTheWorld::West => self.valid_move(0, -1, field),
        }
    }

    pub fn turn_left(&mut self) {
        self.side_of_the_world = match self.side_of_the_world {
            SideOfTheWorld::North => SideOfTheWorld::West,
            SideOfTheWorld::South => SideOfTheWorld::East,
            SideOfTheWorld::East => SideOfTheWorld::North,
            SideOfTheWorld::West => SideOfTheWorld::South,
        }
    }

    pub fn turn_right(&mut self) {
        self.side_of_the_world = match self.side_of_the_world {
            SideOfTheWorld::North => SideOfTheWorld::East,
            SideOfTheWorld::South => SideOfTheWorld::West,
            SideOfTheWorld::East => SideOfTheWorld::South,
            SideOfTheWorld::West => SideOfTheWorld::North,
        }
    }

    pub fn turn_around(&mut self) {
        self.side_of_the_world = match self.side_of_the_world {
            SideOfTheWorld::North => SideOfTheWorld::South,
            SideOfTheWorld::South => SideOfTheWorld::North,
            SideOfTheWorld::East => SideOfTheWorld::West,
            SideOfTheWorld::West => SideOfTheWorld::East,
        }
    }

    pub fn action(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        let (dx, dy) = match self.side_of_the_world {
            SideOfTheWorld::North => (0, -1),
            SideOfTheWorld::South => (0, 1),
            SideOfTheWorld::East => (1, 0),
            SideOfTheWorld::West => (-1, 0),
        };
        let action_performed = self.interaction(dx, dy, field);
        action_performed
    }
}