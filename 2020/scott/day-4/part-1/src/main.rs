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
    let mut usable_lines: Vec<Vec<String>> = vec![];
    let mut use_line = false;
    let mut temp_vec: Vec<String> = vec![];
    let mut valid_passports: Vec<Vec<String>> = vec![];

    for line in lines {
        if line.len() == 0 {
            use_line = false;
        } else {
            use_line = true;
        }

        if use_line == true {
            temp_vec.push(line);
        } else {
            usable_lines.push(temp_vec);
            temp_vec = vec![];
        }
    }

    let mut required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_fields.sort();
    let mut with_optional_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    with_optional_fields.sort();

    // Because we need to add the last one
    usable_lines.push(temp_vec);

    for line in usable_lines {
        let concatenated_line = line.join(" ");
        let passport_params: Vec<&str> = concatenated_line.split(" ").collect();
        let mut temp_vec2: Vec<&str> = vec![];
        let mut is_valid_passport = false;
        &passport_params.iter().for_each(|p| {
            temp_vec2.push(p.split(":").collect::<Vec<&str>>()[0]);
        });

        temp_vec2.sort();

        if temp_vec2 == required_fields || temp_vec2 == with_optional_fields {
            is_valid_passport = true;
        }

        if is_valid_passport {
            valid_passports.push(line);
        }
    }

    println!("Valid passports: {}", valid_passports.len());
}
