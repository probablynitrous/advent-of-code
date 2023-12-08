use once_cell::sync::Lazy;
use regex::{CaptureMatches, Regex};
use std::{fs, time::Instant};

fn main() {
    let file = fs::read_to_string("input.txt").expect("Couldn't find file to read");

    let start = Instant::now();

    let lines = file.lines();

    let all_symbols = file
        .lines()
        .map(|line| {
            line.split("")
                .filter(|c| c.len() > 0)
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    let sum: i32 = lines
        .enumerate()
        .map(|(y_pos, line)| {
            let mut cog_values: Vec<i32> = Vec::new();

            line.chars().enumerate().for_each(|(x_pos, char)| {
                if char != '*' {
                    return;
                }

                let adjacent_numbers =
                    find_surrounding_numbers(x_pos as i32, y_pos as i32, &all_symbols);

                if adjacent_numbers.len() != 2 {
                    return;
                }

                cog_values.push(adjacent_numbers[0] * adjacent_numbers[1]);
            });

            return cog_values.iter().sum::<i32>();
        })
        .sum();

    let total_time = Instant::now() - start;

    println!("sum: {sum}");
    println!("Took {total_time:?}");
}

fn get_line_captures_iter(haystack: &str) -> CaptureMatches {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"[0-9]{1,3}").expect("Couldn't generate regex"));
    RE.captures_iter(haystack)
}

fn find_surrounding_numbers(x_pos: i32, y_pos: i32, all_symbols: &Vec<Vec<&str>>) -> Vec<i32> {
    let mut adjacent_numbers: Vec<i32> = Vec::new();
    if y_pos != 0 {
        // Check all the numbers above the cog
        get_line_captures_iter(&all_symbols[(y_pos as usize) - 1].join("")).for_each(|capture| {
            let cap = capture.get(0).expect("Couldn't get first capture");
            if (cap.start() as i32) < x_pos - 3 || (cap.start() as i32) > x_pos + 1 {
                return;
            }

            if (cap.end() as i32) - 1 < x_pos - 1 || (cap.end() as i32 - 1) > x_pos + 3 {
                return;
            }

            adjacent_numbers.push(
                cap.as_str()
                    .parse::<i32>()
                    .expect("Couldn't parse captured number above"),
            );
        });
    }

    if y_pos != all_symbols.len() as i32 - 1 as i32 {
        // Check all the numbers below the cog
        get_line_captures_iter(&all_symbols[y_pos as usize + 1].join("")).for_each(|capture| {
            let cap = capture.get(0).expect("Couldn't get first capture");
            if (cap.start() as i32) < x_pos - 3 || (cap.start() as i32) > x_pos + 1 {
                return;
            }

            if (cap.end() as i32) - 1 < x_pos - 1 || (cap.end() as i32 - 1) > x_pos + 3 {
                return;
            }

            adjacent_numbers.push(
                cap.as_str()
                    .parse::<i32>()
                    .expect("Couldn't parse captured number below"),
            );
        });
    }

    // Check to the left on the same level
    if x_pos != 0 {
        // If the char to the left of the cog is a digit, then we should continue
        // to try and pass all those
        if all_symbols[y_pos as usize][x_pos as usize - 1]
            .parse::<i32>()
            .is_ok()
        {
            get_line_captures_iter(&all_symbols[y_pos as usize].join("")).for_each(|capture| {
                let cap = capture.get(0).expect("Couldn't get first capture");
                if (
                    // cap.end() always returns the byte _after_ the match
                    cap.end() - 1
                ) as i32
                    != x_pos - 1
                {
                    return;
                }

                adjacent_numbers.push(
                    cap.as_str()
                        .parse::<i32>()
                        .expect("Couldn't parse captured number to the left"),
                );
            });
        }
    }

    // Check to the right on the same level
    if x_pos != (all_symbols[0].len() as i32 - 1) {
        // If the char to the right of the cog is a digit, then we should continue
        // to try and pass all those
        if all_symbols[y_pos as usize][x_pos as usize + 1]
            .parse::<i32>()
            .is_ok()
        {
            get_line_captures_iter(&all_symbols[y_pos as usize].join("")).for_each(|capture| {
                let cap = capture.get(0).expect("Couldn't get first capture");
                if cap.start() as i32 != x_pos + 1 {
                    return;
                }

                adjacent_numbers.push(
                    cap.as_str()
                        .parse::<i32>()
                        .expect("Couldn't parse captured number to the left"),
                );
            });
        }
    }

    return adjacent_numbers;
}
