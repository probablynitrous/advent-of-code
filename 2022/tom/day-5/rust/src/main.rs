use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::env;
use std::time::Instant;
use regex::Regex;

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

fn step1(data: &Vec<String>) -> String {
	let (mut stacks,break_line) = get_initial_stack(data);

	let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

	for line in &data[break_line+1..data.len()] {

		let caps = re.captures(line).unwrap();

		let move_from:usize = caps[2].parse().unwrap();
		let move_to:usize = caps[3].parse().unwrap();
		let move_amount:usize = caps[1].parse().unwrap();
		let mut moved_list: Vec<String> = Vec::new();

		if move_amount == stacks[move_from-1].len() {
			moved_list = stacks[move_from-1].to_vec();
		} else {
			moved_list = stacks[move_from-1][0..move_amount].to_vec();
		}
			for moved in moved_list {
				let moved_arry = &[moved];
				stacks[move_to - 1].splice(0..0,moved_arry.iter().cloned());
				stacks[move_from - 1] = stacks[move_from-1][1..stacks[move_from-1].len()].to_vec();
			}	
		}
		let mut top_stack: String = String::new();

		for stack in &stacks {
			if stack.len() > 0 {
				top_stack += &stack[0];
			}
	}
	return top_stack.to_string();
}

fn step2(data: &Vec<String>) -> String {
	let (mut stacks,break_line) = get_initial_stack(data);

	let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

	for line in &data[break_line+1..data.len()] {

		let caps = re.captures(line).unwrap();

		let move_from:usize = caps[2].parse().unwrap();
		let move_to:usize = caps[3].parse().unwrap();
		let move_amount:usize = caps[1].parse().unwrap();
		let mut moved_list: Vec<String> = Vec::new();

		if move_amount == stacks[move_from-1].len() {
			moved_list = stacks[move_from-1].to_vec();
		} else {
			moved_list = stacks[move_from-1][0..move_amount].to_vec();
		}
			stacks[move_from-1] = stacks[move_from-1][moved_list.len()..stacks[move_from-1].len()].to_vec();
			stacks[move_to - 1].splice(0..0,moved_list.iter().cloned());
		}
		let mut top_stack: String = String::new();

		for stack in &stacks {
			if stack.len() > 0 {
				top_stack += &stack[0];
			}
	}
	return top_stack.to_string();
}

fn get_initial_stack(data:&Vec<String>) -> (Vec<Vec<String>>,usize) {
	let mut initial_stack_lines: Vec<String> = Vec::new();
	let mut break_line: usize = 0;
	for (i,line) in data.iter().enumerate() {
		if line == "" {
			break_line = i;
			break;
		}
		initial_stack_lines.push(line.to_string());
	}
	let stack_count: usize = initial_stack_lines.len();
	let mut stacks : Vec<Vec<String>> = Vec::new();
	
	for _i in 0..stack_count {
		stacks.push(vec![]);
	}
	

	for stack_row in &initial_stack_lines[0..stack_count-1] {
		for (i,c) in stack_row.chars().enumerate() {
			if !c.is_alphabetic() {
				continue;
			}
			let stack = ((i + 3) / 4) - 1;
			let c_array = &[moved];
			stacks[stack].push(c.to_string());
		}
	}

	return (stacks,break_line);
}
