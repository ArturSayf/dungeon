use crate::game::character::Character;
use crate::game::field::{Cell, FIELD_WIDTH, FIELD_HEIGHT};
use crate::game::images::{WALLS, DOORS};
use crate::game::field::SideOfTheWorld;

pub fn fpv(character: &Character, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT]) {
    #[rustfmt::skip]
    let fov = [
        (2, 3), (-2, 3), (1, 3), (-1, 3), (0, 3),
        (1, 2), (-1, 2), (0, 2),
        (1, 1), (-1, 1), (0, 1),
        (1, 0), (-1, 0),
    ];

    let turn_fov = match character.side_of_the_world {
        SideOfTheWorld::South => fov,
        SideOfTheWorld::North => fov.map(|(dx, dy)| (-dx, -dy)),
        SideOfTheWorld::West => fov.map(|(dx, dy)| (-dy, dx)),
        SideOfTheWorld::East => fov.map(|(dx, dy)| (dy, -dx)),
    };

    let mut view = [[' '; 26]; 14];

    for (index, (dx, dy)) in turn_fov.into_iter().enumerate() {
        let nx = character.x as isize + dx;
        let ny = character.y as isize + dy;

        if nx >= 0 && nx < FIELD_WIDTH as isize && ny >= 0 && ny < FIELD_HEIGHT as isize {
            match field[ny as usize][nx as usize] {
                Cell::Wall => draw(&mut view, WALLS[index]),
                Cell::Door { direction: door_direction } => {
                    let door_index = (door_direction as isize - character.side_of_the_world as isize).rem_euclid(4) as usize;
                    draw(&mut view, DOORS[index][door_index]);
                }
                Cell::Key => {
                    continue;
                }
                Cell::Pass => continue,
            }
        } else {
            draw(&mut view, WALLS[index]);
        }
    }

    for line in view {
        for ch in line {
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

fn draw(view: &mut [[char; 26]; 14], image: &str) {
    for (y, line) in image.lines().enumerate() {
        if y >= 14 {
            break;
        }
        for (x, ch) in line.chars().enumerate() {
            if x >= 26 {
                break;
            }
            if ch != '`' {
                view[y][x] = ch;
            }
        }
    }
}