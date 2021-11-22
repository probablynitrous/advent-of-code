use regex::Regex;
use std::time::Instant;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let file = BufReader::new(file);

    let now = Instant::now();

    let mut valid_passwords: Vec<String> = Vec::new();

    for l in file.lines() {
        let line = l.unwrap();

        let re = Regex::new("^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)$")
            .expect("Failed to initialise regex pattern");

        let captures = re.captures(&line).unwrap();

        let min = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let max = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

        let character = captures.get(3).unwrap().as_str();

        let password = captures.get(4).unwrap().as_str();

        if valid(min, max, character, password) {
            valid_passwords.push(password.to_owned());
        }
    }

    println!("Ran in {}ms", now.elapsed().as_millis());

    println!("There are {} valid passwords", valid_passwords.len());

    Ok(())
}

fn valid(min: usize, max: usize, character: &str, password: &str) -> bool {
    let count = password.matches(character).count();
    if count < min {
        return false;
    }
    if max < count {
        return false;
    }
    return true;
}
