fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    let mut zero_visits = 0;
    let mut cursor: i64 = 50;

    file.lines().for_each(|line| {
        let direction = line.chars().nth(0).unwrap();
        let distance = line[1..].parse::<i64>().unwrap();

        let new_cursor_position = match direction {
            'L' => (cursor - distance).rem_euclid(100),
            'R' => (cursor + distance).rem_euclid(100),
            _ => unimplemented!(),
        };

        if new_cursor_position == 0 {
            zero_visits += 1;
        }

        cursor = new_cursor_position;
    });

    println!("Final cursor position: {}", cursor);
    println!("Zero visits: {}", zero_visits);
}
