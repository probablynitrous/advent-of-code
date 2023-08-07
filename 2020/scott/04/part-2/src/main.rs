mod constrained_value;
mod passport;

use crate::passport::Passport;
use std::fs;

const FILE_NAME: &str = "../input.txt";

fn parse_passports(file: String) -> Vec<bool> {
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

    return passports
        .into_iter()
        .map(|p| Passport::is_valid(p))
        .collect::<Vec<bool>>();
}

fn main() {
    let file = fs::read_to_string(FILE_NAME).expect("Failed to read input");
    let passports = parse_passports(file);

    let valid_passports: Vec<Option<Passport>> =
        passports.into_iter().filter(|p| p == true).collect();

    println!("Valid passports {}", valid_passports.len());
}
