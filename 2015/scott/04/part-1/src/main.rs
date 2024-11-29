use hex;
use md5;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Couldn't read file");
    let file = file.trim();
    let mut suffix = 0;

    let mut found = false;

    // We're assuming we don't find it on the first iteration - that would be pretty wild
    while !found {
        suffix += 1;

        let computation_string = file.to_owned() + &suffix.to_string();
        let hash = md5::compute(&computation_string);
        let encoded_hash = hex::encode(&hash.0.as_slice()[0..4]);

        // Need to trim since we get 6 characters here due to the conversion
        if &encoded_hash[0..5] == "00000" {
            found = true;
        }
    }

    println!("Required number: {}", suffix);
}
