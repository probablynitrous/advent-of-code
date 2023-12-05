use std::{fs, intrinsics::raw_eq, time::Instant};

// TODO: Might not need this yet
enum Symbol {
    Star,
    Hash,
    Plus,
    Dollar,
    Slash,
}

struct PartNumber {
    value: String,
    start_index: Option<usize>,
    is_valid: bool,
}

impl From<&str> for Symbol {
    fn from(symbol_text: &str) -> Self {
        return match symbol_text {
            "*" => Self::Star,
            "#" => Self::Hash,
            "+" => Self::Plus,
            "$" => Self::Dollar,
            "/" => Self::Slash,
            _ => todo!(),
        };
    }
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
            let mut cog_values: Vec<u32> = Vec::new();

            line.chars().enumerate().for_each(|(x_pos, char)| {
                if char != '*' {
                    return;
                }

                let adjacent_numbers = find_adjacent_numbers(x_pos, y_pos, &all_symbols);

                cog_values.push(adjacent_numbers[0] * adjacent_numbers[1]);
            });

            return cog_values.iter().sum::<u32>();
        })
        .sum();

    let total_time = Instant::now() - start;

    println!("sum: {sum}");
    println!("Took {total_time:?}");
}

// Verify that there are only ever two numbers surrounding a cog
fn find_adjacent_numbers(x_pos: usize, y_pos: usize, all_symbols: &Vec<Vec<char>>) -> Vec<u32> {
    // Top left, top middle, top right
    let mut top_values: Vec<bool> = Vec::new();

    // Immediate left and immediate right
    let mut middle_values: Vec<bool> = Vec::new();

    // Bottom left, bottom middle, bottom right
    let mut bottom_values: Vec<bool> = Vec::new();

    // Check along the top (if we can)
    if y_pos != 0 {
        if x_pos != 0 {
            // Top left
            top_values.push(all_symbols[y_pos - 1][x_pos - 1].is_digit(10));
        }

        // Top middle
        top_values.push(all_symbols[y_pos - 1][x_pos].is_digit(10));

        if x_pos != all_symbols[0].len() - 1 {
            // Top right
            top_values.push(all_symbols[y_pos - 1][x_pos + 1].is_digit(10));
        }
    }

    // Check the sides (if we can)
    if x_pos != 0 {
        // Left
        middle_values.push(all_symbols[y_pos][x_pos - 1].is_digit(10));
    }

    if x_pos != all_symbols[0].len() - 1 {
        // Right
        middle_values.push(all_symbols[y_pos][x_pos + 1].is_digit(10));
    }

    // Check the bottom (if we can)
    if y_pos != all_symbols.len() - 1 {
        if x_pos != 0 {
            // Bottom left
            bottom_values.push(all_symbols[y_pos + 1][x_pos - 1].is_digit(10));
        }

        // Bottom middle
        bottom_values.push(all_symbols[y_pos + 1][x_pos].is_digit(10));

        if x_pos != all_symbols[0].len() - 1 {
            // Bottom right
            bottom_values.push(all_symbols[y_pos + 1][x_pos + 1].is_digit(10));
        }
    }

    // We now know whether we have values around us, so we need to ensure we
    // only have two numbers adjacent

    return Vec::new();
}
