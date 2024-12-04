use std::char;

fn main() {
    let file = std::fs::read_to_string("./test-input.txt").expect("Failed to read file");

    let mut xmas_count = 0;
    let mut horizontals = 0;
    let mut verticals = 0;
    let mut diagonal_rights = 0;
    let mut diagonal_lefts = 0;

    let lines = file.lines().collect::<Vec<&str>>();

    // Might not be able to use .windows() here if we need to read up as well as down
    for (line_index, multi_line) in lines.windows(4).into_iter().enumerate() {
        // Check horizontal, always the first so we don't double-count
        let first_line_in_group = multi_line.get(0).expect("Couldn't get first line in group");
        let horizontal_for_line = first_line_in_group
            .chars()
            .collect::<Vec<char>>()
            .windows(4)
            .map(|line| {
                let line_str = line.into_iter().collect::<String>();
                let mut count = 0;
                if line_str.eq("XMAS") {
                    count += 1;
                }

                if line_str.eq("SAMX") {
                    count += 1;
                }

                count
            })
            .sum::<u32>();

        xmas_count += horizontal_for_line;
        horizontals += horizontal_for_line;

        for x in 0..first_line_in_group.len() {
            // Check vertical
            let letter_collection = multi_line.iter().fold("".to_owned(), |mut acc, line| {
                acc.push_str(&line.chars().nth(x).unwrap().to_string());
                acc
            });

            if line_index == 4 && x == 6 {
                println!("letter_collection: {}", letter_collection);
            }

            if letter_collection.eq("XMAS") || letter_collection.eq("SAMX") {
                verticals += 1;
                xmas_count += 1;

                println!("Found vertical at x: {}, y: {}", x, line_index);
            }

            // Skip when we don't have space for diagonals
            if x < first_line_in_group.len() - 4 {
                // Check diagonal
                let diagonal_right_collection =
                    multi_line
                        .iter()
                        .enumerate()
                        .fold("".to_owned(), |mut acc, (index, line)| {
                            acc.push_str(&line.chars().nth(x + index).unwrap().to_string());
                            acc
                        });

                if diagonal_right_collection.eq("XMAS") || diagonal_right_collection.eq("SAMX") {
                    xmas_count += 1;
                    diagonal_rights += 1;
                }
            }

            if x > 3 {
                let diagonal_left_collection =
                    multi_line
                        .iter()
                        .enumerate()
                        .fold("".to_owned(), |mut acc, (index, line)| {
                            acc.push_str(&line.chars().nth((x + index) - 4).unwrap().to_string());
                            acc
                        });

                if diagonal_left_collection.eq("XMAS") || diagonal_left_collection.eq("SAMX") {
                    xmas_count += 1;
                    diagonal_lefts += 1;
                }
            }
        }
    }

    println!("Found {xmas_count} \"XMAS\" in word search");
    println!(
        "Of which {} were horizontal,  {} were vertical, {} were diagonal right and {} were diagonal left",
        horizontals, verticals, diagonal_rights, diagonal_lefts
    );
}
