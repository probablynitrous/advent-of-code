use std::time::Instant;

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    let start = Instant::now();

    let sum = file
        .lines()
        .map(|line| {
            let max_length = 12;
            let mut value = Vec::new();
            let mut start_index = 0;

            while value.len() < max_length {
                let remaining_space = max_length - value.len();

                let start = start_index;
                let end = line.len() - remaining_space;

                let mut highest: u32 = 0;
                let mut highest_index: usize = 0;

                for i in start..=end {
                    if line.chars().nth(i).unwrap().to_digit(10).unwrap() > highest {
                        highest = line.chars().nth(i).unwrap().to_digit(10).unwrap();
                        highest_index = i;

                        if highest == 9 {
                            break;
                        }
                    }
                }

                value.push(highest);
                start_index = highest_index + 1;
            }

            return value
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<u64>()
                .unwrap();
        })
        .sum::<u64>();

    let total_time = Instant::now() - start;

    println!("Total joltage: {}", sum);
    println!("Took {:?}", total_time);
}
