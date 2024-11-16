use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Couldn't read file");
    let mut floor = 0;

    file.chars().for_each(|char| {
        if char.to_string().eq("(") {
            floor += 1;
            return;
        }

        if char.to_string().eq(")") {
            floor -= 1;
            return;
        }
    });

    println!("Current floor: {}", floor);
}
