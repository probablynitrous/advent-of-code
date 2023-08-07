use std::{fs, path::Path};

const FILE_NAME: &str = "../input.txt";

fn parse_passports(file: String) -> Vec<String> {
    let mut passports: Vec<String> = Vec::new();
    let mut working_passport = Vec::new();

    file.lines().for_each(|f| {
        if f.len() > 0 {
            working_passport.push(f)
        } else {
            passports.push(working_passport.join(" "));
            working_passport = Vec::new();
        }
    });

    passports
}

fn is_valid_passport(passport: &String) -> bool {
    let mut is_valid = true;
    let required_fields = Vec::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]);

    let data_lines = passport.split(" ");
    let fields: Vec<&str> = data_lines
        .map(|f| {
            f.split(":")
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .to_owned()
        })
        .collect();

    required_fields.into_iter().for_each(|required_field| {
        if required_field != "cid" {
            if !fields.contains(&required_field) {
                is_valid = false;
            }
        }
    });

    return is_valid;
}

// A chance that we might need to build this with OO in the next part, keeping
// it functional for now...
fn main() {
    let file = fs::read_to_string(FILE_NAME).expect("Failed to read input");
    let passports = parse_passports(file);

    let valid_passports: Vec<String> = passports
        .into_iter()
        .filter(|p| is_valid_passport(p))
        .collect();

    println!("Valid passports {}", valid_passports.len());
}
