use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    fn up(position: &Position) -> Position {
        Position {
            x: position.x,
            y: position.y - 1,
        }
    }

    fn down(position: &Position) -> Position {
        Position {
            x: position.x,
            y: position.y + 1,
        }
    }

    fn left(position: &Position) -> Position {
        Position {
            x: position.x - 1,
            y: position.y,
        }
    }

    fn right(position: &Position) -> Position {
        Position {
            x: position.x + 1,
            y: position.y,
        }
    }
}

#[derive(Debug)]
struct Tile {
    pub has_blocker: bool,
}

#[derive(Debug)]
struct Grid {
    tiles: HashMap<Position, Tile>,
}

impl Grid {
    fn is_valid_position(&self, position: &Position) -> bool {
        self.tiles.get(position).is_some()
    }

    fn is_blocker_tile(&self, position: &Position) -> bool {
        self.tiles.get(position).unwrap().has_blocker
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_next(direction: &Direction) -> Direction {
        match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

struct Guard {
    pub position: Position,
    pub direction: Direction,
}

impl Guard {
    fn travel(&mut self) {
        self.position = self.peek();
    }

    fn turn(&mut self) {
        self.direction = Direction::get_next(&self.direction);
    }

    fn peek(&self) -> Position {
        match &self.direction {
            Direction::North => Position::up(&self.position),
            Direction::East => Position::right(&self.position),
            Direction::South => Position::down(&self.position),
            Direction::West => Position::left(&self.position),
        }
    }
}

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Couldn't the test file");

    let mut grid = Grid {
        tiles: HashMap::new(),
    };

    let mut guard = Guard {
        position: Position { x: -1, y: -1 },
        direction: Direction::North,
    };

    file.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            let position = Position {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            };

            let mut has_blocker = false;
            match char {
                '#' => has_blocker = true,
                '^' => {
                    guard.position = position;
                }
                _ => (),
            };

            let tile = Tile { has_blocker };

            grid.tiles.insert(position, tile);
        });
    });

    if guard.position == (Position { x: -1, y: -1 }) {
        panic!("Failed to initialise Guard starting position");
    }

    let mut positions_visited: HashSet<Position> = HashSet::new();

    while grid.is_valid_position(&guard.position) {
        positions_visited.insert(guard.position.clone());

        if !grid.is_valid_position(&guard.peek()) {
            break;
        }

        if grid.is_blocker_tile(&guard.peek()) {
            guard.turn();
        }

        guard.travel();
    }

    println!("Guard finished at {:?}", &guard.position);

    println!("Visited {} unique tiles", positions_visited.len());
}
