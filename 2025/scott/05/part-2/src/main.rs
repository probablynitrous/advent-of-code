use std::time::Instant;

#[derive(Debug)]
struct Range {
    min: u64,
    max: u64,
}

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    let start = Instant::now();
    let mut ranges = Vec::<Range>::new();

    // Parse the file
    file.lines()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let (start, end) = line.split_once("-").unwrap();

            ranges.push(Range {
                min: start.parse().unwrap(),
                max: end.parse().unwrap(),
            });
        });

    // Sort by the min
    ranges.sort_by(|a, b| a.min.cmp(&b.min));

    // Resolve the ranges
    let mut resolved_ranges = Vec::<Range>::new();
    for range in ranges {
        if resolved_ranges.is_empty() {
            resolved_ranges.push(range);
            continue;
        }

        let highest_current = resolved_ranges.last_mut().unwrap();

        // We're happy the ranges are disjoint
        if range.min > highest_current.max {
            resolved_ranges.push(range);
            continue;
        }

        // The range we're looking at is already included in the previous one
        if range.max <= highest_current.max {
            continue;
        }

        // The new range extends the highest current
        if range.min <= highest_current.max && range.max > highest_current.max {
            highest_current.max = range.max;
            continue;
        }
    }

    let total_valid_ids = resolved_ranges
        .iter()
        .map(|range| (range.max - range.min) + 1)
        .sum::<u64>();

    let total_time = Instant::now() - start;

    println!("Total valid ids {:?}", total_valid_ids);
    println!("Took {:?}", total_time);
}
