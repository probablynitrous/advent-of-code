use std::time::Instant;

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    let start = Instant::now();

    let sum = file
        .lines()
        .map(|line| {
            let max = line.chars().map(|c| c.to_digit(10).unwrap()).max().unwrap();
            let max_index = line
                .chars()
                .position(|c| c.to_digit(10).unwrap() == max)
                .unwrap();

            if max_index == 0 {
                // Max is at the start of the line, skip that and find the next max
                let second_max = line.chars().skip(1).max().unwrap();
                return (max.to_string() + second_max.to_string().as_str())
                    .parse::<u64>()
                    .unwrap();
            }

            if max_index == line.len() - 1 {
                // Max is at the end of the line, ignore that and find the next max in the same way
                // again
                let second_max = line.chars().take(line.len() - 1).max().unwrap();
                return (second_max.to_string() + max.to_string().as_str())
                    .parse::<u64>()
                    .unwrap();
            }

            // Now we have to check both sides:
            // The max to the left of the digit
            let left_max_joltage = (line.chars().take(max_index).max().unwrap().to_string()
                + max.to_string().as_str())
            .parse::<u64>()
            .unwrap();
            let right_max_joltage = (max.to_string()
                + line
                    .chars()
                    .skip(max_index + 1)
                    .max()
                    .unwrap()
                    .to_string()
                    .as_str())
            .parse::<u64>()
            .unwrap();

            return left_max_joltage.max(right_max_joltage);
        })
        .sum::<u64>();

    let total_time = Instant::now() - start;

    println!("Total joltage: {}", sum);
    println!("Took {:?}", total_time);
}
