use std::{collections::HashMap, fs, time::Instant};

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn get_duplicate_from_group(group: Vec<&str>, alphabet: &HashMap<char, i64>) -> i64 {
    let mut groups_with_priorities = group
        .into_iter()
        .map(|group| {
            // Convert each of the bags into their respective priorities
            let mut priority_for_group = group
                .chars()
                // Need to add 1 here since indexes start at 0
                .map(|item| alphabet.get(&item).unwrap() + (1 as i64))
                .collect::<Vec<i64>>();

            // Sort so we can start at the lowest
            priority_for_group.sort();

            // Dedup for quicker iteration time
            priority_for_group.dedup();

            return priority_for_group;
        })
        .collect::<Vec<Vec<i64>>>();

    // Grab the first one so we can compare with the rest
    let first_group = groups_with_priorities.pop().unwrap();

    return first_group
        .into_iter()
        .find(|priority| {
            // Find the priority that exists in the rest of the bags
            return groups_with_priorities
                .iter()
                .all(|group| group.contains(priority));
        })
        .unwrap();
}

fn main() {
    let file = get_file_as_string("input.txt");

    let now = Instant::now();
    // Store the alphabet in a hashmap so that we can index by char in O(1) time
    let alphabet: HashMap<char, i64> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect::<Vec<char>>()
        .into_iter()
        .enumerate()
        .map(|(i, char)| (char, i as i64))
        .collect();

    // There's definitely a better way to split a vec of strings into groups
    // of three, but I didn't want to use external libraries such as itertools.
    let mut groups: Vec<Vec<&str>> = vec![];
    let mut working_group: Vec<&str> = vec![];

    for (idx, bag) in file.split("\n").enumerate() {
        if idx > 0 && idx % 3 == 0 {
            groups.push(working_group);
            working_group = vec![];
        }

        working_group.push(bag);
    }

    // Most of the work is done in this little section here.
    let sum = groups
        .into_iter()
        .map(|group| get_duplicate_from_group(group.to_vec(), &alphabet))
        .sum::<i64>();

    let elapsed = now.elapsed();
    println!("sum: {:?}", sum);
    println!("took: {:?}", elapsed);
}
