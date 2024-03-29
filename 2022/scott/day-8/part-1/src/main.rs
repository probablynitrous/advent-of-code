use std::fs;

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn get_tree_from_coord(x: usize, y: usize, all_trees: &Vec<Vec<i64>>) -> i64 {
    all_trees[y][x]
}

fn is_visible_right(x: usize, y: usize, all_trees: &Vec<Vec<i64>>) -> bool {
    let test_tree = get_tree_from_coord(x, y, all_trees);
    let mut working_x = x + 1;
    let mut is_visible: bool = true;
    while working_x <= all_trees[y].len() - 1 && is_visible {
        if get_tree_from_coord(working_x, y, all_trees) >= test_tree {
            is_visible = false;
        }

        working_x += 1;
    }

    is_visible
}

fn is_visible_above(x: usize, y: usize, all_trees: &Vec<Vec<i64>>) -> bool {
    let test_tree = get_tree_from_coord(x, y, all_trees);
    let mut working_y: i64 = y as i64 - 1;
    let mut is_visible: bool = true;
    while working_y >= 0 && is_visible {
        if get_tree_from_coord(x, working_y as usize, all_trees) >= test_tree {
            is_visible = false;
        }

        working_y -= 1;
    }

    is_visible
}

fn is_visible_left(x: usize, y: usize, all_trees: &Vec<Vec<i64>>) -> bool {
    let test_tree = get_tree_from_coord(x, y, all_trees);
    println!("test tree: {:?}", test_tree);
    let mut working_x: i64 = x as i64 - 1;
    let mut is_visible: bool = true;
    while working_x >= 0 && is_visible {
        if get_tree_from_coord(working_x as usize, y, all_trees) >= test_tree {
            is_visible = false;
        }

        working_x -= 1;
    }

    is_visible
}

fn is_visible_bottom(x: usize, y: usize, all_trees: &Vec<Vec<i64>>) -> bool {
    let test_tree = get_tree_from_coord(x, y, all_trees);
    let mut working_y: i64 = y as i64 + 1;
    let mut is_visible: bool = true;
    while working_y <= all_trees.len() as i64 - 1 && is_visible {
        if get_tree_from_coord(x, working_y as usize, all_trees) >= test_tree {
            is_visible = false;
        }

        working_y += 1;
    }

    is_visible
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

    let visible_trees = all_trees
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, line)| {
            // If we're on the first or last line then all trees are visible
            if idx == 0 || idx == all_trees.len() - 1 {
                return line.len() as i64;
            }

            return line
                .clone()
                .into_iter()
                .enumerate()
                .map(|(tree_idx, _)| {
                    if tree_idx == 0 || tree_idx == line.len() - 1 {
                        return 1;
                    }

                    println!("coords: {:?} {:?}", tree_idx, idx);

                    if is_visible_left(tree_idx, idx, &all_trees) {
                        println!("visible from left");
                        return 1;
                    }
                    if is_visible_right(tree_idx, idx, &all_trees) {
                        println!("Visible from right");
                        return 1;
                    }
                    if is_visible_above(tree_idx, idx, &all_trees) {
                        println!("Visible from above");
                        return 1;
                    }
                    if is_visible_bottom(tree_idx, idx, &all_trees) {
                        println!("Visible from bottom");
                        return 1;
                    }

                    return 0;
                })
                .sum::<i64>();
        })
        .sum::<i64>();

    println!("Visible Trees: {:?}", visible_trees);
}
