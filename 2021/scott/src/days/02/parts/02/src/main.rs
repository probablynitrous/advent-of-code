use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
    time::Instant,
};

mod direction;
mod instruction;
mod submarine;

use direction::Direction;
use instruction::Instruction;
use submarine::Submarine;

fn build_path(filename: &str) -> PathBuf {
    // Since we're reading from the build directory, we need to do some
    // footwork to get to the right directory
    let mut cwd = PathBuf::from(&std::env::current_exe().unwrap());

    // Step back three times so that we're at the root of the project
    cwd.pop();
    cwd.pop();
    cwd.pop();

    // Then add the file name so we can reference it
    cwd.push(filename);

    cwd
}

fn get_lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(build_path(filename)).expect("Could not find file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| {
            l.expect("Could not read line")
        })
        .collect()
}

fn main() {
    let lines = get_lines_from_file("input.txt");
    let now = Instant::now();

    let mut sub = Submarine::new();

    for line in &lines {
        let split = line.split(" ").collect::<Vec<&str>>(); 
        let instruction = Instruction::new(  
            Direction::from_str(split[0]).unwrap(),
            split[1].parse().expect("Valid integer"),
        );

        sub.travel(instruction);
    }

    println!("Distance: {}, Depth: {}", sub.get_distance(), sub.get_depth());

    println!("Solution: {}", sub.get_distance() * sub.get_depth());
    println!(
        "Ran in {}ms, ({}mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
}
