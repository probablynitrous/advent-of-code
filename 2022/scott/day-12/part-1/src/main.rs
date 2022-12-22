use std::{collections::VecDeque, fs};

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Tile {
    pub value: i64,
    pub position: Coordinate,
    pub is_finish: bool,
}

type Grid = Vec<Vec<Tile>>;

fn generate_grid(file: &str) -> Grid {
    file.lines()
        .enumerate()
        .map(|(line_idx, line)| {
            let heights = line.chars();

            heights
                .enumerate()
                .map(|(height_idx, height)| Tile {
                    position: Coordinate {
                        x: height_idx,
                        y: line_idx,
                    },
                    value: if height.eq(&'S') {
                        'a' as i64
                    } else if height.eq(&'E') {
                        'z' as i64
                    } else {
                        height as i64
                    },
                    is_finish: if height.eq(&'E') { true } else { false },
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Grid>()
}

fn is_edge_tile(tile: &Tile, tiles: &Grid) -> bool {
    tile.position.x == 0
        || tile.position.x == tiles.len() - 1
        || tile.position.y == 0
        || tile.position.y == tiles.len() - 1
}

fn get_edges_for_tile(tile: &Tile, tiles: &Grid) -> Vec<Tile> {
    let mut edges: Vec<Tile> = Vec::new();
    if is_edge_tile(tile, tiles) {
        if tile.position.x == 0 {
            edges.push(tiles[tile.position.y][tile.position.x + 1].clone());
        }

        if tile.position.x == tiles[tile.position.y].len() - 1 {
            edges.push(tiles[tile.position.y][tile.position.x - 1].clone());
        }

        if tile.position.y == 0 {
            edges.push(tiles[tile.position.y + 1][tile.position.x].clone());
        }

        if tile.position.y == tiles[tile.position.y].len() - 1 {
            edges.push(tiles[tile.position.y - 1][tile.position.x].clone());
        }
    }

    return edges;
}

fn bfs(tiles: &Grid) -> i64 {
    let mut queue: VecDeque<Tile> = VecDeque::new();

    queue.push_back(tiles[0][0].clone());

    println!(
        "edges: {:?}",
        get_edges_for_tile(queue.pop_front().unwrap(), tiles)
    );

    // while queue.len() > 0 {
    //     // We shouldn't ever panic
    //     let current = queue.pop_front().unwrap();
    //
    //     if current.is_finish {
    //         return queue.len() as i64;
    //     }
    //
    //     let edges = get_edges_for_tile(current, tiles);
    // }

    return -1;
}

fn main() {
    let file = get_file_as_string("test-input.txt");

    let tiles = generate_grid(&file);

    let distance = bfs(&tiles);

    println!("distance: {:?}", distance);
}
