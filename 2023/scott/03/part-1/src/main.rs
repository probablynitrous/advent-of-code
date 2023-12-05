use std::{fs, time::Instant};

struct PartNumber {
    value: String,
    start_index: Option<usize>,
    is_valid: bool,
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Couldn't find file to read");

    let start = Instant::now();

    let lines = file.lines();

    let all_symbols = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let sum: u32 = lines
        .enumerate()
        .map(|(y_pos, line)| {
            let mut valid_numbers_in_line: Vec<u32> = Vec::new();
            let mut working_number = PartNumber {
                value: "".to_string(),
                start_index: None,
                is_valid: false,
            };

            // What we want to do is:
            // - Go through each line, only bothering to stop if there's a
            //   number
            //
            // - We set a boolean to say that we're working on a number, and we
            //   have a boolean (that defaults to false) to say whether it's
            //   valid or not
            //
            // - We go through and do all the annoying checking, based on the
            //   individual digits position in the line (which decides what other
            //   places we check).
            //
            // - Then we should have a list of numbers that are valid

            line.chars().enumerate().for_each(|(x_pos, char)| {
                // Skip over the elements that aren't a number
                if !char.is_digit(10) {
                    if working_number.start_index.is_some() {
                        if working_number.is_valid {
                            valid_numbers_in_line.push(
                                working_number
                                    .value
                                    .parse::<u32>()
                                    .expect("Couldn't parse value"),
                            );
                        }

                        // Reset the working number for the next time we hit one
                        working_number.start_index = None;
                        working_number.value = "".to_string();
                        working_number.is_valid = false;
                    }
                    return;
                }

                if working_number.start_index.is_none() {
                    working_number.start_index = Some(x_pos);
                }
                working_number.value.push(char);

                // We only want to update this in the event that we've not
                // previously done so - to prevent overwriting a valid number
                // that we've checked halfway through, with a false-negative
                // at the end of that number
                if !working_number.is_valid {
                    working_number.is_valid = is_valid_number(x_pos, y_pos, &all_symbols);
                }

                // If we're at the end of the line, we need to store the number
                // if it's valid, as we won't get another chance to that at the
                // start of the next iteration
                if x_pos == line.len() - 1 {
                    if working_number.is_valid {
                        valid_numbers_in_line.push(
                            working_number
                                .value
                                .parse::<u32>()
                                .expect("Couldn't parse value"),
                        );
                    }
                }
            });

            return valid_numbers_in_line.iter().sum::<u32>();
        })
        .sum();

    let total_time = Instant::now() - start;

    println!("sum: {sum}");
    println!("Took {total_time:?}");
}

fn is_valid_number(x_pos: usize, y_pos: usize, all_symbols: &Vec<Vec<char>>) -> bool {
    if x_pos != 0 {
        // Directly to the left
        if is_symbol_char(all_symbols[y_pos][x_pos - 1]) {
            return true;
        }

        if y_pos != 0 {
            // Above to the left
            if is_symbol_char(all_symbols[y_pos - 1][x_pos - 1]) {
                return true;
            }
        }

        if y_pos != all_symbols.len() - 1 {
            // Below and to the left
            if is_symbol_char(all_symbols[y_pos + 1][x_pos - 1]) {
                return true;
            }
        }
    }

    if x_pos != all_symbols[0].len() - 1 {
        // Directly to the right
        if is_symbol_char(all_symbols[y_pos][x_pos + 1]) {
            return true;
        }

        if y_pos != 0 {
            // Above to the right
            if is_symbol_char(all_symbols[y_pos - 1][x_pos + 1]) {
                return true;
            }
        }

        if y_pos != all_symbols.len() - 1 {
            // Below and to the right
            if is_symbol_char(all_symbols[y_pos + 1][x_pos + 1]) {
                return true;
            }
        }
    }

    if y_pos != 0 {
        // Directly above
        if is_symbol_char(all_symbols[y_pos - 1][x_pos]) {
            return true;
        }
    }

    if y_pos != all_symbols.len() - 1 {
        // Directly below
        if is_symbol_char(all_symbols[y_pos + 1][x_pos]) {
            return true;
        }
    }

    return false;
}

fn is_symbol_char(char: char) -> bool {
    if char.is_digit(10) || char == '.' {
        return false;
    }

    return true;
}
