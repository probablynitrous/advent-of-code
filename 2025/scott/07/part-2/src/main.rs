use std::{collections::HashMap, time::Instant};

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    let start = Instant::now();

    let mut beam_count: HashMap<usize, usize> = HashMap::new();

    beam_count.insert(
        file.lines().take(1).collect::<Vec<&str>>()[0]
            .chars()
            .position(|ch| ch == 'S')
            .unwrap(),
        1,
    );

    for line in file.lines().skip(1) {
        let mut next_beam_count = HashMap::new();

        for (&position, &count) in &beam_count {
            if line.chars().nth(position) == Some('^') {
                // Split: add to left and right (position itself not included)
                *next_beam_count.entry(position - 1).or_insert(0) += count;
                *next_beam_count.entry(position + 1).or_insert(0) += count;
            } else {
                // Continue: stays at same position
                *next_beam_count.entry(position).or_insert(0) += count;
            }
        }

        beam_count = next_beam_count;
    }

    let total_routes = beam_count.values().sum::<usize>();

    let total_time = Instant::now() - start;

    println!("Total routes: {}", total_routes);
    println!("Took {:?}", total_time);
}
