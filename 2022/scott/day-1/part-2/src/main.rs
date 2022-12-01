use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn get_lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Could not find file.");
    let buf = BufReader::new(file);

    buf.lines().map(|line| line.unwrap()).collect()
}

fn main() {
    let lines = get_lines_from_file("input.txt");

    let mut largest: Vec<usize> = vec![];
    let mut current: usize = 0;

    for (index, line) in lines.iter().enumerate() {
        // Special case if we're on the last line and we've just started
        // another food.
        if index == lines.len() - 1 && current == 0 {
            insert_if_required(
                &mut largest,
                line.parse::<usize>().expect("Expected to parse integer"),
            );
            continue;
        }

        // If we find a value then just add that to the current value
        if line.len() != 0 && index != lines.len() {
            current += line.parse::<usize>().expect("Expected to parse integer");
            continue;
        }

        // If the top 3 list doesn't have 3 elements, then push the value
        // and reset the current
        if largest.len() < 3 {
            largest.push(current);
            current = 0;
            continue;
        }

        insert_if_required(&mut largest, current);

        current = 0;
        continue;
    }
    println!("sum: {:?}", largest);
    println!("sum: {:?}", largest.into_iter().sum::<usize>());
}

fn insert_if_required(collection: &mut Vec<usize>, needle: usize) -> Vec<usize> {
    // Get the index of the value that is less than the currnet
    let idx = collection
        .to_owned()
        .into_iter()
        .position(|value| value < needle);

    // If we've found one, then update that value
    if idx.is_some() {
        collection[idx.unwrap()] = needle;
    }

    return collection.to_vec();
}
