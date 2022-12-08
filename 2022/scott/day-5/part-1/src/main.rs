use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, time::Instant};

lazy_static! {
    static ref RE: Regex = Regex::new(r"\[\w\]").expect("Failed to parse box Regex");
}

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

#[derive(PartialEq)]
enum ProcessingMode {
    Setup,
    Instructions,
}

#[derive(Clone, Debug)]
struct Stack {
    pub crates: Vec<Option<Box>>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { crates: vec![] }
    }

    // We need to clone here so we can pass ownership back to the caller,
    // since both `.last()` and the box itself return Options.
    pub fn get_top(&self) -> Option<Box> {
        self.crates.last().unwrap().clone()
    }

    pub fn add_to_stack(&mut self, value: &str) {
        // We only need to add to the stack if we actually have a box, as this
        // will save us having to trim the `None()`'s after the fact.
        if RE.is_match(value) {
            // Make sure that we're inserting to the front since we're building
            // this from the top down.
            self.crates.insert(
                0,
                Some(Box {
                    name: value.chars().collect::<Vec<char>>()[1].to_string(),
                }),
            );
        }
    }
}

#[derive(Clone, Debug)]
struct Box {
    pub name: String,
}

#[derive(Debug)]
struct Instruction {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl Instruction {
    // Pretty brittle this, but the assumption is that the input isn't going to
    // change and so we can continue to read the instructions in with this
    // specific format.
    pub fn from_str(line: &str) -> Instruction {
        let slice = line.split(" ").collect::<Vec<&str>>();

        return Instruction {
            count: slice[1]
                .parse()
                .expect("Couldn't parse value for instruction"),
            from: slice[3]
                .parse()
                .expect("Couldn't parse value for instruction"),
            to: slice[5]
                .parse()
                .expect("Couldn't parse value for instruction"),
        };
    }
}

// Interpret the row of crates, ignoring it if we don't reach it (in the event
// that we've hit the dividing line between the setup and instructions)
// We're returning Option here as it's nice and fast to filter out
fn get_row(line: &str) -> Option<Vec<String>> {
    if !RE.is_match(line) {
        return None;
    }

    let mut columns: Vec<String> = vec![];
    let mut working_column: Vec<char> = vec![];

    // Add the boxes into the appropriate columns
    for (idx, char) in line.chars().into_iter().enumerate() {
        // If we're at the end of the line, then we need to push the final character
        // and then push the colection to the columns vec
        if idx == line.len() - 1 {
            working_column.push(char);
            columns.push(String::from_iter(&working_column).to_owned());
            continue;
        }

        // If the working group is of the right length, then we can move that
        // to the list of columns
        if working_column.len() == 3 {
            columns.push(String::from_iter(&working_column).to_owned());
            working_column = vec![];
        } else {
            // Otherwise just continue to increase the working group
            working_column.push(char);
        }
    }

    return Some(columns);
}

// Could do this in separate functions, but we might as well do them together
// Builds a vector of stacks (for the crates) and a vector of instructions (for
// the movements).
fn build_stacks_and_instructions(file: String) -> (Vec<Stack>, Vec<Instruction>) {
    let mut processing_mode = ProcessingMode::Setup;
    let mut stacks: Vec<Stack> = vec![];
    let mut instructions: Vec<Instruction> = vec![];

    file.split("\n").into_iter().for_each(|line| {
        // If we hit the divider then we should swap to swallowing the instructions instead
        if line == "" {
            // This will also be hit at the end of the file but we don't care (for now).
            processing_mode = ProcessingMode::Instructions;
            return;
        }

        if processing_mode == ProcessingMode::Setup {
            let row = get_row(line);

            if row.is_none() {
                return;
            } else {
                for (index, bag) in row.unwrap().into_iter().enumerate() {
                    if stacks.get(index).is_none() {
                        stacks.push(Stack::new());
                    }
                    stacks[index].add_to_stack(&bag);
                }
            }
        }

        if processing_mode == ProcessingMode::Instructions {
            instructions.push(Instruction::from_str(line));
        }
    });

    (stacks, instructions)
}

// Iterate over the instructions and apply the movements to the crates
fn apply_instructions(instructions: &Vec<Instruction>, stacks: &mut Vec<Stack>) {
    for instruction in instructions {
        let mut i = 0;
        while i < instruction.count {
            // Need to add 1 here since the instructions are 1-based and arrays
            // are 0-based.
            let box_to_move = stacks[instruction.from - 1].crates.pop().unwrap();
            stacks[instruction.to - 1].crates.push(box_to_move);

            i += 1;
        }
    }
}

fn main() {
    let file = get_file_as_string("input.txt");

    let now = Instant::now();

    let (mut stacks, instructions) = build_stacks_and_instructions(file);

    apply_instructions(&instructions, &mut stacks);

    let top_boxes = stacks
        .into_iter()
        .map(|stack| stack.get_top())
        .filter(|v| v.is_some())
        .map(|v| v.unwrap().name)
        .collect::<Vec<String>>()
        .join("");

    let elapsed = now.elapsed();

    println!("top boxes: {:?}", top_boxes);
    println!("took {:?}", elapsed);
}
