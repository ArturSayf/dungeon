use std::io;

struct  Character{
        x: i32,
        y: i32,
        sideOfTheWorld: String
    }
fn main() {
    let mut p = Character{x: 0, y: 0, sideOfTheWorld: String::from("south")};

    loop {
        println!("Ваши координаты: {}.{}, Вы смотрите на {}", p.x, p.y, p.sideOfTheWorld);
    println!("Двигайтесь! (left, right, up, down, turn north, turn east, turn south, turn west, forward)");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Ошибка!");

        if input.contains("left"){
            p.x -= 1;
        } else if input.contains("right") {
            p.x += 1;
        } else if input.contains("up") {
            p.y += 1;
        } else if input.contains("down") {
            p.y -= 1;
        } else if input.contains("turn north") {
            p.sideOfTheWorld = String::from("north");
        } else if input.contains("turn south") {
            p.sideOfTheWorld = String::from("south");
        } else if input.contains("turn east") {
            p.sideOfTheWorld = String::from("east");
        } else if input.contains("turn west") {
            p.sideOfTheWorld = String::from("west");
        } else if input.contains("forward") {
            if p.sideOfTheWorld == "north"{
                p.y += 1;
            } else if p.sideOfTheWorld == "south" {
                p.y -= 1;
            } else if p.sideOfTheWorld == "east" {
                p.x += 1;
            } else if p.sideOfTheWorld == "west" {
                p.x -= 1;
            } else {
                println!("Неверная команда");
            }
        } else {
            println!("Неверная команда");
            break;
        }   
    }

    
}
