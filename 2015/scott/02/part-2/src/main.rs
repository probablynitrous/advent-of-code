use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Couldn't read file");

    let wrapping_paper_required: u32 = file
        .split("\n")
        .map(|line| {
            if line.len() == 0 {
                return 0;
            }

            let mut dimensions = line
                .split("x")
                .map(|dimension| dimension.parse().unwrap())
                .collect::<Vec<u32>>();

            return compute_ribbon(&mut dimensions);
        })
        .sum();

    println!("Total amount: {}", wrapping_paper_required);
}

fn compute_ribbon(dimensions: &mut Vec<u32>) -> u32 {
    dimensions.sort();

    let perimeter = dimensions.get(0).unwrap() * 2 + dimensions.get(1).unwrap() * 2;

    return perimeter + dimensions.iter().product::<u32>();
}
