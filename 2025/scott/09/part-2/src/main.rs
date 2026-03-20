use std::{collections::HashMap, time::Instant};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Color {
    Green,
    Red,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Tile {
    x: i32,
    y: i32,
    color: Color,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn main() {
    let file = std::fs::read_to_string("./test-input.txt").expect("Failed to read input file");
    let start = Instant::now();

    let starting_tiles = file
        .lines()
        .map(|line| {
            let split = line.split(",").collect::<Vec<&str>>();
            return Tile {
                x: split.first().unwrap().parse().unwrap(),
                y: split.iter().nth(1).unwrap().parse().unwrap(),
                color: Color::Red,
            };
        })
        .collect::<Vec<Tile>>();

    let mut path: HashMap<Coordinate, Tile> = HashMap::new();

    for tile_window in starting_tiles.windows(2) {
        let red_one = tile_window.first().unwrap();
        let red_two = tile_window.iter().nth(1).unwrap();

        // First try to push the first point
        path.insert(
            Coordinate {
                x: red_one.x,
                y: red_one.y,
            },
            red_one.to_owned(),
        );

        // Build a line of Green tiles between the two red tiles
        for tile in compute_interim_tiles(red_one, red_two) {
            path.insert(
                Coordinate {
                    x: tile.x,
                    y: tile.y,
                },
                tile,
            );
        }

        path.insert(
            Coordinate {
                x: red_two.x,
                y: red_two.y,
            },
            red_two.to_owned(),
        );
    }

    // Wrap around
    for tile in compute_interim_tiles(
        starting_tiles.last().unwrap(),
        starting_tiles.first().unwrap(),
    ) {
        path.insert(
            Coordinate {
                x: tile.x,
                y: tile.y,
            },
            tile,
        );
    }

    let max_width = starting_tiles.iter().map(|tile| tile.x).max().unwrap() + 2;
    let max_height = starting_tiles.iter().map(|tile| tile.y).max().unwrap() + 2;
    let mut grid: Vec<Vec<Option<Tile>>> = Vec::new();

    // Build the grid (and raycast)
    for y in 0..max_height {
        let mut row: Vec<Option<Tile>> = Vec::new();
        let mut cross_count = 0;
        for x in 0..max_width {
            if let Some(tile) = path.get(&Coordinate { x, y }) {
                if let None = path.get(&Coordinate { x: x + 1, y }) {
                    cross_count += 1;
                }
                row.push(Some(tile.clone()));
            } else {
                if cross_count % 2 == 0 {
                    row.push(None);
                } else {
                    row.push(Some(Tile {
                        color: Color::Green,
                        x,
                        y,
                    }));
                }
            }
        }

        for tile in row.clone() {
            match tile {
                Some(tile) => match tile.color {
                    Color::Green => print!("X"),
                    Color::Red => print!("#"),
                },
                None => print!("."),
            }
        }

        print!("\n");

        grid.push(row);
    }

    let mut max = -1;

    for p1 in starting_tiles.iter() {
        for p2 in starting_tiles.iter() {
            if p2.eq(p1) {
                continue;
            }

            let min_x = i32::min(p1.x, p2.x);
            let min_y = i32::min(p1.y, p2.y);
            let max_x = i32::max(p1.x, p2.x);
            let max_y = i32::max(p1.y, p2.y);

            let top_left = Coordinate { x: min_x, y: min_y };
            let top_right = Coordinate { x: max_x, y: min_y };
            let bottom_left = Coordinate { x: min_x, y: max_y };
            let bottom_right = Coordinate { x: max_x, y: max_y };

            let mut is_valid = true;

            // First verify it's a valid rectangle
            // Top
            for x in (top_left.x)..=(top_right.x) {
                if !grid
                    .get(top_left.y as usize)
                    .unwrap()
                    .iter()
                    .nth(x as usize)
                    .iter()
                    .all(|t| t.is_some())
                {
                    is_valid = false;
                }
            }

            // Bottom
            for x in (bottom_left.x)..=(bottom_right.x) {
                if !grid
                    .get(bottom_left.y as usize)
                    .unwrap()
                    .iter()
                    .nth(x as usize)
                    .iter()
                    .all(|t| t.is_some())
                {
                    is_valid = false;
                }
            }

            // Left
            for y in (top_left.y)..=(bottom_left.y) {
                if grid
                    .get(y as usize)
                    .unwrap()
                    .get(min_x as usize)
                    .unwrap()
                    .is_none()
                {
                    is_valid = false;
                }
            }

            // Right
            for y in (top_right.y)..=(bottom_right.y) {
                if grid
                    .get(y as usize)
                    .unwrap()
                    .get(max_x as usize)
                    .unwrap()
                    .is_none()
                {
                    is_valid = false;
                }
            }

            if !is_valid {
                continue;
            }

            let product = ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1);
            if product > max {
                max = product
            }
        }
    }

    let total_time = Instant::now() - start;

    println!("max: {}", max);
    println!("Took {:?}", total_time);
}

fn compute_interim_tiles(tile_a: &Tile, tile_b: &Tile) -> Vec<Tile> {
    let mut interims = Vec::new();

    if tile_a.x == tile_b.x {
        for i in 1..(tile_a.y - tile_b.y).abs() {
            let green_tile = Tile {
                x: tile_a.x,
                y: if tile_a.y < tile_b.y {
                    tile_a.y + i
                } else {
                    tile_b.y + i
                },
                color: Color::Green,
            };
            interims.push(green_tile);
        }
    } else if tile_a.y == tile_b.y {
        for i in 1..(tile_a.x - tile_b.x).abs() {
            let green_tile = Tile {
                y: tile_a.y,
                x: if tile_a.x < tile_b.x {
                    tile_a.x + i
                } else {
                    tile_b.x + i
                },
                color: Color::Green,
            };
            interims.push(green_tile);
        }
    } else {
        panic!(
            "Found two red tiles that don't have a matching coordinate so we can't build a path between them"
        )
    }

    interims
}
