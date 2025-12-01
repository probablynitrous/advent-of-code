use std::time::Instant;

fn main() {
    let file = std::fs::read_to_string("./test-input.txt").expect("Failed to read input file");

    let start = Instant::now();

    let mut zero_visits = 0;
    let mut cursor: i64 = 50;

    file.lines().for_each(|line| {
        let direction = line.chars().nth(0).unwrap();
        let distance = line[1..].parse::<i64>().unwrap();
        // let full_rotations = distance / 100;

        for i in 0..distance {
            let new_cursor_position = match direction {
                'L' => cursor - i,
                'R' => cursor + i,
                _ => unimplemented!(),
            }
            .rem_euclid(100);

            if new_cursor_position == 0 {
                zero_visits += 1;
            }

            cursor = new_cursor_position;
        }

        // let new_cursor_position = match direction {
        //     'L' => cursor - distance,
        //     'R' => cursor + distance,
        //     _ => unimplemented!(),
        // };
        //
        // if
        // // If we arrive on a zero
        // new_cursor_position == 0
        //     ||
        // // If we've gone past zero (but didn't start on it)
        // (new_cursor_position.abs() != new_cursor_position && cursor != 0)
        // ||
        // // If we're beyond the limit in either direction
        // new_cursor_position / 100 > 0
        // {
        //     zero_visits += 1;
        // }
        //
        // // Don't need to include the last rotation as we've already captured that in the "landing
        // // on zero" case
        // if full_rotations > 0 {
        //     if new_cursor_position.rem_euclid(100) == 0 {
        //         zero_visits += full_rotations - 1;
        //     } else {
        //         zero_visits += full_rotations;
        //     }
        // }

        // println!("cursor: {}", cursor);
        // println!("instruction: {}{}", direction, distance);
        // println!("new_cursor_position: {}", new_cursor_position);
        // println!("zero visit count: {}", zero_visits);
        // println!("=================");

        // cursor = new_cursor_position.rem_euclid(100);
    });
    let total_time = Instant::now() - start;

    println!("Final cursor position: {}", cursor);
    println!("Zero visits: {}", zero_visits);
    println!("Took {:?}", total_time);
}
