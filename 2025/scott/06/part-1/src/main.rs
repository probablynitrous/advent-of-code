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
    digits: Vec<u64>,
    operation: Option<Op>,
}

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    let start = Instant::now();

    let mut calculations: Vec<Calculation> = Vec::new();

    file.lines().for_each(|line| {
        let values = line.split_whitespace().collect::<Vec<&str>>();
        if values.iter().all(|c| c.chars().all(|c| c.is_digit(10))) {
            for (index, digit) in values
                .iter()
                .take_while(|v| !v.is_empty())
                .enumerate()
                .map(|(i, c)| (i, c.parse::<u64>().unwrap()))
            {
                if let Some(calculation) = calculations.get_mut(index) {
                    calculation.digits.push(digit);
                } else {
                    calculations.push(Calculation {
                        digits: vec![digit],
                        operation: None,
                    });
                }
            }
        } else {
            for (index, operation) in values.iter().take_while(|v| !v.is_empty()).enumerate() {
                if let Some(calculation) = calculations.get_mut(index) {
                    calculation.operation = Op::from_str(*operation).ok();
                } else {
                    calculations.push(Calculation {
                        digits: Vec::new(),
                        operation: Op::from_str(*operation).ok(),
                    });
                }
            }
        }
    });

    let grand_total = calculations
        .iter()
        .map(|calculation| {
            match calculation
                .operation
                .expect("Reached computation without operation")
            {
                Op::Multiply => calculation.digits.iter().product::<u64>(),
                Op::Add => calculation.digits.iter().sum::<u64>(),
            }
        })
        .sum::<u64>();

    let total_time = Instant::now() - start;

    println!("Grand total: {}", grand_total);
    println!("Took {:?}", total_time);
}
