use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::Instant,
};

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Could not find file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line."))
        .collect()
}

fn main() {
    let lines = get_lines_from_file("../input.txt");
    let preamble_length = 25;
    let mut solution: i32 = 0;
    let mut has_solution = false;
    let mut idx = preamble_length;

    let now = Instant::now();
    while !has_solution {
        let line = lines.get(idx).expect("Valid index");
        let target: i32 = line.parse().expect("Valid integer");

        if !is_sum_of_previous(&lines[idx - preamble_length..idx].to_vec(), target) {
            solution = target;
            has_solution = true;
        }

        idx += 1;
    }

    println!(
        "Ran in {}ms ({} mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
    println!("Solution: {}", solution);
}

fn is_sum_of_previous(numbers: &Vec<String>, target: i32) -> bool {
    let mut is_valid_target = false;
    let mut index = 0;

    while index < numbers.len() && !is_valid_target {
        let mut inner_index = 1;
        while inner_index < numbers.len() - 1 && !is_valid_target {
            let num1: i32 = numbers
                .get(index)
                .expect("Valid index")
                .parse()
                .expect("valid integer");
            let num2: i32 = numbers
                .get(inner_index)
                .expect("Valid index")
                .parse()
                .expect("Valid integer");

            if num1 + num2 == target {
                is_valid_target = true;
            }

            inner_index += 1;
        }

        index += 1;
    }

    return is_valid_target;
}
