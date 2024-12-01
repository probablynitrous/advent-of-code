use std::collections::HashMap;

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");

    let (left, right): (Vec<i32>, Vec<i32>) = file
        .trim()
        .lines()
        .map(|line| {
            let splits = line.split("   ").collect::<Vec<&str>>();
            (
                splits.clone().get(0).unwrap().parse::<i32>().unwrap(),
                splits.clone().get(1).unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    let mut occurrence_count: HashMap<&i32, i32> = HashMap::new();

    left.iter().for_each(|value| {
        if occurrence_count.contains_key(value) {
            return;
        }

        let occurrences = right
            .iter()
            .filter(|right_value| *right_value == value)
            .collect::<Vec<&i32>>()
            .len();

        occurrence_count.insert(value, occurrences as i32);
    });

    let similarity_score: i32 = occurrence_count
        .iter()
        .map(|(key, value)| value * *key)
        .sum();

    println!("similarity score: {}", similarity_score);
}
