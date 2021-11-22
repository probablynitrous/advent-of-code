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

struct Airplane {
    seat_ids: Vec<f64>,
}

struct BoardingPass {
    indicated_row: String,
    row_value: f64,
    indicated_column: String,
    column_value: f64,
    seat_id: f64,
}

fn main() {
    let lines = get_lines_from_file("../input.txt");
    let mut highest_seat_id: f64 = -1.0;

    let mut airplane = Airplane { seat_ids: vec![] };

    for line in lines {
        let mut boarding_pass = BoardingPass {
            indicated_row: line[..7].to_owned(),
            row_value: 1.0,
            indicated_column: line[7..].to_owned(),
            column_value: 1.0,
            seat_id: 1.0,
        };

        let mut left: f64 = 0.;
        let mut right: f64 = 127.;

        for direction in boarding_pass.indicated_row.chars() {
            boarding_pass.row_value = round::floor((right + left) / 2.0, 0);
            if String::from(direction) == "B" {
                left = boarding_pass.row_value + 1.0;
            } else if String::from(direction) == "F" {
                right = boarding_pass.row_value;
            }
        }

        boarding_pass.row_value = round::floor((right + left) / 2.0, 0);

        let mut indexes: Vec<f64> = vec![0.0, 7.0];
        for direction in boarding_pass.indicated_column.chars() {
            if String::from(direction) == "R" {
                indexes[0] = round::floor((indexes[1] + indexes[0]) / 2.0, 0) + 1.0;
            } else if String::from(direction) == "L" {
                indexes[1] = round::floor((indexes[1] + indexes[0]) / 2.0, 0) - 1.0;
            }
        }

        boarding_pass.column_value = indexes[0];

        boarding_pass.seat_id = boarding_pass.row_value * 8.0 + boarding_pass.column_value;

        airplane.seat_ids.push(boarding_pass.seat_id);
    }

    airplane.seat_ids.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut my_seat_id: f64 = -1.0;

    for (index, seat_id) in airplane.seat_ids.iter().enumerate() {
        if index != airplane.seat_ids.len() - 1 {
            let next_id = airplane
                .seat_ids
                .get(index + 1)
                .expect("Could not get next id")
                .to_owned();

            if next_id != seat_id + 1.0 {
                my_seat_id = seat_id.clone() + 1.0;
            }
        }
    }

    println!("My seat id: {}", my_seat_id);
}
