use itertools::Itertools;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Airplane {
    groups: Vec<Group>,
}

#[derive(Debug)]
struct Group {
    answers: Vec<Answers>,
}

#[derive(Debug)]
struct Answers {
    line: String,
}

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Could not find file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line."))
        .collect()
}

fn main() {
    let lines = get_lines_from_file("../input.txt");
    let mut airplane = Airplane { groups: vec![] };
    let mut group = Group { answers: vec![] };
    let mut use_line: bool;

    // Similar to what we were doing on Day 4, we need to be able to group all of the answers onto
    // the same line so we can just iterate through them. Therefore, we need two Vectors to contain
    // both the working line and the collection of usable lines.
    for line in lines {
        if line.len() == 0 {
            use_line = false;
        } else {
            use_line = true;
        }

        if use_line {
            group.answers.push(Answers { line });
        } else {
            airplane.groups.push(group);
            group = Group { answers: vec![] };
        }
    }

    // Push the last one
    airplane.groups.push(group);

    let mut count = 0;

    for g in airplane.groups {
        // If there is only one answer in the group, then we can just add that line as "all" the
        // people in the group answered the same questions.
        if *&g.answers.len() as i32 == 1 {
            count += &g.answers[0].line.len();
        } else {
            let mut answers: String = "".to_owned();
            for answer in &g.answers {
                answers.push_str(&answer.line);
            }

            let mut answer_chars = answers.chars().collect::<Vec<char>>();
            answer_chars.sort();

            let sorted_answer_chars: Vec<String> = answer_chars
                .iter()
                .group_by(|&x| x)
                .into_iter()
                .map(|(_, r)| r.collect())
                .collect();

            let mut grouped_answers = 0;

            for current_char_collection in sorted_answer_chars {
                if &current_char_collection.len() == &g.answers.len() {
                    grouped_answers += 1
                }
            }

            count += grouped_answers;
        }
    }

    println!("Solution: sum is {}", count);
}
