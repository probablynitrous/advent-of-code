use rayon::prelude::*;
use std::{collections::HashSet, sync::Mutex, time::Instant};

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");

    let start = Instant::now();

    let ranges = file
        .lines()
        .take(1)
        .flat_map(|line| line.split(",").collect::<Vec<&str>>())
        .collect::<Vec<&str>>();

    let invalid_ids = Mutex::new(HashSet::new());

    ranges.par_iter().for_each(|range| {
        let splits = range
            .split("-")
            .map(|split| split.parse().unwrap())
            .collect::<Vec<u64>>();

        let (min, max) = (splits[0], splits[1]);
        let mut local_invalids = HashSet::new();

        for value in min..=max {
            let value_str = value.to_string();

            // Can't have pairs in a single-digit number
            if value_str.len() == 1 {
                continue;
            }

            // Check single digit case
            let value_hashset: HashSet<char> = value_str.chars().collect();
            if value_hashset.len() == 1 {
                local_invalids.insert(value);
                continue;
            }

            for pattern_size in 2..=value_str.len() / 2 {
                let (pattern, rest) = value_str.split_at(pattern_size);

                if rest
                    .as_bytes()
                    .chunks(pattern_size)
                    .all(|window| pattern.as_bytes() == window)
                {
                    local_invalids.insert(value);
                }
            }
        }

        invalid_ids.lock().unwrap().extend(local_invalids);
    });

    let sum = invalid_ids.lock().unwrap().iter().sum::<u64>();
    let total_time = Instant::now() - start;

    println!("Sum of invalid IDs: {}", sum);
    println!("Took {:?}", total_time);
}
