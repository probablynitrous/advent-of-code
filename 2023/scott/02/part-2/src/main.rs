use std::fs;

enum CubeColor {
    Red,
    Green,
    Blue,
}

struct Cube {
    color: CubeColor,
    num: u32,
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Error reading file");

    let sum: u32 = file.lines().map(|game| get_power_set(game)).sum();

    println!("sum: {sum}");
}

fn get_power_set(game: &str) -> u32 {
    let string_parts = game.split(":").collect::<Vec<&str>>();
    let revealed_sets = string_parts
        .last()
        .expect("Couldn't get last item")
        .trim()
        .split(";")
        .collect::<Vec<&str>>();

    let mut red = Cube {
        color: CubeColor::Red,
        num: 0,
    };
    let mut green = Cube {
        color: CubeColor::Green,
        num: 0,
    };
    let mut blue = Cube {
        color: CubeColor::Blue,
        num: 0,
    };

    revealed_sets.iter().for_each(|set| {
        let (new_red, new_green, new_blue) = get_max_cubes_for_set(set);

        if new_red.num > red.num {
            red.num = new_red.num
        }
        if new_green.num > green.num {
            green.num = new_green.num
        }
        if new_blue.num > blue.num {
            blue.num = new_blue.num
        }
    });

    return red.num * green.num * blue.num;
}

// 12 red cubes
// 13 green cubes
// 14 blue cubes
fn get_max_cubes_for_set(set: &str) -> (Cube, Cube, Cube) {
    let mut red = Cube {
        color: CubeColor::Red,
        num: 0,
    };
    let mut green = Cube {
        color: CubeColor::Green,
        num: 0,
    };
    let mut blue = Cube {
        color: CubeColor::Blue,
        num: 0,
    };
    set.split(",").for_each(|pull| {
        let pull_parts = pull.trim().split(" ").collect::<Vec<&str>>();
        let value = pull_parts
            .first()
            .expect("Couldn't get first")
            .parse::<u32>()
            .expect("Couldn't parse value");
        let color = pull_parts.last().expect("Couldn't get last");

        match color {
            &"blue" => blue.num = value,
            &"green" => green.num = value,
            &"red" => red.num = value,
            _ => todo!(),
        }
    });

    return (red, green, blue);
}
