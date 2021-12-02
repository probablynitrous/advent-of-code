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

fn get_lines_from_file(filename: &str) -> Vec<u64> {
    let file = File::open(build_path(filename)).expect("Could not find file.");
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
    let lines = get_lines_from_file("test.txt");
    let now = Instant::now();

    let mut previous = lines.get(0).unwrap().to_owned();
    let mut count: u64 = 0;

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
