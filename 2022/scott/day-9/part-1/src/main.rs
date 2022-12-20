use std::{collections::HashSet, fs, hash::Hash, time::Instant};

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    pub x: i64,
    pub y: i64,
}

impl Coordinate {
    pub fn get_destination(start: &Coordinate, instruction: &Instruction) -> Coordinate {
        match instruction.direction {
            Direction::Up => Coordinate {
                y: start.y + instruction.distance,
                x: start.x,
            },
            Direction::Down => Coordinate {
                y: start.y - instruction.distance,
                x: start.x,
            },
            Direction::Right => Coordinate {
                x: start.x + instruction.distance,
                y: start.y,
            },
            Direction::Left => Coordinate {
                x: start.x - instruction.distance,
                y: start.y,
            },
        }
    }

    pub fn is_touching(&self, target: &Coordinate) -> bool {
        if
        // they're on top of each other
        self.x != target.x && 
                // check the right
                self.x + 1 != target.x
                // check the left
                && self.x - 1 != target.x
        {
            return false;
        }

        if
        // they're on top of each other
        self.y != target.y &&
                // check above
                self.y + 1 != target.y &&
                // check below
                self.y - 1 != target.y
        {
            return false;
        }

        return true;
    }

    pub fn step_to(&mut self, target: &Coordinate) {
        // The diagonal case
        if self.x != target.x && self.y != target.y {
            if target.x > self.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
            if target.y > self.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        } else if self.x == target.x && self.y != target.y {
            if target.y > self.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        } else if self.y == target.y && self.x != target.x {
            if target.x > self.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        }
    }
}

#[derive(Debug)]
struct Instruction {
    pub direction: Direction,
    pub distance: i64,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_str(line: &str) -> Direction {
        match line {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => todo!(),
        }
    }
}

fn main() {
    let file = get_file_as_string("input.txt");

    let now = Instant::now();
    let mut head_position = Coordinate { x: 0, y: 0 };
    let mut tail_position = Coordinate { x: 0, y: 0 };
    let mut visited_locations: HashSet<Coordinate> = HashSet::new();
    visited_locations.insert(tail_position.clone());

    let mut instructions: Vec<Instruction> = Vec::new();

    file.lines().for_each(|line| {
        let parts = line.split(" ").collect::<Vec<&str>>();

        instructions.push(Instruction {
            distance: parts[1].parse::<i64>().unwrap(),
            direction: Direction::from_str(parts[0]),
        });
    });

    instructions.iter().for_each(|instruction| {
        let destination = Coordinate::get_destination(&head_position, &instruction);

        while head_position != destination {
            head_position.step_to(&destination);

            while !tail_position.is_touching(&head_position) {
                tail_position.step_to(&head_position);
                visited_locations.insert(tail_position.clone());
            }
        }
    });

    let elapsed = now.elapsed();
    println!("total visited_locations {:?}", visited_locations.len());

    println!("Took: {:?}", elapsed);
}
