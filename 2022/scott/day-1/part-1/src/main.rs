use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn get_lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Could not find file.");
    let buf = BufReader::new(file);

    buf.lines().map(|line| line.unwrap()).collect()
}

fn main() {
    let lines = get_lines_from_file("input.txt");

    let mut largest = -1;
    let mut current = 0;

    lines.iter().for_each(|line| {
        if line.len() == 0 {
            if largest < current {
                largest = current;
            }

            current = 0;
            return;
        }

        current += line.parse::<i64>().expect("Expected to parse integer");
    });
    println!("largest: {:?}", largest);
}
