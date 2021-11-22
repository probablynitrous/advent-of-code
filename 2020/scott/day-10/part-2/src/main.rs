use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::Instant,
};

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("Could not find file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| {
            l.expect("Could not read line")
                .parse()
                .expect("Valid integer")
        })
        .collect()
}

fn get_binary_lists(numbers: Vec<i32>) -> Vec<Vec<i32>> {
    let mut wrapper: Vec<Vec<i32>> = vec![];
    let mut inner: Vec<i32> = vec![];

    for (idx, number) in numbers.iter().enumerate() {
        // Push the first one into the inner vec
        if idx == 0 {
            inner.push(number.to_owned());
        }

        // If we're not at the end then we can compare
        if idx > 0 && idx < numbers.len() - 1 {
            // Check if we're at the start of a block
            if number - numbers.get(idx - 1).expect("Valid index") > 1 {
                if inner.len() > 1 {
                    wrapper.push(inner.clone());
                }
                inner = vec![];
                inner.push(number.to_owned());
            } else if numbers.get(idx + 1).expect("Valid index") - number != 1
                && number - numbers.get(idx - 1).expect("Valid index") == 1
            {
                // Check if we're at the end of a block
                // Then we need to push the value and reset the array
                inner.push(number.to_owned());
                if inner.len() > 1 {
                    wrapper.push(inner.clone());
                }
                inner = vec![];
            } else if numbers.get(idx + 1).expect("Valid index") - number == 1 {
                // This is the standard push
                inner.push(number.to_owned());
            }
        }

        // If we're at the end then we need to compare
        if idx == numbers.len() - 1 {
            if number - numbers.get(idx - 1).expect("Valid index") == 1 {
                inner.push(number.to_owned());
            }

            if inner.len() > 1 {
                wrapper.push(inner.clone());
            }
        }
    }

    return wrapper;
}

fn get_permutations(length: i32) -> i64 {
    if length == 1 {
        return 1;
    }
    // The seed for the algorithm
    let (mut num1, mut num2, mut num3) = (0, 0, 1);
    let mut perm = 0;

    for _ in 0..(length - 1) {
        perm = num1 + num2 + num3;
        num1 = num2;
        num2 = num3;
        num3 = perm;
    }

    return perm;
}

fn main() {
    let mut lines = get_lines_from_file("../input.txt");
    lines.push(0);
    lines.sort();

    lines.push(lines.get(lines.len() - 1).expect("Valid index") + 3);

    let now = Instant::now();

    let binary_lists: Vec<Vec<i32>> = get_binary_lists(lines.to_owned());

    let mut count: i64 = 1;
    for list in binary_lists {
        let perms = get_permutations(list.len() as i32);
        count = count * perms;
    }

    println!("Solution: {}", count);
    println!(
        "Ran in {}ms, ({}mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
}
