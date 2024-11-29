use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Couldn't read file");
    let mut floor: i32 = 0;
    let mut found_index: i32 = -1;

    file.chars().enumerate().for_each(|(index, char)| {
        if char.eq(&'(') {
            floor += 1;
        }

        if char.eq(&')') {
            floor -= 1;
        }

        if floor == -1 && found_index as i32 == -1 {
            found_index = index as i32;
        }
    });

    println!("First point to get to -1: {}", found_index + 1);
}
