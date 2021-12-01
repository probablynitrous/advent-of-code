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
    let lines = get_lines_from_file("./test.txt");
    let now = Instant::now();

    let mut previous = lines.get(0).unwrap().to_owned();
    let mut count = 0;


    for line in lines {
        if line > previous {
            count += 1;
        }

        previous = line;
    }

    println!("Solution: {}", count);
    println!(
        "Ran in {}ms, ({}mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
}
