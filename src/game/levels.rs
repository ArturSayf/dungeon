use crate::game::field::{Cell, SideOfTheWorld, Item};

pub const FIELD_WIDTH: usize = 22;
pub const FIELD_HEIGHT: usize = 12;

pub const PAPER_NOTE_1: &str = "Первая цифра кода: 7";
pub const PAPER_NOTE_2: &str = "Вторая цифра кода: 1";
pub const PAPER_NOTE_3: &str = "Третья цифра кода: 4";
pub const PAPER_NOTE_4: &str = "Последняя цифра кода: 8";
pub const PAPER_NOTE_5: &str = "Ключ от главного выхода у одного из охранников.";
pub const PAPER_NOTE_6: &str = "Местные охранники - не отличаются умом и сообразительностью.";
pub const PAPER_NOTE_7: &str = "Это был Ганс.";
pub const PAPER_NOTE_8: &str = "Это был Боб.";
pub const PAPER_NOTE_9: &str = "Это был Билл.";
pub const PAPER_NOTE_10: &str = "Это был Патрик.";
pub const PAPER_NOTE_11: &str = "Что бы открыть дверь, ведущую к выходу, нужно найти рычаг. Рычаг находится в комнате за дверью №1";
pub const PAPER_NOTE_12: &str = "................Аттестат............... 
Русский_язык................5 (отлично) 
Литература..................4 (хорошо) 
Иностранный_язык............5 (отлично)  
История.....................5 (отлично)
Обществознание..............4 (хорошо) 
География...................5 (отлично)
Математика..................5 (отлично)
Информатика.................5 (отлично)
Физика......................5 (отлично)
Биология....................4 (хорошо) 
Химия.......................4 (хорошо) 
Физическая_культура.........5 (отлично)";
pub const PAPER_NOTE_13: &str = "..............Объявление!..............\0\0 Студенты, не сдавшие сессию, повешены на третьем этаже главного корпуса.";
pub const PAPER_NOTE_14: &str = "..............Объявление!..............\0\0 Студенты, не сдающие верхнюю одежду в гардероб, будут повешены на третьем этаже гланого корпуса.";
pub const PAPER_NOTE_15: &str = "Код от сейфа: (5!+6!)*4/3-98";

#[derive(Clone)]
pub struct Level {
    pub name: String,
    pub field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT],
    pub player_start_x: usize,
    pub player_start_y: usize,
    pub player_start_direction: SideOfTheWorld,
    pub enemies: Vec<crate::game::Enemy>,
}

impl Level {
    pub fn new(number: u8, name: &str) -> Self {
        let paper_note_1 = PAPER_NOTE_1.to_string();
        let paper_note_2 = PAPER_NOTE_2.to_string();
        let paper_note_3 = PAPER_NOTE_3.to_string();
        let paper_note_4 = PAPER_NOTE_4.to_string();
        let paper_note_5 = PAPER_NOTE_5.to_string();
        let paper_note_6 = PAPER_NOTE_6.to_string();
        let paper_note_7 = PAPER_NOTE_7.to_string();
        let paper_note_8 = PAPER_NOTE_8.to_string();
        let paper_note_9 = PAPER_NOTE_9.to_string();
        let paper_note_10 = PAPER_NOTE_10.to_string();
        let paper_note_11 = PAPER_NOTE_11.to_string();
        let paper_note_12 = PAPER_NOTE_12.to_string();
        let paper_note_13 = PAPER_NOTE_13.to_string();
        let paper_note_14 = PAPER_NOTE_14.to_string();
        let paper_note_15 = PAPER_NOTE_15.to_string();

        let (field, player_x, player_y, player_dir, enemies) = match number {
            1 => Self::create_level_1(&paper_note_1, &paper_note_2, &paper_note_3, &paper_note_4),
            2 => Self::create_level_2(&paper_note_5, &paper_note_6, &paper_note_7, &paper_note_8, &paper_note_9, &paper_note_10),
            3 => Self::create_level_3(&paper_note_11, &paper_note_12),
            4 => Self::create_level_4(&paper_note_13, &paper_note_14, &paper_note_15),
            _ => Self::create_level_1(&paper_note_1, &paper_note_2, &paper_note_3, &paper_note_4),
        };

        Self {
            name: name.to_string(),
            field,
            player_start_x: player_x,
            player_start_y: player_y,
            player_start_direction: player_dir,
            enemies,
        }
    }

    fn create_level_1(paper_note_1: &str, paper_note_2: &str, paper_note_3: &str, paper_note_4: &str) -> 
        ([[Cell; FIELD_WIDTH]; FIELD_HEIGHT], usize, usize, SideOfTheWorld, Vec<crate::game::Enemy>) {
        
        let field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT] = [
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Safe { state: false, direction: SideOfTheWorld::East, password: 7148, items: vec![Item::Key(4)] }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Exit { state: false, direction: SideOfTheWorld::North }, Cell::Wall, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Door { direction: SideOfTheWorld::West, number: 4, state: false }, Cell::Pass, Cell::Wall, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Key { number: 3 }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::South, number: 2, state: false }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Toggle { state: false, number: 2, direction: SideOfTheWorld::South }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Box { items: vec![Item::Paper(paper_note_2.to_string())] }, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Toggle { state: false, number: 1, direction: SideOfTheWorld::South }, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Box { items: vec![Item::Paper(paper_note_3.to_string()), Item::Key(2)] }, Cell::Wall],
            [Cell::Wall, Cell::Box { items: vec![Item::Paper(paper_note_1.to_string())] }, Cell::LiftingGates { state: false, number: 2, direction: SideOfTheWorld::West }, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::LiftingGates { state: false, number: 1, direction: SideOfTheWorld::West }, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Door { direction: SideOfTheWorld::East, number: 1, state: false }, Cell::Pass, Cell::Wall, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::North, number: 3, state: false }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Box { items: vec![Item::Paper(paper_note_4.to_string())] }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Key { number: 1 }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
        ];

        let enemies = vec![];

        (field, 12, 5, SideOfTheWorld::South, enemies)
    }

    fn create_level_2(paper_note_5: &str, paper_note_6: &str, paper_note_7: &str, paper_note_8: &str, paper_note_9: &str, paper_note_10: &str) -> 
        ([[Cell; FIELD_WIDTH]; FIELD_HEIGHT], usize, usize, SideOfTheWorld, Vec<crate::game::Enemy>) {
        
        let field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT] = [
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Paper { text: paper_note_6.to_string() }, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Medkit { amount: 20 }, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Paper { text: paper_note_5.to_string() }, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Box { items: vec![Item::Medkit(20), Item::Medkit(20), Item::Medkit(20)] }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::South, number: 1, state: false }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Medkit { amount: 20 }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Box { items: vec![Item::Medkit(20), Item::Medkit(20), Item::Medkit(20)] }, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Exit { state: false, direction: SideOfTheWorld::South }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
        ];

        let mut enemy1 = crate::game::Enemy::new(10, 5, SideOfTheWorld::West);
        let mut enemy2 = crate::game::Enemy::new(15, 8, SideOfTheWorld::North);
        let mut enemy3 = crate::game::Enemy::new(7, 1, SideOfTheWorld::West);
        let mut enemy4 = crate::game::Enemy::new(15, 10, SideOfTheWorld::North);
        enemy1.add_to_inventory(Item::Paper(paper_note_7.to_string()));
        enemy2.add_to_inventory(Item::Key(1));
        enemy2.add_to_inventory(Item::Paper(paper_note_8.to_string()));
        enemy3.add_to_inventory(Item::Paper(paper_note_9.to_string()));
        enemy4.add_to_inventory(Item::Paper(paper_note_10.to_string()));

        let enemies = vec![enemy1, enemy2, enemy3, enemy4];

        (field, 1, 1, SideOfTheWorld::East, enemies)
    }

    fn create_level_3(paper_note_11: &str, paper_note_12: &str) -> 
        ([[Cell; FIELD_WIDTH]; FIELD_HEIGHT], usize, usize, SideOfTheWorld, Vec<crate::game::Enemy>) {
        
        let field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT] = [
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::South, number: 1, state: false }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::LiftingGates { state: false, number: 1, direction: SideOfTheWorld::North }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Toggle { state: false, number: 2, direction: SideOfTheWorld::West }, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall], 
            [Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Paper { text: paper_note_12.to_string() }, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Box { items: vec![Item::Paper(paper_note_11.to_string()), Item::Key(1)] }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Pass, Cell::LiftingGates { state: false, number: 2, direction: SideOfTheWorld::West }, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Toggle { state: false, number: 1, direction: SideOfTheWorld::East }, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall],
            [Cell::Wall, Cell::Exit { state: false, direction: SideOfTheWorld::South }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
        ];

        let enemies = vec![];

        (field, 18, 9, SideOfTheWorld::South, enemies)
    }

    fn create_level_4(paper_note_13: &str, paper_note_14: &str, paper_note_15: &str) -> 
        ([[Cell; FIELD_WIDTH]; FIELD_HEIGHT], usize, usize, SideOfTheWorld, Vec<crate::game::Enemy>) {
        
        let field: [[Cell; FIELD_WIDTH]; FIELD_HEIGHT] = [
    [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
    [Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::LiftingGates { state: false, number: 3, direction: SideOfTheWorld::West }, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::LiftingGates { state: false, number: 5, direction: SideOfTheWorld::West }, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Exit { state: false, direction: SideOfTheWorld::East }],
    [Cell::Wall, Cell::Pass, Cell::Wall, Cell::LiftingGates { state: false, number: 2, direction: SideOfTheWorld::North }, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::LiftingGates { state: false, number: 4, direction: SideOfTheWorld::North }, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Door { direction: SideOfTheWorld::North, number: 5, state: false }, Cell::Wall, Cell::Wall],
    [Cell::Toggle { state: false, number: 1, direction: SideOfTheWorld::East }, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall],
    [Cell::Wall, Cell::LiftingGates { state: false, number: 1, direction: SideOfTheWorld::North }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::South, number: 1, state: false }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::South, number: 2, state: false }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::South, number: 3, state: false }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Door { direction: SideOfTheWorld::South, number: 4, state: false }, Cell::Wall, Cell::Wall],
    [Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Wall],
    [Cell::Wall, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Paper { text: paper_note_15.to_string() }, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Box { items: vec![Item::Key(4)] }, Cell::Wall, Cell::Pass, Cell::Wall],
    [Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Box { items: vec![Item::Key(3)] }, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall],
    [Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Box { items: vec![Item::Paper(paper_note_14.to_string()), Item::Key(2)] }, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall],
    [Cell::Wall, Cell::Wall, Cell::Pass, Cell::Toggle { state: false, number: 2, direction: SideOfTheWorld::West }, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Wall],
    [Cell::Wall, Cell::Box { items: vec![Item::Paper(paper_note_13.to_string()), Item::Key(1)] }, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Pass, Cell::Wall, Cell::Wall, Cell::Toggle { state: false, number: 4, direction: SideOfTheWorld::North }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Toggle { state: false, number: 5, direction: SideOfTheWorld::North }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Safe { state: false, direction: SideOfTheWorld::South, password: 2142, items: vec![Item::Key(5)] }, Cell::Wall],
    [Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Toggle { state: false, number: 3, direction: SideOfTheWorld::North }, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
];

        let enemies = vec![];

        (field, 1, 1, SideOfTheWorld::South, enemies)
    }

    pub fn get_all_levels() -> Vec<Level> {
        vec![
            Level::new(1, "Подвал Виктора Носаня"),
            Level::new(2, "Лабиринт ужасов"),
            Level::new(3, "Школа №24"),
            Level::new(4, "Под СИНХом"),
        ]
    }
}