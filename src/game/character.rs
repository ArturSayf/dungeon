use crate::game::field::{Cell, Item, SideOfTheWorld, FIELD_WIDTH, FIELD_HEIGHT};

use std::io;

 #[derive(PartialEq, Debug, Clone)]
pub struct Character {
    pub x: usize,
    pub y: usize,
    pub side_of_the_world: SideOfTheWorld,
    pub inventory: Vec<Item>,
}

impl Character {
    pub fn new(x: usize, y:usize, direction:SideOfTheWorld) -> Self {
        Self { 
            x, 
            y, 
            side_of_the_world: direction, 
            inventory: Vec::new(),
        }
    }

    pub fn add_key(&mut self, key_number: u8) {
        self.add_to_inventory(Item::Key(key_number));
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
            Cell::Toggle { state, number, .. } => {
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
                    println!("Ключ не подходит! нужен ключ №{}", number);
                    false
                }
            },

            Cell::Box { items } => {
                self.manage_box(items);
                true
            },
            Cell::Safe { state, password, items, direction } if !*state => {
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
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Ошибка ввода!");
                    let input = input.trim();
                
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
            _ => false,
        }
    }

    pub fn manage_inventory(&mut self) {
        let max_width = 25;
        let content_width = max_width - 4;
        print!("\x1bc");
        
        println!("⁠┌──────⁠┤ИНВЕНТАРЬ├──────⁠┐");
    
        if self.inventory.is_empty() {
            println!(" │    Инвентарь пуст     │");
            println!("⁠└⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─┘");
        } else {
            println!(" │ Предметы в инвентаре: │");
            for (i, item) in self.inventory.iter().enumerate() {
                let item_text = format!("{} - {}", i + 1, item);
                let item_len = item_text.chars().count();
                
                if item_len <= content_width {
                    let padding = content_width - item_len;
                    println!(" │ {}{} │", item_text, " ".repeat(padding));
                } else {
                    let truncated: String = item_text.chars().take(content_width - 3).collect();
                    println!(" │ {}... │", truncated);
                }
            }
            
            println!("⁠└⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─⁠─┘");
        }
    
        println!("Нажмите любую клавишу чтобы продолжить...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода!");
    }
    

    pub fn add_to_inventory(&mut self, item: Item) {
        if self.inventory.len() < 3 {
            self.inventory.push(item);
            println!("Подобран предмет: {}", item);
        } else {
            println!("В инвентаре нет места.");
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
                    Item::Paper => format!("{} - Бумага", i + 1),
                    Item::Stone => format!("{} - Камень", i + 1),
                }
            } else {
                "".to_string()
            };
            
            let right_item = if i < box_items.len() {
                match &box_items[i] {
                    Item::Key(num) => format!("{} - Ключ №{}", i + 1, num),
                    Item::Paper => format!("{} - Бумага", i + 1),
                    Item::Stone => format!("{} - Камень", i + 1),
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
        println!(" 1-3 - взять предмет из контейнера");
        println!(" a-c - положить предмет в контейнер");
        println!(" x   - закрыть контейнер");
        println!();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода!");
        let command = input.trim().to_lowercase();
        
        let mut message = String::new();
        
        match command.as_str() {
            "x" => {
                message = "Контейнер закрыт.".to_string();
                println!("{}", message);
                break;
            },
            "1" | "2" | "3" => {
                let index = command.parse::<usize>().unwrap() - 1;
                if index < box_items.len() {
                    if self.inventory.len() < 3 {
                        let item = box_items.remove(index);
                        let item_name = match &item {
                            Item::Key(num) => format!("Ключ №{}", num),
                            Item::Paper => "Бумага".to_string(),
                            Item::Stone => "Камень".to_string(),
                        };
                        self.inventory.push(item);
                        message = format!("Предмет '{}' будет перемещен в инвентарь.", item_name);
                    } else {
                        message = "Инвентарь заполнен! Максимум 3 предмета.".to_string();
                    }
                } else {
                    message = "Неверный номер предмета в контейнере!".to_string();
                }
            },
            "a" => {
                if self.inventory.len() >= 1 {
                    if box_items.len() < 3 {
                        let item = self.inventory.remove(0);
                        let item_name = match &item {
                            Item::Key(num) => format!("Ключ №{}", num),
                            Item::Paper => "Бумага".to_string(),
                            Item::Stone => "Камень".to_string(),
                        };
                        box_items.push(item);
                        message = format!("Предмет '{}' будет перемещен в контейнер.", item_name);
                    } else {
                        message = "Контейнер заполнен! Максимум 3 предмета.".to_string();
                    }
                } else {
                    message = "В инвентаре нет предмета под номером 1!".to_string();
                }
            },
            "b" => {
                if self.inventory.len() >= 2 {
                    if box_items.len() < 3 {
                        let item = self.inventory.remove(1);
                        let item_name = match &item {
                            Item::Key(num) => format!("Ключ №{}", num),
                            Item::Paper => "Бумага".to_string(),
                            Item::Stone => "Камень".to_string(),
                        };
                        box_items.push(item);
                        message = format!("Предмет '{}' будет перемещен в контейнер.", item_name);
                    } else {
                        message = "Контейнер заполнен! Максимум 3 предмета.".to_string();
                    }
                } else {
                    message = "В инвентаре нет предмета под номером 2!".to_string();
                }
            },
            "c" => {
                if self.inventory.len() >= 3 {
                    if box_items.len() < 3 {
                        let item = self.inventory.remove(2);
                        let item_name = match &item {
                            Item::Key(num) => format!("Ключ №{}", num),
                            Item::Paper => "Бумага".to_string(),
                            Item::Stone => "Камень".to_string(),
                        };
                        box_items.push(item);
                        message = format!("Предмет '{}' будет перемещен в контейнер.", item_name);
                    } else {
                        message = "Контейнер заполнен! Максимум 3 предмета.".to_string();
                    }
                } else {
                    message = "В инвентаре нет предмета под номером 3!".to_string();
                }
            },
            _ => {
                message = "Неверная команда! Используйте 1-3, a/b/c или x.".to_string();
            }
        }
        
        if !message.is_empty() {
            println!("{}", message);
            println!("Нажмите Enter чтобы продолжить...");
            let _ = io::stdin().read_line(&mut String::new());
        }
        }
    }


    pub fn from_inventary_to_box(&mut self, index: usize, box_items: &mut Vec<Item>) {
        if index < self.inventory.len() {
            if box_items.len() < 3 {
                let item = self.inventory.remove(index);
                box_items.push(item);
            } else {
                println!("Инвентарь заполнен!");
            }
        } else {
            println!("Неверный номер предмета в ящике!");
        }
    }

    pub fn valid_move(&mut self, dx: isize, dy: isize, field: &mut[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool{

    let nx = match self.x.checked_add_signed(dx) {
      Some(v) => v,
      None => return false,  
    };
    let ny = match self.y.checked_add_signed(dy) {
      Some(v) => v,
      None => return false,  
    };

    if nx < FIELD_WIDTH && ny < FIELD_HEIGHT{
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
                self.add_key(*number);
                field[ny][nx] = Cell::Pass;
                true
            }
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
            Cell::Exit => {
                self.x = nx;
                self.y = ny;
                true
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
        
        if !action_performed {
            println!("Здесь не с чем взаимодействовать.");
        }
        
        action_performed
    }
}