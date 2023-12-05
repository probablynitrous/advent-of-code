use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Error reading file");

    let sum: u32 = file
        .lines()
        .filter_map(|game| {
            if is_game_possible(game) {
                Some(get_game_id(game))
            } else {
                None
            }
        })
        .sum();

    println!("sum: {sum}");
}

fn is_game_possible(game: &str) -> bool {
    let string_parts = game.split(":").collect::<Vec<&str>>();
    let revealed_sets = string_parts
        .last()
        .expect("Couldn't get last item")
        .trim()
        .split(";")
        .collect::<Vec<&str>>();

    revealed_sets.iter().all(|set| is_set_possible(set))
}

// 12 red cubes
// 13 green cubes
// 14 blue cubes
fn is_set_possible(set: &str) -> bool {
    return set.split(",").all(|pull| {
        let pull_parts = pull.trim().split(" ").collect::<Vec<&str>>();
        let value = pull_parts
            .first()
            .expect("Couldn't get first")
            .parse::<u32>()
            .expect("Couldn't parse value");
        let color = pull_parts.last().expect("Couldn't get last");

        match color {
            &"blue" => value <= 14,
            &"green" => value <= 13,
            &"red" => value <= 12,
            _ => todo!(),
        }
    });
}

fn get_game_id(game: &str) -> u32 {
    let string_parts = game.split(":").collect::<Vec<&str>>();

    let game_id_parts = string_parts.first().expect("Couldn't get first item");

    game_id_parts
        .split(" ")
        .collect::<Vec<&str>>()
        .last()
        .expect("Couldn't get last item")
        .parse::<u32>()
        .expect("Couldn't parse digit")
}
