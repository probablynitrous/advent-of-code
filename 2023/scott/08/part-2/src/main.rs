use std::{collections::HashMap, fs, time::Instant};

enum Instruction {
    Left,
    Right,
}

impl Instruction {
    pub fn from(value: &str) -> Option<Self> {
        match value {
            "R" => Some(Self::Right),
            "L" => Some(Self::Left),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Path<'a> {
    start: &'a str,
    left: &'a str,
    right: &'a str,
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Couldn't find file");
    let start = Instant::now();

    // The order of directions we need to take
    let instruction_set = file
        .lines()
        .take(1)
        .collect::<Vec<&str>>()
        .get(0)
        .expect("Couldn't find instruction set at the start of the file")
        .split("")
        .filter_map(|c| Instruction::from(c))
        .collect::<Vec<Instruction>>();

    // Get all the paths in an appropriate place
    let paths = get_paths(&file);

    // Get all the starting paths, since we can be on more than one at this point
    let mut working_paths: Vec<&Path> = Vec::new();

    for (key, value) in paths.iter() {
        if key.chars().last().unwrap() == 'A' {
            working_paths.push(value);
        };
    }

    let mut steps = 0;
    let mut ending_paths_count: i64 = 0;

    // Continue to step through until we hit Z in all the working_paths
    while ending_paths_count != working_paths.len() as i64 {
        steps += 1;

        let instruction = instruction_set
            .get((steps - 1) % instruction_set.len())
            .expect("Couldn't get instruction for step: {step}");

        working_paths.iter_mut().for_each(|path| {
            // Need to get the right destination based on the instruction_set
            match instruction {
                Instruction::Right => {
                    *path = paths
                        .get(path.right)
                        .expect("Couldn't find path to the right");

                    if path.start.chars().last().unwrap() == 'Z' {
                        ending_paths_count += 1;
                    } else if ending_paths_count > 0 {
                        ending_paths_count -= 1;
                    }
                }
                Instruction::Left => {
                    *path = paths
                        .get(path.left)
                        .expect("Couldn't find a path to the left");

                    if path.start.chars().last().unwrap() == 'Z' {
                        ending_paths_count += 1;
                    } else if ending_paths_count > 0 {
                        ending_paths_count -= 1;
                    }
                }
            }
        });
    }

    let total_time = Instant::now() - start;
    println!("steps: {:?}", steps);
    println!("took: {:?}", total_time);
}

fn get_paths(file: &str) -> HashMap<String, Path> {
    let mut paths: HashMap<String, Path> = HashMap::new();

    file.lines().skip(2).for_each(|line| {
        let parts = line.split(" = ").collect::<Vec<&str>>();

        let start = parts.get(0).expect("Couldn't get the starting point");

        let destinations = parts
            .get(1)
            .expect("Couldn't get destinations")
            .split(", ")
            .collect::<Vec<&str>>();

        let left_with_bracket = destinations.get(0).expect("Couldn't get left destination");
        let left = &left_with_bracket[1..left_with_bracket.len()];
        let right_with_bracket = destinations.get(1).expect("Couldn't get right destination");
        let right = &right_with_bracket[0..right_with_bracket.len() - 1];

        paths.insert(start.to_string(), Path { start, left, right });
    });

    return paths;
}
