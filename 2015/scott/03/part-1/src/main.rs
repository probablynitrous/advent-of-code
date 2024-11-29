use std::{collections::HashMap, fs};

#[derive(Clone, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Couldn't read file");

    let mut position = Position { x: 0, y: 0 };
    let mut houses: HashMap<Position, u32> = HashMap::new();

    file.trim().chars().for_each(|direction| {
        if direction.to_string().eq("") {
            return;
        }

        match direction {
            '>' => position.x += 1,
            '^' => position.y -= 1,
            'v' => position.y += 1,
            '<' => position.x -= 1,
            _ => todo!("Encountered an unexpected character: {}", direction),
        }

        houses.insert(
            position.clone(),
            if let Some(current_count) = houses.get(&position) {
                current_count + 1
            } else {
                1
            },
        );
    });

    println!("number of houses: {}", houses.len());
}
