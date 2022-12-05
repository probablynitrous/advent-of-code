use std::{collections::HashMap, fs};

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn get_duplicate_from_bag(bag: &str) -> Option<char> {
    let mut first_half = bag[0..bag.len() / 2].chars().collect::<Vec<char>>();
    let mut second_half = bag[bag.len() / 2..].chars().collect::<Vec<char>>();

    first_half.sort();
    second_half.sort();

    return first_half.into_iter().find(|char| {
        return second_half.contains(char);
    });
}

fn main() {
    let alphabet: HashMap<char, i64> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect::<Vec<char>>()
        .into_iter()
        .enumerate()
        .map(|(i, char)| (char, i as i64))
        .collect();

    let file = get_file_as_string("input.txt");

    let sum: i64 = file
        .split("\n")
        .map(get_duplicate_from_bag)
        .filter(|p| p.is_some())
        .map(|duplicate| alphabet.get(&duplicate.unwrap()).unwrap() + 1)
        .sum();

    println!("sum: {:?}", sum);
}
