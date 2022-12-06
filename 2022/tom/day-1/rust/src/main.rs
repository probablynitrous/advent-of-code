use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::env;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data;
    let test_flag: String = "test".to_string();
    if args.contains(&test_flag) {
        data = lines_from_file("../test.txt");
    } else {
        data = lines_from_file("../input.txt");
    }

    let step1 = step1(&data);

    println!("Step 1: {}", step1);
    let step2= step2(&data);

    println!("Step 2: {}", step2);
}

fn step1(data: &Vec<String>) -> i64 {
	let mut top_calories = 0;
	let mut current_calories = 0;
	for line in data {
		if line == "" {
			if top_calories < current_calories {
				top_calories = current_calories;
			}
			current_calories = 0;
		} else {
			let calories: i64 = line.parse().unwrap();
			current_calories += calories	;
		}
	
	} 
	
	if top_calories < current_calories {
		top_calories = current_calories;
	}
	return top_calories;
}

fn step2(data: &Vec<String>) -> i64 {
	let mut top_calories = vec![0,0,0];
	let mut current_calories = 0;
	for line in data {
		if line == "" {
			top_calories = update_top_calories(top_calories,current_calories);
			current_calories = 0;
		} else {
			let calories: i64 = line.parse().unwrap();
			current_calories += calories	;
		}
	}
	
	top_calories = update_top_calories(top_calories,current_calories);

	let mut total = 0;
    for calories in top_calories {
		total += calories;
	}
	return total;
}

fn update_top_calories(mut top_calories: Vec<i64>,current_calories: i64) -> Vec<i64>{
    top_calories.sort_by(|a, b| b.cmp(a));
    if current_calories > top_calories[2] {
        top_calories.pop();
        top_calories.push(current_calories)
    }
    return top_calories;
  }