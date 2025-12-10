use std::{str::FromStr, time::Instant};

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Multiply,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Op::Multiply),
            "+" => Ok(Op::Add),
            _ => Err(String::from("Unhandled operation")),
        }
    }
}

#[derive(Clone, Debug)]
struct Calculation {
    starting_index: usize,
    column_count: usize,
    operation: Op,
}

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    let start = Instant::now();

    let mut calculations: Vec<Calculation> = Vec::new();
    let num_lines = file.lines().collect::<Vec<&str>>().len();

    // Only loop over the last line, since the operators denote the sizes of the columns
    for line in file.lines().skip(num_lines - 1) {
        let characters = line.chars().collect::<Vec<char>>();

        for (i, character) in characters.iter().enumerate() {
            if let Ok(operation) = Op::from_str(character.to_string().as_str()) {
                let next_op = characters
                    .iter()
                    .skip(i + 1)
                    .position(|c| *c == '*' || *c == '+');

                calculations.push(Calculation {
                    starting_index: i,
                    column_count: if next_op.is_some() {
                        next_op.unwrap()
                    } else {
                        characters.len() - i
                    },
                    operation,
                });
            }
        }
    }

    let grand_total = calculations
        .iter()
        .map(|calculation| {
            let digits = (calculation.starting_index
                ..calculation.starting_index + calculation.column_count)
                .map(|i| {
                    file.lines()
                        .take(num_lines - 1)
                        .map(|line| line.chars().nth(i).unwrap().to_string())
                        .collect::<Vec<String>>()
                        .join("")
                        .trim()
                        .parse::<u64>()
                        .unwrap()
                })
                .collect::<Vec<u64>>();

            match calculation.operation {
                Op::Add => digits.iter().sum::<u64>(),
                Op::Multiply => digits.iter().product::<u64>(),
            }
        })
        .sum::<u64>();

    let total_time = Instant::now() - start;

    println!("Grand total: {}", grand_total);
    println!("Took {:?}", total_time);
}
