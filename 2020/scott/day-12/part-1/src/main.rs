use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::Instant,
};

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Could not find file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line."))
        .collect()
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

impl Turn {
    fn from_str(turn: &str) -> Option<Self> {
        match turn {
            "R" => Some(Self::Right),
            "L" => Some(Self::Left),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_str(direction: &str) -> Option<Self> {
        match direction {
            "N" => Some(Self::North),
            "S" => Some(Self::South),
            "E" => Some(Self::East),
            "W" => Some(Self::West),
            _ => None,
        }
    }

    fn parse_bearing(bearing: &i32) -> Option<Self> {
        match bearing {
            0 => Some(Self::North),
            90 => Some(Self::East),
            180 => Some(Self::South),
            270 => Some(Self::West),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Ship {
    north: i32,
    south: i32,
    east: i32,
    west: i32,
    bearing: i32,
}

impl Ship {
    // Turn the ship
    fn turn(&mut self, direction: Turn, amount: i32) {
        match direction {
            Turn::Left => {
                if self.bearing - amount < 0 {
                    self.bearing = (self.bearing - amount) + 360;
                } else {
                    self.bearing -= amount;
                }

                if self.bearing == 360 {
                    self.bearing = 0;
                }
            }
            Turn::Right => {
                if self.bearing + amount > 360 {
                    self.bearing = (self.bearing + amount) - 360;
                } else {
                    self.bearing += amount;
                }

                if self.bearing == 360 {
                    self.bearing = 0;
                }
            }
        };
    }

    // Travel in the direction the ship is currently facing
    fn travel(&mut self, direction: Direction, distance: i32) {
        match direction {
            Direction::North => self.north += distance,
            Direction::South => self.south += distance,
            Direction::East => self.east += distance,
            Direction::West => self.west += distance,
        };
    }

    // Get the total distance
    fn get_manhattan_distance(&self) -> i32 {
        (&self.north - &self.south).abs() + (&self.east - &self.west).abs()
    }
}

fn main() {
    let instructions = get_lines_from_file("../input.txt");
    let now = Instant::now();
    let mut ship = Ship {
        north: 0,
        south: 0,
        east: 0,
        west: 0,
        bearing: 90,
    };

    for instruction in &instructions {
        let chars = instruction.chars().collect::<Vec<char>>();
        let instruction = chars[0].to_string();
        let amount: i32 = chars[1..]
            .iter()
            .clone()
            .collect::<String>()
            .parse()
            .expect("Valid integer");

        if instruction == "F" {
            ship.travel(
                Direction::parse_bearing(&ship.bearing).expect("Valid integer"),
                amount,
            );
        } else {
            if instruction == "L" || instruction == "R" {
                let turn = Turn::from_str(&instruction).expect("Valid direction");
                ship.turn(turn, amount)
            } else {
                let direction = Direction::from_str(&instruction).expect("Valid direction");
                ship.travel(direction, amount);
            }
        }
    }
    println!("Solution: {}", ship.get_manhattan_distance());

    println!(
        "Ran in {}ms ({} mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
}
