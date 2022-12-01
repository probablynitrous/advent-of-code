use std::fs;

fn main() {
    let max = fs::read_to_string("input.txt")
        .expect("Failed to read input")
        .split("\n\n")
        .map(|line| line.split("\n").flat_map(str::parse::<i64>).sum::<i64>())
        .max();

    println!("solution: {:?}", max.expect("Solution couldn't be found"));
}
