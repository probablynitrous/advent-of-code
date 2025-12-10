use std::{collections::HashMap, time::Instant};

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    let start = Instant::now();

    let mut beam_count: HashMap<usize, usize> = HashMap::new();
    let mut split_count = 0;

    beam_count.insert(
        file.lines().take(1).collect::<Vec<&str>>()[0]
            .chars()
            .position(|ch| ch == 'S')
            .unwrap(),
        1,
    );

    for line in file.lines().skip(1) {
        for (x, ch) in line.chars().enumerate() {
            if ch == '^' {
                let count = beam_count.remove(&x).unwrap_or(0);

                if count != 0 {
                    split_count += 1;
                }
                *beam_count.entry(x - 1).or_insert(0) += count;
                *beam_count.entry(x + 1).or_insert(0) += count;
            }
        }
    }

    let total_time = Instant::now() - start;

    println!("Beam split {} times", split_count);
    println!("Took {:?}", total_time);
}
