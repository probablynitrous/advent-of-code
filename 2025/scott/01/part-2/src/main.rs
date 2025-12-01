use std::time::Instant;

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");

    let start = Instant::now();

    let mut zero_visits = 0;
    let mut cursor: i64 = 50;

    file.lines().for_each(|line| {
        let direction = line.chars().nth(0).unwrap();
        let distance = line[1..].parse::<i64>().unwrap();

        let new_cursor_position = match direction {
            'L' => cursor - distance,
            'R' => cursor + distance,
            _ => unimplemented!(),
        };

        zero_visits += distance / 100;

        let remainder = distance % 100;

        match direction {
            'L' => {
                if cursor - remainder <= 0 && cursor != 0 {
                    zero_visits += 1;
                }
            }
            'R' => {
                if cursor + remainder >= 100 {
                    zero_visits += 1;
                }
            }
            _ => unimplemented!(),
        }

        cursor = new_cursor_position.rem_euclid(100);
    });

    let total_time = Instant::now() - start;

    println!("Final cursor position: {}", cursor);
    println!("Zero visits: {}", zero_visits);
    println!("Took {:?}", total_time);
}
