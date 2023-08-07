use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::Instant,
};

mod line;
mod toboggan;

use line::*;
use toboggan::*;

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<Line> {
    let file = File::open(filename).expect("Could not find file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| Line::new(l.expect("Could not parse line.")))
        .collect()
}

fn main() -> std::io::Result<()> {
    let lines = get_lines_from_file("../input.txt");

    let mut toboggan = Toboggan::new(0, 3, 0);

    let now = Instant::now();
    for line in lines {
        let current_tile = line.get_tile_at(toboggan.get_current_pos());

        if toboggan.will_hit_tree(current_tile) {
            toboggan.has_hit_tree();
        }

        toboggan.navigate(&line);
    }

    println!(
        "Ran in {}ms ({}ns)",
        now.elapsed().as_millis(),
        now.elapsed().as_nanos()
    );
    println!("Hit {} trees", toboggan.get_tree_count());

    Ok(())
}
