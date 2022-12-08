use std::{collections::HashSet, fs, time::Instant};

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn main() {
    let file = get_file_as_string("input.txt");

    let now = Instant::now();

    let total = file
        .chars()
        .enumerate()
        .find(|(idx, _)| {
            // Get the next 4 chars
            let working_group = &file.chars().collect::<Vec<char>>()[*idx..*idx + 14];

            // Throw them into a set, which will remove any duplicates
            let group: HashSet<char> = HashSet::from_iter(working_group.to_vec());

            // Check whether the length of the set is the same as the input
            return group.len() == working_group.len();
        })
        .unwrap()
        .0
        // Add 4 here since the output of the `find` is the index, and we need
        // to include all the processed chars
        + 14;

    let elapsed = now.elapsed();

    println!("Number of chars = {:?}", total);
    println!("Took: {:?}", elapsed);
}
