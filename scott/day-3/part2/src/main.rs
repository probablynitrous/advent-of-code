use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::Chars,
};

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Could not find file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() -> std::io::Result<()> {
    let lines = get_lines_from_file("../input.txt");

    // Now testing the following routes:
    //      a) Right 1, down 1
    //      b) Right 3, down 1 (already checked)
    //      c) Right 5, down 1
    //      d) Right 7, down 1
    //      e) Right 1, down 2
    //
    // Pretty sure that we can do all of them, just have to keep track of where we are in the file
    // Bonus is that we don't drop down if we wrap around, so we can just do all the checking on
    // the same line

    let mut a_current_line_index = 0;
    let mut a_trees: i64 = 0;
    let mut b_current_line_index = 0;
    let mut b_trees: i64 = 0;
    let mut c_current_line_index = 0;
    let mut c_trees: i64 = 0;
    let mut d_current_line_index = 0;
    let mut d_trees: i64 = 0;
    let mut e_current_line_index = 0;
    let mut e_trees: i64 = 0;
    let mut should_skip_line = true; // keep track of moving down 2 lines
    let mut line_count = 0;

    for l in lines {
        let chars = l.chars();
        a_trees = test_char_at_index(a_current_line_index, chars.clone(), a_trees);
        b_trees = test_char_at_index(b_current_line_index, chars.clone(), b_trees);
        c_trees = test_char_at_index(c_current_line_index, chars.clone(), c_trees);
        d_trees = test_char_at_index(d_current_line_index, chars.clone(), d_trees);

        if line_count > 0 {
            if !should_skip_line {
                // Ensure that we're only taking characters and updating indexes once every two line
                // increments.
                e_trees = test_char_at_index(e_current_line_index, chars.clone(), e_trees);
                should_skip_line = true;
            } else {
                e_current_line_index = update_index(e_current_line_index, chars.clone(), 1);
                should_skip_line = false;
            }
        }

        a_current_line_index = update_index(a_current_line_index, chars.clone(), 1);
        b_current_line_index = update_index(b_current_line_index, chars.clone(), 3);
        c_current_line_index = update_index(c_current_line_index, chars.clone(), 5);
        d_current_line_index = update_index(d_current_line_index, chars.clone(), 7);

        line_count += 1;
    }

    println!(
        "solution: {}",
        a_trees * b_trees * c_trees * d_trees * e_trees
    );

    Ok(())
}

fn test_char_at_index(index: usize, chars: Chars, trees: i64) -> i64 {
    let current_char = chars.clone().nth(index).unwrap();
    if String::from(current_char) == "#" {
        trees + 1
    } else {
        trees
    }
}

fn update_index(index: usize, chars: Chars, right_increment: usize) -> usize {
    if index + right_increment >= chars.clone().count() {
        // How far we are from the end.
        let remainder = chars.count() - index;
        right_increment - remainder
    } else {
        index + right_increment
    }
}
