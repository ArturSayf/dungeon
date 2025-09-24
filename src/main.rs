use std::io;

struct  Character{
        x: i32,
        y: i32
    }
fn main() {
    let mut p = Character{x: 0, y: 0};

    loop {
        println!("Ваши координаты: {}.{}", p.x, p.y);
    println!("Двигайтесь! (left, right, up, down)");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Ошибка!");

    if input.contains("left"){
        p.x -= 1;
    } else if input.contains("right") {
        p.x += 1;s
    } else if input.contains("up") {
        p.y += 1
    } else if input.contains("down") {
        p.y -= 1
    } else {
        println!("Неверная команда");
        break;
    }   
    }

    
}
