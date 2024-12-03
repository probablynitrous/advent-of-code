use regex::Regex;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Failed to read file");

    let operator_re =
        Regex::new(r"mul\(\d{1,3},\d{1,3}\)").expect("Failed to parse operator regex");
    let values_re = Regex::new(r"\d{1,3}").expect("Failed to parse values regex");

    let memory: i32 = operator_re
        .captures_iter(&file)
        .map(|capture| {
            let extraction = capture.get(0).expect("Didn't find match for operator");
            values_re
                .captures_iter(extraction.into())
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
