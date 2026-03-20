use std::time::Instant;

#[derive(Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    let start = Instant::now();

    let points = file
        .lines()
        .map(|line| {
            let split = line.split(",").collect::<Vec<&str>>();
            return Point {
                x: split.first().unwrap().parse().unwrap(),
                y: split.iter().nth(1).unwrap().parse().unwrap(),
            };
        })
        .collect::<Vec<Point>>();

    let mut max = -1;

    for p1 in points.iter() {
        for p2 in points.iter() {
            if p2.eq(p1) {
                continue;
            }
            let product = ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1);
            if product > max {
                max = product
            }
        }
    }

    let total_time = Instant::now() - start;

    println!("max: {}", max);
    println!("Took {:?}", total_time);
}
