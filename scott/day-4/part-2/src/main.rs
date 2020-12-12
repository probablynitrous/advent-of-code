use regex::Regex;
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

    let valid_eye_color = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

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
            for param in &line {
                println!("line: {:?}", line);
                let key_value: Vec<&str> = param.split(":").collect::<Vec<&str>>();
                println!("key_value: {:?}", key_value);
                let key = key_value[0];
                let value = key_value[1];

                // Could use a `match` satement here but I wasn't going to entertain the battle
                // with types and exhaustive checking.
                if key == "byr" {
                    let year: i32 = value.parse().unwrap();
                    if year < 1920 || year > 2002 {
                        is_valid_passport = false;
                    }
                }

                if key == "iyr" {
                    println!("value: {}", value);
                    let year: i32 = value.parse().unwrap();
                    if year < 2010 || year > 2020 {
                        is_valid_passport = false;
                    }
                }

                if key == "eyr" {
                    let year: i32 = value.parse().unwrap();
                    if year < 2020 || year > 2030 {
                        is_valid_passport = false;
                    }
                }

                if key == "hgt" {
                    let unit: &str = &value[..value.len() - 2];
                    if unit == "cm" {
                        let measurement: &i32 = &value[0..value.len() - 2].parse().unwrap();
                        if measurement < &150 as &i32 || measurement > &193 as &i32 {
                            is_valid_passport = false;
                        }
                    } else if unit == "in" {
                        let measurement: &i32 = &value[0..value.len() - 2].parse().unwrap();
                        if measurement < &59 as &i32 || measurement > &76 as &i32 {
                            is_valid_passport = false;
                        }
                    } else {
                        is_valid_passport = false;
                    }
                }

                if key == "hcl" {
                    if String::from(value.chars().collect::<Vec<char>>()[0]) != "#" {
                        is_valid_passport = false;
                    }

                    if value.len() != 7 {
                        is_valid_passport = false;
                    }
                }

                if key == "ecl" {
                    if !valid_eye_color
                        .clone()
                        .into_iter()
                        .any(|color| value == color)
                    {
                        is_valid_passport = false;
                    }
                }

                if key == "pid" {
                    let re: Regex =
                        Regex::new(r"^(\d){9}$").expect("Failed to initialise regex pattern");
                    if !re.is_match(value) {
                        is_valid_passport = false;
                    }
                }
            }
        }

        if is_valid_passport {
            valid_passports.push(line);
        }
    }

    println!("Valid passports: {}", valid_passports.len());
}
