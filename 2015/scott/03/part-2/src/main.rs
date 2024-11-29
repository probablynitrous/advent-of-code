use std::{collections::HashMap, fs};

#[derive(Clone, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Couldn't read file");

    let mut santa_pos = Position { x: 0, y: 0 };
    let mut robot_pos = Position { x: 0, y: 0 };
    let mut houses: HashMap<Position, u32> = HashMap::new();

    file.trim()
        .chars()
        .collect::<Vec<char>>()
        .chunks_exact(2)
        .for_each(|directions| {
            parse_direction(
                &mut santa_pos,
                directions.get(0).expect("Couldn't get santa's direction"),
                &mut houses,
            );
            parse_direction(
                &mut robot_pos,
                directions.get(1).expect("Couldn't get robot's direction"),
                &mut houses,
            );
        });

    println!("number of houses: {}", houses.len());
}

fn parse_direction(
    current_pos: &mut Position,
    direction: &char,
    houses: &mut HashMap<Position, u32>,
) {
    match direction {
        '>' => current_pos.x += 1,
        '^' => current_pos.y -= 1,
        'v' => current_pos.y += 1,
        '<' => current_pos.x -= 1,
        _ => todo!("Encountered an unexpected character: {}", direction),
    }

    houses.insert(
        current_pos.clone(),
        if let Some(visits) = houses.get(&current_pos) {
            visits + 1
        } else {
            1
        },
    );
}
