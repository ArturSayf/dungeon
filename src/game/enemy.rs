use std::collections::VecDeque;
use rand::Rng;
use crate::game::field::{Cell, Item, SideOfTheWorld};
use crate::game::character::Character;
use crate::game::{FIELD_WIDTH, FIELD_HEIGHT};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum EnemyState {
    Patrolling,
    Chasing,
    Dead,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Enemy {
    pub x: usize,
    pub y: usize,
    pub side_of_the_world: SideOfTheWorld,
    pub state: EnemyState,
    pub inventory: Vec<Item>,
    pub health: u8,
    pub max_health: u8,
    pub last_known_player_pos: Option<(usize, usize)>,
    pub path_to_player: VecDeque<(usize, usize)>,
    pub consecutive_failed_moves: u8,
    pub hit_wall: bool,
}

impl Enemy {
    pub fn new(x: usize, y: usize, direction: SideOfTheWorld) -> Self {
        Self {
            x,
            y,
            side_of_the_world: direction,
            state: EnemyState::Patrolling,
            inventory: Vec::new(),
            health: 50,
            max_health: 50,
            last_known_player_pos: None,
            path_to_player: VecDeque::new(),
            consecutive_failed_moves: 0,
            hit_wall: false,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.state != EnemyState::Dead && self.health > 0
    }

    pub fn take_damage(&mut self, amount: u8) {
        if amount >= self.health {
            self.health = 0;
            self.state = EnemyState::Dead;
        } else {
            self.health -= amount;
        }
    }

    pub fn add_to_inventory(&mut self, item: Item) -> bool {
        if self.inventory.len() < 5 {
            self.inventory.push(item.clone());
            true
        } else {
            false
        }
    }

    pub fn can_see_player(&self, player: &Character, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        
        let fov = [
            (2, 3), (-2, 3), (1, 3), (-1, 3), (0, 3),
            (1, 2), (-1, 2), (0, 2),
            (1, 1), (-1, 1), (0, 1),
            (1, 0), (-1, 0),
        ];

        let turn_fov = match self.side_of_the_world {
            SideOfTheWorld::South => fov,
            SideOfTheWorld::North => fov.map(|(dx, dy)| (-dx, -dy)),
            SideOfTheWorld::West => fov.map(|(dx, dy)| (-dy, dx)),
            SideOfTheWorld::East => fov.map(|(dx, dy)| (dy, -dx)),
        };

        for (dx, dy) in turn_fov.iter() {
            let mut nx = self.x as isize;
            let mut ny = self.y as isize;
            
            
            for _ in 0..3 { 
                nx += dx;
                ny += dy;
                
                if nx < 0 || nx >= FIELD_WIDTH as isize || ny < 0 || ny >= FIELD_HEIGHT as isize {
                    break;
                }
                
                
                if let Cell::Wall = field[ny as usize][nx as usize] {
                    break;
                }
                
                
                if nx == player.x as isize && ny == player.y as isize {
                    return true;
                }
            }
        }
        
        false
    }

    pub fn is_adjacent_to_player(&self, player: &Character) -> bool {
        (self.x as isize - player.x as isize).abs() <= 1 && 
        (self.y as isize - player.y as isize).abs() <= 1
    }

    pub fn is_facing_player(&self, player: &Character) -> bool {
        let dx = player.x as isize - self.x as isize;
        let dy = player.y as isize - self.y as isize;
        
        match self.side_of_the_world {
            SideOfTheWorld::North => dx == 0 && dy == -1,
            SideOfTheWorld::South => dx == 0 && dy == 1,
            SideOfTheWorld::East => dx == 1 && dy == 0,
            SideOfTheWorld::West => dx == -1 && dy == 0,
        }
    }

    pub fn attack_player(&self, player: &mut Character) -> bool {
        if !self.is_adjacent_to_player(player) {
            return false;
        }
        
        
        if self.is_facing_player(player) {
            let damage = rand::thread_rng().gen_range(5..=15);
            player.take_damage(damage);
            true
        } else {
            false
        }
    }

    pub fn get_direction_to_player(&self, player: &Character) -> Option<SideOfTheWorld> {
        let dx = player.x as isize - self.x as isize;
        let dy = player.y as isize - self.y as isize;
        
        
        if dx == 0 && dy < 0 {
            Some(SideOfTheWorld::North)
        } else if dx == 0 && dy > 0 {
            Some(SideOfTheWorld::South)
        } else if dx > 0 && dy == 0 {
            Some(SideOfTheWorld::East)
        } else if dx < 0 && dy == 0 {
            Some(SideOfTheWorld::West)
        } else {
            
            if dx.abs() > dy.abs() {
                if dx > 0 {
                    Some(SideOfTheWorld::East)
                } else {
                    Some(SideOfTheWorld::West)
                }
            } else {
                if dy > 0 {
                    Some(SideOfTheWorld::South)
                } else {
                    Some(SideOfTheWorld::North)
                }
            }
        }
    }

    pub fn get_turn_direction(&self, target_direction: SideOfTheWorld) -> Option<&'static str> {
        let current = self.side_of_the_world as isize;
        let target = target_direction as isize;
        
        let diff = (target - current).rem_euclid(4);
        
        match diff {
            1 => Some("right"), 
            3 => Some("left"),  
            2 => Some("around"), 
            _ => None, 
        }
    }

    pub fn can_move_forward(&self, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        let (dx, dy) = match self.side_of_the_world {
            SideOfTheWorld::North => (0, -1),
            SideOfTheWorld::South => (0, 1),
            SideOfTheWorld::East => (1, 0),
            SideOfTheWorld::West => (-1, 0),
        };
        
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
        
        match &field[ny][nx] {
            Cell::Wall => false,
            Cell::Door { state, .. } => *state,
            Cell::LiftingGates { state, .. } => *state,
            Cell::Toggle { .. } | Cell::Box { .. } | Cell::Safe { .. } | Cell::Exit { .. } => false,
            _ => true,
        }
    }

    pub fn find_path_to_player(&mut self, player: &Character, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) {
        self.path_to_player.clear();
        
        
        let mut visited = [[false; FIELD_WIDTH]; FIELD_HEIGHT];
        let mut queue = VecDeque::new();
        let mut parent = [[None; FIELD_WIDTH]; FIELD_HEIGHT];
        
        queue.push_back((self.x, self.y));
        visited[self.y][self.x] = true;
        
        while let Some((x, y)) = queue.pop_front() {
            if x == player.x && y == player.y {
                
                let mut current = (x, y);
                while let Some(prev) = parent[current.1][current.0] {
                    self.path_to_player.push_front(current);
                    current = prev;
                }
                break;
            }
            
            
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; 
            
            for (dx, dy) in directions.iter() {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                
                if nx >= 0 && nx < FIELD_WIDTH as isize && ny >= 0 && ny < FIELD_HEIGHT as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    
                    if !visited[ny][nx] && self.can_move_to(nx, ny, field) {
                        visited[ny][nx] = true;
                        parent[ny][nx] = Some((x, y));
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }

    pub fn can_move_to(&self, x: usize, y: usize, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        if x >= FIELD_WIDTH || y >= FIELD_HEIGHT {
            return false;
        }
        
        match &field[y][x] {
            Cell::Wall => false,
            Cell::Door { state, .. } => *state,
            Cell::LiftingGates { state, .. } => *state,
            Cell::Toggle { .. } | Cell::Box { .. } | Cell::Safe { .. } | Cell::Exit { .. } => false,
            _ => true,
        }
    }

    pub fn move_towards_player(&mut self, player: &Character, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        
        if let Some(target_dir) = self.get_direction_to_player(player) {
            if self.side_of_the_world != target_dir {
                
                let turn_action = self.get_turn_direction(target_dir);
                match turn_action {
                    Some("left") => {
                        self.turn_left();
                        return true;
                    },
                    Some("right") => {
                        self.turn_right();
                        return true;
                    },
                    Some("around") => {
                        self.turn_around();
                        return true;
                    },
                    _ => {}, 
                }
            }
        }
        
        
        if self.move_forward(field) {
            self.consecutive_failed_moves = 0;
            true
        } else {
            self.consecutive_failed_moves += 1;
            
            
            if self.consecutive_failed_moves >= 2 {
                let turn_action = rand::thread_rng().gen_range(0..3);
                match turn_action {
                    0 => self.turn_left(),
                    1 => self.turn_right(),
                    2 => self.turn_around(),
                    _ => (),
                }
                self.consecutive_failed_moves = 0;
                true
            } else {
                false
            }
        }
    }

    pub fn move_towards_last_known_pos(&mut self, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        if let Some((last_x, last_y)) = self.last_known_player_pos {
            let dx = last_x as isize - self.x as isize;
            let dy = last_y as isize - self.y as isize;
            
            
            let target_dir = if dx == 0 && dy < 0 {
                SideOfTheWorld::North
            } else if dx == 0 && dy > 0 {
                SideOfTheWorld::South
            } else if dx > 0 && dy == 0 {
                SideOfTheWorld::East
            } else if dx < 0 && dy == 0 {
                SideOfTheWorld::West
            } else if dx.abs() > dy.abs() {
                if dx > 0 { SideOfTheWorld::East } else { SideOfTheWorld::West }
            } else {
                if dy > 0 { SideOfTheWorld::South } else { SideOfTheWorld::North }
            };
            
            
            if self.side_of_the_world != target_dir {
                let turn_action = self.get_turn_direction(target_dir);
                match turn_action {
                    Some("left") => {
                        self.turn_left();
                        return true;
                    },
                    Some("right") => {
                        self.turn_right();
                        return true;
                    },
                    Some("around") => {
                        self.turn_around();
                        return true;
                    },
                    _ => {}, 
                }
            }
            
            
            if self.move_forward(field) {
                true
            } else {
                
                let turn_action = rand::thread_rng().gen_range(0..3);
                match turn_action {
                    0 => self.turn_left(),
                    1 => self.turn_right(),
                    2 => self.turn_around(),
                    _ => (),
                }
                true
            }
        } else {
            false
        }
    }

    pub fn valid_move(&mut self, dx: isize, dy: isize, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        let nx = match self.x.checked_add_signed(dx) {
          Some(v) => v,
          None => return false,  
        };
        let ny = match self.y.checked_add_signed(dy) {
          Some(v) => v,
          None => return false,  
        };

        if nx < FIELD_WIDTH && ny < FIELD_HEIGHT {
            
            self.interact_with_cell(nx, ny, field);
            
            match &field[ny][nx] {
                Cell::Wall => false,
                Cell::Pass => {
                    self.x = nx;
                    self.y = ny;
                    true
                },
                Cell::Door { state, .. } => {
                    if *state {
                        self.x = nx;
                        self.y = ny;
                        true
                    } else {
                        false
                    }
                },
                Cell::Key {number} => {
                    self.x = nx;
                    self.y = ny;
                    if self.add_to_inventory(Item::Key(*number)) {
                        field[ny][nx] = Cell::Pass;
                    }
                    true
                },
                Cell::Paper { text } => {
                    self.x = nx;
                    self.y = ny;
                    if self.add_to_inventory(Item::Paper(text.clone())) {
                        field[ny][nx] = Cell::Pass;
                    }
                    true
                },
                Cell::Medkit { amount } => {
                    self.x = nx;
                    self.y = ny;
                    if self.add_to_inventory(Item::Medkit(*amount)) {
                        field[ny][nx] = Cell::Pass;
                    }
                    true
                },
                Cell::Toggle { .. } => false,
                Cell::LiftingGates { state, .. } => {
                    if *state {
                        self.x = nx;
                        self.y = ny;
                        true
                    } else {
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
                        false
                    }
                },
            }
        } else {
            false
        }
    }

    pub fn interact_with_cell(&mut self, x: usize, y: usize, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) {
        if x >= FIELD_WIDTH || y >= FIELD_HEIGHT {
            return;
        }
        
        
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        for (dx, dy) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            
            if nx >= 0 && nx < FIELD_WIDTH as isize && ny >= 0 && ny < FIELD_HEIGHT as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                
                if let Cell::Box { items } = &mut field[ny][nx] {
                    if !items.is_empty() && self.inventory.len() < 5 {
                        let item = items.remove(0);
                        self.add_to_inventory(item);
                    }
                }
            }
        }
    }

    pub fn update(&mut self, player: &Character, field: &mut [[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) -> bool {
        if !self.is_alive() {
            return false;
        }
        
        match self.state {
            EnemyState::Patrolling => {
                
                if self.can_see_player(player, field) {
                    self.state = EnemyState::Chasing;
                    self.last_known_player_pos = Some((player.x, player.y));
                    self.find_path_to_player(player, field);
                    return true;
                }
                if self.hit_wall {
                    
                    self.turn_around();
                    self.hit_wall = false; 
                    return true;
                } else {
                    
                    if self.can_move_forward(field) {
                        
                        let action = rand::thread_rng().gen_range(0..100);
                        
                        if action < 80 {
                            
                            if self.move_forward(field) {
                                return true;
                            } else {
                                
                                self.hit_wall = true;
                                return false;
                            }
                        } else {
                            
                            self.turn_around();
                            self.hit_wall = true; 
                            return true;
                        }
                    } else {
                        
                        self.hit_wall = true;
                        return false; 
                    }
                }
            }
            
            EnemyState::Chasing => {
                
                if self.can_see_player(player, field) {
                    self.last_known_player_pos = Some((player.x, player.y));
                    
                    
                    if self.is_adjacent_to_player(player) {
                        
                        if !self.is_facing_player(player) {
                            
                            if let Some(target_dir) = self.get_direction_to_player(player) {
                                let turn_action = self.get_turn_direction(target_dir);
                                match turn_action {
                                    Some("left") => self.turn_left(),
                                    Some("right") => self.turn_right(),
                                    Some("around") => self.turn_around(),
                                    _ => (),
                                }
                            }
                        }
                        return true;
                    }
                    
                    
                    self.move_towards_player(player, field);
                } else {
                    
                    if let Some((last_x, last_y)) = self.last_known_player_pos {
                        if self.x == last_x && self.y == last_y {
                            
                            self.state = EnemyState::Patrolling;
                            self.last_known_player_pos = None;
                            self.path_to_player.clear();
                            self.hit_wall = false; 
                        } else {
                            self.move_towards_last_known_pos(field);
                        }
                    } else {
                        
                        self.state = EnemyState::Patrolling;
                        self.hit_wall = false; 
                    }
                }
                
                true
            }
            
            EnemyState::Dead => false,
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

    pub fn loot_corpse(&mut self, player: &mut Character) -> bool {
        if self.state != EnemyState::Dead || self.inventory.is_empty() {
            return false;
        }
        
        
        loop {  
            print!("\x1bc");
            
            let col_width = 20;
            let total_width = col_width * 2 + 3;
            
            println!("┌{:─<width$}┐", "", width = total_width - 2);
            
            let title = " ТРУП ПРОТИВНИКА ";
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
                     "ВАШ ИНВЕНТАРЬ", 
                     "ТРУП ПРОТИВНИКА",
                     left = col_width,
                     right = col_width);
            
            println!("├{:─<left$}┼{:─<right$}┤", 
                     "", "", 
                     left = col_width,
                     right = col_width);
            
            let max_rows = 3.max(player.inventory.len()).max(self.inventory.len());
            
            for i in 0..max_rows {
                let left_item = if i < player.inventory.len() {
                    match &player.inventory[i] {
                        Item::Key(num) => format!("{} - Ключ №{}", i + 1, num),
                        Item::Paper(..) => format!("{} - Бумага", i + 1),
                        Item::Medkit(amount) => format!("{} - Аптечка (+{} HP)", i + 1, amount),
                    }
                } else {
                    "".to_string()
                };
                
                let right_item = if i < self.inventory.len() {
                    match &self.inventory[i] {
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
            println!(" 1-5 - взять предмет из трупа");
            println!(" a-e - положить предмет в труп");
            println!(" x   - закрыть");
            println!();
            
            let input = crate::game::read_input("Введите команду: ");
            
            let message;
            
            match input.as_str() {
                "x" => {
                    message = "Оставляете труп.".to_string();
                    println!("{}", message);
                    return true;
                },
                "1" | "2" | "3" | "4" | "5" => {
                    let index = input.parse::<usize>().unwrap() - 1;
                    if index < self.inventory.len() {
                        if player.inventory.len() < 5 {
                            let item = self.inventory.remove(index);
                            let item_name = match &item {
                                Item::Key(num) => format!("Ключ №{}", num),
                                Item::Paper(..) => "Бумага".to_string(),
                                Item::Medkit(amount) => format!("Аптечка (+{} HP)", amount),
                            };
                            player.inventory.push(item);
                            message = format!("Предмет '{}' перемещен в ваш инвентарь.", item_name);
                        } else {
                            message = "Ваш инвентарь заполнен! Максимум 5 предметов.".to_string();
                        }
                    } else {
                        message = "Неверный номер предмета!".to_string();
                    }
                },
                "a" | "b" | "c" | "d" | "e" => {
                    let index = match input.as_str() {
                        "a" => 0, "b" => 1, "c" => 2, "d" => 3, "e" => 4,
                        _ => 0,
                    };
                    
                    if index < player.inventory.len() {
                        if self.inventory.len() < 5 {
                            let item = player.inventory.remove(index);
                            let item_name = match &item {
                                Item::Key(num) => format!("Ключ №{}", num),
                                Item::Paper(..) => "Бумага".to_string(),
                                Item::Medkit(amount) => format!("Аптечка (+{} HP)", amount),
                            };
                            self.inventory.push(item);
                            message = format!("Предмет '{}' перемещен в труп.", item_name);
                        } else {
                            message = "У трупа нет места! Максимум 5 предметов.".to_string();
                        }
                    } else {
                        message = "В вашем инвентаре нет предмета под этим номером!".to_string();
                    }
                },
                _ => {
                    message = "Неверная команда! Используйте 1-5, a/b/c/d/e или x.".to_string();
                }
            }
            
            if !message.is_empty() {
                println!("{}", message);
                println!("Нажмите Enter чтобы продолжить...");
                let _ = std::io::stdin().read_line(&mut String::new());
            }
        }
    }
}