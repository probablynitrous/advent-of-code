use std::char;

fn main() {
    let file = std::fs::read_to_string("./test-input.txt").expect("Failed to read file");

    let mut xmas_count = 0;
    let mut vertical_count = 0;
    let mut horizontal_count = 0;
    let mut diagonal_right_count = 0;
    let mut diagonal_left_count = 0;

    let lines = file.lines().collect::<Vec<&str>>();
    let line_length = lines.get(0).expect("Couldn't get first line").len();

    // All we need to do is find each X, then try to build XMAS from it
    file.lines().enumerate().for_each(|(y, line)| {
        println!("y: {y}");
        // Don't care if we don't have an X on this line, just skip over it
        if !line.contains("X") {
            return;
        }

        let chars = line.chars().collect::<Vec<char>>();

        line.chars().enumerate().for_each(|(x, char)| {
            // Don't care if we're not at an X, just skip over it
            if char.ne(&'X') {
                return;
            }

            // Read horizontally forwards
            if x < line_length - 3 {
                if has_xmas_horizontally(x, x + 4, &chars) {
                    xmas_count += 1;
                    horizontal_count += 1;
                }
            }

            // Read horizontally backwards
            if x > 3 {
                if has_xmas_horizontally(x - 3, x + 1, &chars) {
                    xmas_count += 1;
                    horizontal_count += 1;
                }
            }

            // Read veritcally down
            if y < lines.len() - 3 {
                if has_xmas_vertically(y, y + 4, x, &lines) {
                    xmas_count += 1;
                    vertical_count += 1;
                }
            }

            // Read veritically up
            if y > 3 {
                if has_xmas_vertically(y - 3, y + 1, x, &lines) {
                    xmas_count += 1;
                    vertical_count += 1;
                }
            }

            // Read diagonally down and right
            if y < lines.len() - 3 && x < line_length - 3 {
                let mut word = "".to_owned();

                for n in 0..4 {
                    let line = &lines
                        .get(y + n)
                        .expect("Couldn't find line at given X + N position");

                    word.push_str(
                        &line
                            .chars()
                            .collect::<Vec<char>>()
                            .get(x + n)
                            .expect("Couldn't find char at given (x,y) coordinate")
                            .to_string(),
                    )
                }

                if word == "XMAS" || word == "SAMX" {
                    xmas_count += 1;
                    diagonal_right_count += 1;
                }
            }

            // Read diagonally up and right
            if y > 3 && x < line_length - 3 {
                let mut word = "".to_owned();

                for n in 0..4 {
                    let line = &lines
                        .get(y - n)
                        .expect("Couldn't find line at given X + N position");

                    word.push_str(
                        &line
                            .chars()
                            .collect::<Vec<char>>()
                            .get(x + n)
                            .expect("Couldn't find char at given (x,y) coordinate")
                            .to_string(),
                    )
                }

                if word == "XMAS" || word == "SAMX" {
                    xmas_count += 1;
                    diagonal_right_count += 1;
                }
            }

            // Read diagonally down and left
            if y < lines.len() - 3 && x > 3 {
                let mut word = "".to_owned();

                for n in 0..4 {
                    let line = &lines
                        .get(y + n)
                        .expect("Couldn't find line at given X + N position");

                    word.push_str(
                        &line
                            .chars()
                            .collect::<Vec<char>>()
                            .get(x - n)
                            .expect("Couldn't find char at given (x,y) coordinate")
                            .to_string(),
                    )
                }

                if word == "XMAS" || word == "SAMX" {
                    xmas_count += 1;
                    diagonal_left_count += 1;
                }
            }

            // Read diagonally up and left
            if y > 3 && x > 3 {
                let mut word = "".to_owned();

                for n in 0..4 {
                    let line = &lines
                        .get(y - n)
                        .expect("Couldn't find line at given X + N position");

                    word.push_str(
                        &line
                            .chars()
                            .collect::<Vec<char>>()
                            .get(x - n)
                            .expect("Couldn't find char at given (x,y) coordinate")
                            .to_string(),
                    )
                }

                if word == "XMAS" || word == "SAMX" {
                    xmas_count += 1;
                    diagonal_left_count += 1;
                }
            }
        });
    });

    println!("Found {xmas_count} \"XMAS\" in word search");
    println!("Of which {horizontal_count} were horizontal, {vertical_count} were vertical, {diagonal_right_count} were diagonally to the right and {diagonal_left_count} were diagonally to the left");
}

fn has_xmas_horizontally(start: usize, end: usize, chars: &Vec<char>) -> bool {
    let word = &chars[start..end]
        .iter()
        .fold("".to_owned(), |mut acc, value| {
            acc.push_str(&value.to_string());
            acc
        });

    return word == "XMAS" || word == "SAMX";
}

fn has_xmas_vertically(start_y: usize, end_y: usize, x: usize, lines: &Vec<&str>) -> bool {
    let word = &lines[start_y..end_y]
        .iter()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .get(x)
                .expect("Couldn't find char at given (x,y) coordinate")
                .clone()
        })
        .fold("".to_owned(), |mut acc, value| {
            acc.push_str(&value.to_string());
            acc
        });

    return word == "XMAS" || word == "SAMX";
}
