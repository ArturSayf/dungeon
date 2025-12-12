use std::io::{self, Write};

pub fn read_input(promt: &str) -> String {
    print!("{}", promt);
    io::stdout().flush().expect("Не удалось очистить буфер вывода.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода!");
    input.trim().to_string()
}