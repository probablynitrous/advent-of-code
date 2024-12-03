use core::panic;
use regex::Regex;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Failed to read file");

    let operator_re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))")
        .expect("Failed to parse operator regex");
    let values_re = Regex::new(r"\d{1,3}").expect("Failed to parse values regex");

    let mut should_operate = true;

    let memory: i32 = operator_re
        .captures_iter(&file)
        .map(|capture| {
            if capture.get(2).is_some() {
                should_operate = true;
                return 0;
            }

            if capture.get(3).is_some() {
                should_operate = false;
                return 0;
            }

            if !should_operate {
                return 0;
            }

            if capture.get(1).is_none() {
                panic!("Failed to get values for multiplication");
            }

            values_re
                .captures_iter(capture.get(1).unwrap().into())
                .map(|value| {
                    let extraction: &str = value
                        .get(0)
                        .expect("Didn't get match for multiplicator")
                        .into();
                    extraction
                        .parse::<i32>()
                        .expect("Failed to parse multiplication value")
                })
                .product::<i32>()
        })
        .sum();

    println!("Memory: {memory}");
}
