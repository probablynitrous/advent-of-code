use math::round;
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

struct Route {
    value: i64,
    position: i64,
}

fn main() {
    let lines = get_lines_from_file("../input.txt");
    let now = Instant::now();

    let bus_routes: Vec<&str> = lines
        .get(1)
        .expect("Valid index")
        .split(",")
        .collect::<Vec<&str>>();

    let mut parsed_routes: Vec<(i64, i64)> = bus_routes
        .iter()
        .enumerate()
        .filter_map(|(i, l)| l.parse().ok().map(|l| (l - i as i64, l)))
        .collect();

    let prod: i64 = parsed_routes.iter().map(|n| n.1).product::<i64>();

    let ecd_solution: i64 = parsed_routes
        .iter()
        .map(|(pr, route)| {
            let partial = prod / route;
            pr * ((extended_euclidean(partial, route.to_owned()).1 % route + route) % route)
                * partial
        })
        .sum::<i64>();

    println!("solution: {}", ecd_solution % prod);
    println!(
        "Ran in {}ms ({} mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
}

fn extended_euclidean(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    } else {
        let (g, x, y) = extended_euclidean(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
