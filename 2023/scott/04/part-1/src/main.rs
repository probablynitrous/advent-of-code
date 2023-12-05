use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Error reading file");

    let start = Instant::now();

    let points: u32 = file
        .lines()
        .map(|card| {
            let both_numbers = card.split(":").collect::<Vec<&str>>()[1]
                .split("|")
                .collect::<Vec<&str>>();

            let winning_numbers: HashSet<u32> = HashSet::from_iter(
                both_numbers
                    .get(0)
                    .expect("Missing winning numbers")
                    .trim()
                    .split(" ")
                    .filter_map(|num| {
                        if num.len() == 0 {
                            return None;
                        }

                        return Some(num.parse::<u32>().expect("Couldn't parse number"));
                    }),
            );

            let your_numbers: HashSet<u32> = HashSet::from_iter(
                both_numbers
                    .get(1)
                    .expect("Missing your numbers")
                    .trim()
                    .split(" ")
                    .filter_map(|num| {
                        if num.len() == 0 {
                            return None;
                        }

                        return Some(num.parse::<u32>().expect("Couldn't parse number"));
                    })
                    .collect::<Vec<u32>>(),
            );

            let total_wins = &your_numbers.len() - (&your_numbers - &winning_numbers).len();

            if total_wins == 0 {
                return 0;
            }

            return u32::pow(2, (total_wins as u32) - 1);
        })
        .sum();

    let total_time = Instant::now() - start;

    println!("Points: {points}");
    println!("Took: {total_time:?}");
}
