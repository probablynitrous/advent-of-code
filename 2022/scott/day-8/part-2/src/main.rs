use std::fs;

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn get_tree_from_coord(x: usize, y: usize, all_trees: &Vec<Vec<i64>>) -> i64 {
    all_trees[y][x]
}

fn right_score(x: usize, y: usize, all_trees: &Vec<Vec<i64>>) -> i64 {
    let mut score = 0;
    let test_tree = get_tree_from_coord(x, y, all_trees);
    let mut working_x = x + 1;
    let mut is_visible = true;
    while working_x <= all_trees[y].len() - 1 && is_visible {
        if get_tree_from_coord(working_x, y, all_trees) >= test_tree {
            is_visible = false;
        }

        score += 1;
        working_x += 1;
    }

    score
}

fn top_score(x: usize, y: usize, all_trees: &Vec<Vec<i64>>) -> i64 {
    let mut score = 0;
    let test_tree = get_tree_from_coord(x, y, all_trees);
    let mut working_y: i64 = y as i64 - 1;
    let mut is_visible: bool = true;
    while working_y >= 0 && is_visible {
        if get_tree_from_coord(x, working_y as usize, all_trees) >= test_tree {
            is_visible = false;
        }

        working_y -= 1;
        score += 1;
    }

    score
}

fn left_score(x: usize, y: usize, all_trees: &Vec<Vec<i64>>) -> i64 {
    let mut score = 0;
    let test_tree = get_tree_from_coord(x, y, all_trees);
    let mut working_x: i64 = x as i64 - 1;
    let mut is_visible: bool = true;
    while working_x >= 0 && is_visible {
        if get_tree_from_coord(working_x as usize, y, all_trees) >= test_tree {
            is_visible = false;
        }

        working_x -= 1;
        score += 1;
    }

    score
}

fn bottom_score(x: usize, y: usize, all_trees: &Vec<Vec<i64>>) -> i64 {
    let mut score = 0;
    let test_tree = get_tree_from_coord(x, y, all_trees);
    let mut working_y: i64 = y as i64 + 1;
    let mut is_visible: bool = true;
    while working_y <= all_trees.len() as i64 - 1 && is_visible {
        if get_tree_from_coord(x, working_y as usize, all_trees) >= test_tree {
            is_visible = false;
        }

        working_y += 1;
        score += 1;
    }

    score
}

fn main() {
    let file = get_file_as_string("input.txt");

    let all_trees = file
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as i64)
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let highest_score = all_trees
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, line)| {
            return line
                .clone()
                .into_iter()
                .enumerate()
                .map(|(tree_idx, _)| {
                    let score = top_score(tree_idx, idx, &all_trees)
                        * right_score(tree_idx, idx, &all_trees)
                        * left_score(tree_idx, idx, &all_trees)
                        * bottom_score(tree_idx, idx, &all_trees);

                    return score;
                })
                .max();
        })
        .max();

    println!("Highest score: {:?}", highest_score);
}
