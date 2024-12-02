use std::cmp::Ordering;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("./input.txt")?;

    let safe_reports = file
        .lines()
        .filter(|line| {
            let values = line
                .split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .peekable()
                .collect::<Vec<i32>>();

            let mut deduplicated = values.clone();
            deduplicated.dedup();

            // If the lengths don't match then all the values weren't increasing or decreasing
            if deduplicated.len() != values.len() {
                return false;
            }

            let mut order: Ordering = Ordering::Greater;

            if values.get(1).unwrap() < values.get(0).unwrap() {
                order = Ordering::Less;
            }

            return values.windows(2).all(|values| {
                let current = values.get(0).unwrap();
                let next = values.get(1).unwrap();
                if order == Ordering::Greater {
                    return next > current && next - current <= 3;
                }

                return next < current && current - next <= 3;
            });
        })
        .collect::<Vec<&str>>()
        .len();

    println!("safe reports: {:?}", safe_reports);

    Ok(())
}
