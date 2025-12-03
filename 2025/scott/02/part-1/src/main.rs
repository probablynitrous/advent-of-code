use std::time::Instant;

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");

    let start = Instant::now();

    let ranges = file
        .lines()
        .take(1)
        .flat_map(|line| line.split(",").collect::<Vec<&str>>())
        .collect::<Vec<&str>>();

    let mut invalid_ids: Vec<u64> = Vec::new();

    for range in ranges {
        let splits = range
            .split("-")
            .map(|split| split.parse().unwrap())
            .collect::<Vec<u64>>();

        let (min, max) = (splits[0], splits[1]);

        for value in min..=max {
            let value_str = value.to_string();
            let (first_half, second_half) = value_str.split_at(value_str.len() / 2);

            if first_half == second_half {
                invalid_ids.push(value);
                continue;
            }
        }
    }

    let sum = invalid_ids.iter().sum::<u64>();
    let total_time = Instant::now() - start;

    println!("Sum of invalid IDs: {}", sum);
    println!("Took {:?}", total_time);
}
