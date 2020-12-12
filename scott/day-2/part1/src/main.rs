use regex::Regex;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    time::Instant,
};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let file = BufReader::new(file);

    let now = Instant::now();

    let mut valid_passwords: Vec<String> = Vec::new();

    for l in file.lines() {
        let line = l.unwrap();
        let v: Vec<&str> = line.split(":").collect();
        let criteria: Vec<&str> = v[0].split(" ").collect();
        let password = v[1].trim();
        let letter = criteria[1].to_owned();
        let range: Vec<&str> = criteria[0].split("-").collect();
        let (min, max) = (range[0], range[1]);

        let regex_string = format!(
            "^([^{}]*{}[^{}]*){{{},{}}}$",
            letter, letter, letter, min, max
        );

        let re: Regex = Regex::new(&regex_string).expect("Failed to initialise regex pattern");

        if re.is_match(password) {
            valid_passwords.push(password.to_owned());
        }
    }

    println!("Ran in {}ms", now.elapsed().as_millis());

    println!("solution: {} valid passwords", valid_passwords.len());

    Ok(())
}
