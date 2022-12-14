use std::fs;

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn main() {
    let file = get_file_as_string("test-input.txt");

    let all_trees = file
        .split("\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<&str>>();

    let visible_trees = all_trees
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, line)| {
            // If we're on the first or last line then all trees are visible
            if idx == 0 || idx == all_trees.len() - 1 {
                return line.chars().collect::<Vec<char>>().len() as i64;
            }

            println!("testing line: {:?}", line);
            let trees_in_line = line
                .chars()
                .map(|char| char.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            return line
                .chars()
                .map(|tree| tree.to_string().parse::<i64>().unwrap())
                .enumerate()
                .map(|(tree_idx, tree)| {
                    if tree_idx == 0 || tree_idx == trees_in_line.len() - 1 {
                        return 1;
                    }
                    println!("testing tree: {:?}", tree);

                    // Check to the left
                    if trees_in_line[tree_idx - 1] < tree {
                        println!("visible to left");
                        return 1;
                    }
                    // Check to the right
                    if trees_in_line[tree_idx + 1] < tree {
                        println!("visible to right");
                        return 1;
                    }

                    // Check above
                    let mut working_idx: i64 = idx as i64 - 1;
                    while working_idx >= 0 {
                        if all_trees.clone()[working_idx as usize]
                            .split("")
                            .collect::<Vec<&str>>()[tree_idx]
                            .parse::<i64>()
                            .unwrap()
                            < tree
                        {
                            println!("Visible above");
                            return 1;
                        } else {
                            working_idx = working_idx - 1;
                        }
                    }

                    // Check below
                    let mut working_idx2: i64 = idx as i64 + 1;
                    while working_idx2 <= all_trees.len() as i64 - 1 {
                        if all_trees[working_idx2 as usize]
                            .split("")
                            .collect::<Vec<&str>>()[tree_idx]
                            .parse::<i64>()
                            .unwrap()
                            < tree
                        {
                            println!("visible below");
                            return 1;
                        } else {
                            working_idx2 = working_idx2 + 1;
                        }
                    }

                    return 0;
                })
                .sum::<i64>();
        })
        .sum::<i64>();

    println!("Visible Trees: {:?}", visible_trees);
}
