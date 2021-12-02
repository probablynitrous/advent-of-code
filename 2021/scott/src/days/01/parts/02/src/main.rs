use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::Instant,
};

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<u16> {
    let file = File::open(filename).expect("Could not find file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| {
            l.expect("Could not read line")
                .parse()
                .expect("Valid integer")
        })
        .collect()
}

fn main() {
    let lines = get_lines_from_file("./test.txt");
    let now = Instant::now();

    let mut count: u16 = 0;
    let mut previous: u16 = 0;

    for (idx, line) in lines.iter().enumerate() {
        if idx < (lines.len() - 2) {
            let sum = line.clone()
                + lines.get(idx + 1).unwrap().clone()
                + lines.get(idx + 2).unwrap().clone();

            if previous < sum {
                count += 1;
            }

            previous = sum;
        }
    }

    println!("Solution: {}", count - 1);
    println!(
        "Ran in {}ms, ({}mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
}
