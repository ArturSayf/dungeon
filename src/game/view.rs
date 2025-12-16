use crate::game::character::Character;
use crate::game::field::{Cell, SideOfTheWorld};
use crate::game::images::{
    CLOSE_DOORS, OPEN_DOORS, TOGGLE_OFF, TOGGLE_ON, WALLS, 
    CLOSE_LIFTING_GATES, OPEN_LIFTING_GATES, KEYS, PAPER, 
    MEDKIT, BOX, CLOSE_SAFE, OPEN_SAFE, OPEN_EXIT, CLOSE_EXIT, 
    ENEMY, DEAD_ENEMY,
};
use crate::game::enemy::{Enemy, EnemyState};
use crate::game::{FIELD_WIDTH, FIELD_HEIGHT};

pub fn fpv(character: &Character, field: &[[Cell; FIELD_WIDTH]; FIELD_HEIGHT], enemies: &[Enemy]) {
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
        let nx = match character.x.checked_add_signed(dx) {
          Some(v) => v,
          None => continue,  
        };
        let ny = match character.y.checked_add_signed(dy) {
          Some(v) => v,
          None => continue,  
        };

        if nx < FIELD_WIDTH && ny < FIELD_HEIGHT {
            // Сначала проверяем врагов
            let mut enemy_found = false;
            for enemy in enemies {
                if enemy.x == nx && enemy.y == ny {
                    match enemy.state {
                        EnemyState::Dead => {
                            draw(&mut view, DEAD_ENEMY[index]);
                        },
                        EnemyState::Patrolling | EnemyState::Chasing => {
                            let enemy_index = (enemy.side_of_the_world as isize - character.side_of_the_world as isize).rem_euclid(4) as usize;
                            draw(&mut view, ENEMY[index][enemy_index]);
                        }
                    }
                    enemy_found = true;
                    break;
                }
            }
            
            if enemy_found {
                continue;
            }
            
            match field[ny as usize][nx as usize] {
                Cell::Wall => draw(&mut view, WALLS[index]),
                Cell::Door { direction: door_direction, state, .. } => {
                    if state {
                        let door_index = (door_direction as isize - character.side_of_the_world as isize).rem_euclid(4) as usize;
                        draw(&mut view, OPEN_DOORS[index][door_index]);
                    } else {
                        let door_index = (door_direction as isize - character.side_of_the_world as isize).rem_euclid(4) as usize;
                        draw(&mut view, CLOSE_DOORS[index][door_index]);
                    }
                },
                Cell::Key { .. } => draw(&mut view, KEYS[index]),
                Cell::Paper { .. } => draw(&mut view, PAPER[index]),
                Cell::Medkit { .. } => draw(&mut view, MEDKIT[index]),
                Cell::Pass => continue,
                Cell::Toggle { state, direction: toggle_direction, .. } => {
                    if state{
                        let toggle_index = (toggle_direction as isize - character.side_of_the_world as isize).rem_euclid(4) as usize;
                        draw(&mut view, TOGGLE_ON[index][toggle_index]);
                    } else {
                        let toggle_index = (toggle_direction as isize - character.side_of_the_world as isize).rem_euclid(4) as usize;
                        draw(&mut view, TOGGLE_OFF[index][toggle_index]);
                    }    
                },
                Cell::LiftingGates { state, direction, .. } => {
                    if state{
                        let gates_index = (direction as isize - character.side_of_the_world as isize).rem_euclid(4) as usize;
                    draw(&mut view, OPEN_LIFTING_GATES[index][gates_index]);
                    } else {
                        let gates_index = (direction as isize - character.side_of_the_world as isize).rem_euclid(4) as usize;
                    draw(&mut view, CLOSE_LIFTING_GATES[index][gates_index]);
                    }
                },
                Cell::Box { .. } => draw(&mut view, BOX[index]),
                Cell::Safe { state, direction: safe_direction, .. } => {
                    if state {
                        let safe_index = (safe_direction as isize - character.side_of_the_world as isize).rem_euclid(4) as usize;
                        draw(&mut view, OPEN_SAFE[index][safe_index]);
                    } else {
                        let safe_index = (safe_direction as isize - character.side_of_the_world as isize).rem_euclid(4) as usize;
                        draw(&mut view, CLOSE_SAFE[index][safe_index]);
                    }
                },
                Cell::Exit {direction: exit_direction, state } => {
                    if state {
                        let exit_index = (exit_direction as isize - character.side_of_the_world as isize).rem_euclid(4) as usize;
                        draw(&mut view, OPEN_EXIT[index][exit_index]);
                    } else {
                        let exit_index = (exit_direction as isize - character.side_of_the_world as isize).rem_euclid(4) as usize;
                        draw(&mut view, CLOSE_EXIT[index][exit_index]);
                    }
                },
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