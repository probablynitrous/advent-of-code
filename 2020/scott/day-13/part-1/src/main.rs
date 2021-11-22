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
    let now = Instant::now();

    let timestamp: i32 = lines
        .get(0)
        .expect("valid index")
        .parse()
        .expect("Valid integer");

    let bus_routes: Vec<&str> = lines
        .get(1)
        .expect("Valid index")
        .split(",")
        .collect::<Vec<&str>>();

    let mut closest_time = -1;
    let mut stored_id = -1;

    for i in 0..bus_routes.len() {
        if bus_routes[i] == "x" {
            continue;
        }

        let bus_id = bus_routes[i].parse().expect("Valid integer");

        let closest_factor = get_closest_factor(bus_id, timestamp);
        if closest_time == -1 || closest_time > closest_factor {
            closest_time = closest_factor;
            stored_id = bus_id;
        }
    }

    println!("Solution: {}", (closest_time - timestamp) * stored_id);

    println!(
        "Ran in {}ms ({} mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
}

fn get_closest_factor(interval: i32, timestamp: i32) -> i32 {
    let mut closest_factor = -1;
    let mut inc = timestamp;

    while closest_factor == -1 {
        if inc % interval == 0 {
            closest_factor = inc;
        } else {
            inc += 1;
        }
    }

    return inc;
}
