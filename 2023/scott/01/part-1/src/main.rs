use std::fs;

fn main() {
    let file = fs::read_to_string("../input.txt").expect("Couldn't find file");
    let number_regex = regex::Regex::new(r"[0-9]").expect("Couldn't build regex");

    let lines = file.lines().collect::<Vec<&str>>();

    let value: u32 = lines
        .into_iter()
        .map(|line| {
            let first_digit = line
                .chars()
                .find(|char| number_regex.is_match(&char.to_string()));

            let last_digit = line
                .chars()
                .rev()
                .find(|char| number_regex.is_match(&char.to_string()));

            let mut combined_digits: String = "".to_string();
            combined_digits.push_str(&first_digit.unwrap_or('0').to_string());
            combined_digits.push_str(&last_digit.unwrap_or('0').to_string());

            return combined_digits
                .parse::<u32>()
                .expect("Couldn't parse digits");
        })
        .sum();

    println!("value: {value}");
}
