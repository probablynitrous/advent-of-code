use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::env;
use std::time::Instant;

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

	
	let mut now = Instant::now();
	let step1 = step1(&data);
	let mut elapsed = now.elapsed();

	println!("Step 1: {}", step1);
	println!("Step 1 took: {:?}", elapsed);

	now = Instant::now();
	let step2 = step2(&data);
	elapsed = now.elapsed();

	println!("Step 2: {}", step2);
	println!("Step 2 took: {:?}", elapsed);
}

fn step1(data: &Vec<String>) -> i64 {
	let mut priority_sum:i64 = 0;
	for line in data {
		let (compartment1,compartment2) = line.split_at(line.chars().count()/2);

		for c in compartment1.chars() {
			if compartment2.contains(c) {	
				priority_sum += get_priority(c);
				break;
			}
		}
	}
	return priority_sum;
}

fn step2(data: &Vec<String>) -> i64 {
	let now = Instant::now();
	let mut priority_sum:i64 = 0;
	let mut groups: Vec<Vec<String>> = Vec::new();
	let mut counter:usize = 0;

	//Group data
	for (i,_) in data.iter().enumerate() {
		if counter == 2 {
			groups.push(get_group(i,&data));
			counter = 0;
		} else {
			counter += 1;
		}
	}

	//Compare values in groups
	for group in groups {
		for c in group[0].chars() {
			if group[1].contains(c) {	
				if group[2].contains(c) {	
					priority_sum += get_priority(c);	
					break;
				}
			}
		}
	}  

	return	priority_sum;
}

fn get_priority(c:char) -> i64 {
	if c.is_lowercase() {
		return c as i64 - 96;
	} else {
		return c as i64 - 38;
	}
}

fn get_group(i:usize, data: &Vec<String>) -> Vec<String> {
	return data[i-2..i+1].to_vec()
}