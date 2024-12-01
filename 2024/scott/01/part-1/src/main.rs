fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = file
        .trim()
        .lines()
        .map(|line| {
            let splits = line.split("   ").collect::<Vec<&str>>();
            (
                splits.clone().get(0).unwrap().parse::<i32>().unwrap(),
                splits.clone().get(1).unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    // Have to do this manually because you can't sort an iterator of two Vectors within an
    // iterator
    left.sort();
    right.sort();

    let total_difference: i32 = left
        .into_iter()
        .enumerate()
        .map(|(index, value)| (value - right.get(index).unwrap()).abs())
        .sum();

    println!("total_difference: {}", total_difference);
}
