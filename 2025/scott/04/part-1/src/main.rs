use std::{collections::HashMap, time::Instant};

#[derive(Debug)]
struct Cell {
    x: usize,
    y: usize,
    occupied: bool,
}

struct Grid {
    rows: HashMap<usize, Vec<Cell>>,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> &Cell {
        self.rows.get(&y).unwrap().get(x).unwrap()
    }
}

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    let start = Instant::now();

    let grid = build_grid(&file);
    let accessible_rolls = grid
        .rows
        .values()
        .map(|row| {
            row.iter()
                .filter(|cell| count_surrounds(cell, &grid) < 4 && cell.occupied)
                .count()
        })
        .sum::<usize>();

    let total_time = Instant::now() - start;

    println!("Accessible roll {}", accessible_rolls);
    println!("Took {:?}", total_time);
}

fn build_grid(file: &str) -> Grid {
    let mut grid = Grid {
        rows: HashMap::new(),
    };

    file.lines().enumerate().for_each(|(y, line)| {
        let row = line
            .chars()
            .enumerate()
            .map(|(x, ch)| Cell {
                x,
                y,
                occupied: ch == '@',
            })
            .collect();
        grid.rows.insert(y, row);
    });

    grid
}

fn count_surrounds(cell: &Cell, grid: &Grid) -> u8 {
    let mut surrounds = 0;
    let row_length = grid.rows.get(&0).unwrap().len();

    // Immediate left
    if cell.x > 0 && grid.get(cell.x - 1, cell.y).occupied {
        surrounds += 1;
    }

    // Immediate right
    if cell.x < row_length - 1 && grid.get(cell.x + 1, cell.y).occupied {
        surrounds += 1;
    }

    // Immediate top
    if cell.y > 0 && grid.get(cell.x, cell.y - 1).occupied {
        surrounds += 1;
    }

    // Immediate bottom
    if cell.y < row_length - 1 && grid.get(cell.x, cell.y + 1).occupied {
        surrounds += 1;
    }

    // Top right
    if cell.y > 0 && cell.x < row_length - 1 && grid.get(cell.x + 1, cell.y - 1).occupied {
        surrounds += 1;
    }

    // Bottom left
    if cell.y < grid.rows.len() - 1 && cell.x > 0 && grid.get(cell.x - 1, cell.y + 1).occupied {
        surrounds += 1;
    }

    // Top left
    if cell.y > 0 && cell.x > 0 && grid.get(cell.x - 1, cell.y - 1).occupied {
        surrounds += 1;
    }

    // Bottom right
    if cell.y < grid.rows.len() - 1
        && cell.x < row_length - 1
        && grid.get(cell.x + 1, cell.y + 1).occupied
    {
        surrounds += 1;
    }

    return surrounds;
}
