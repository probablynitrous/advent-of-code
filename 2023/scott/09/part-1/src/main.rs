use std::{fs, time::Instant};

fn main() {
    let file = fs::read_to_string("input.txt").expect("Couldn't find input file");

    let start_time = Instant::now();

    let histories = file.lines().map(|line| {
        line.split(" ")
            .map(|c| c.parse::<i32>().expect("Couldn't parse value"))
            .collect::<Vec<i32>>()
    });

    let sum: i32 = histories
        .map(|history| {
            let predicted_differences = predict_new_differences(&history);
            return history.last().unwrap() + predicted_differences.last().unwrap();
        })
        .sum();
    let total_time = Instant::now() - start_time;

    println!("sum: {}", sum);
    println!("Took: {:?}", total_time);
}

fn predict_new_differences(history: &Vec<i32>) -> Vec<i32> {
    let differences = get_current_differences(&history);

    if differences.iter().all(|v| v == &0) {
        let mut new_differences = differences.clone();
        new_differences.push(0);

        return new_differences;
    }

    let predicted_differences = predict_new_differences(&differences);

    let mut new_differences = differences.clone();
    new_differences.push(differences.last().unwrap() + predicted_differences.last().unwrap());

    return new_differences;
}

fn get_current_differences(history: &Vec<i32>) -> Vec<i32> {
    let mut differences = Vec::new();

    history.iter().enumerate().for_each(|(idx, value)| {
        if idx == history.len() - 1 {
            return;
        }

        differences.push(history.get(idx + 1).unwrap() - value);
    });

    return differences;
}
