use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
    time::Instant,
};

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

    let mut positions = lines.get(0).unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    positions.sort_unstable();

    let cheapest_fuel = *positions.get(positions.len() / 2).unwrap();
    let mut total_fuel_usage = 0;

    for position in positions {
        total_fuel_usage += (cheapest_fuel - position).abs();
    }

    println!("Solution: {}", total_fuel_usage);
    println!(
        "Ran in {}ms, ({}mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
}
