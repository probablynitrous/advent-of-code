use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::Instant,
};

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
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
    let mut lines = get_lines_from_file("../input.txt");
    let mut one_gap: Vec<i32> = vec![];
    let mut three_gap: Vec<i32> = vec![];

    let now = Instant::now();
    lines.sort();

    for (index, line) in lines.iter().enumerate() {
        if index == lines.len() - 1 {
            // If we're at the end then push our device
            three_gap.push(line + 3);
            break;
        }

        // The first index
        if index == 0 {
            if line - 0 == 1 {
                one_gap.push(line.to_owned());
            } else if line - 0 == 3 {
                three_gap.push(line.to_owned());
            }
        }

        if lines[index + 1] - line == 3 {
            three_gap.push(line.to_owned());
        } else if lines[index + 1] - line == 1 {
            one_gap.push(line.to_owned());
        }
    }

    println!("Solution: {}", one_gap.len() * three_gap.len());
    println!(
        "Ran in {}ms, ({}mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
}
