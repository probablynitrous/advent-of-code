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

    fn remove(&mut self, x: usize, y: usize) {
        self.rows.get_mut(&y).unwrap().get_mut(x).unwrap().occupied = false;
    }
}

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    let start = Instant::now();

    let mut grid = build_grid(&file);

    let mut has_delta = true;
    let mut total_rolls_removed = 0;

    while has_delta {
        let cells_to_remove = grid
            .rows
            .values()
            .flat_map(|row| row.iter())
            .filter(|cell| count_surrounds(cell, &grid) < 4 && cell.occupied)
            .map(|cell| (cell.x, cell.y))
            .collect::<Vec<(usize, usize)>>();

        if cells_to_remove.is_empty() {
            has_delta = false;
        } else {
            total_rolls_removed += cells_to_remove.len();
            cells_to_remove
                .iter()
                .for_each(|(x, y)| grid.remove(x.clone(), y.clone()));
        }
    }

    let total_time = Instant::now() - start;

    println!("Rolls removed: {}", total_rolls_removed);
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
