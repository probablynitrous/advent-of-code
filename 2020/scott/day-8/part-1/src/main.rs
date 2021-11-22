use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::Instant,
};

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Could not find file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line."))
        .collect()
}

struct Interpreter {
    accumulator: i32,
    instruction_pointer: i32,
}

impl Interpreter {
    fn add(&mut self, amount: i32) {
        self.accumulator += amount;
    }

    fn move_pointer(&mut self, distance: i32) {
        self.instruction_pointer += distance;
    }

    fn get_ip(&self) -> i32 {
        self.instruction_pointer
    }

    fn get_accumulator(&self) -> i32 {
        self.accumulator
    }
}

struct Instruction {
    line: String,
}

#[derive(Debug)]
enum Command {
    NOP,
    ACC,
    JMP,
}

impl Command {
    fn from_str(command: &str) -> Option<Self> {
        match command {
            "nop" => Some(Self::NOP),
            "acc" => Some(Self::ACC),
            "jmp" => Some(Self::JMP),
            _ => None,
        }
    }
}

fn main() {
    let lines = get_lines_from_file("../input.txt");
    let mut interpreter = Interpreter {
        accumulator: 0,
        instruction_pointer: 0,
    };
    let mut completed_instructions: Vec<i32> = vec![];
    let mut has_seen_instruction = false;

    let now = Instant::now();
    while !has_seen_instruction {
        let instruction_line = lines
            .get(interpreter.get_ip() as usize)
            .expect("Could not get instruction");
        let current_instruction = Instruction {
            line: instruction_line.to_owned(),
        };

        if completed_instructions.len() > 0
            && completed_instructions
                .iter()
                .any(|&i| &i == &interpreter.get_ip())
        {
            has_seen_instruction = true;
        } else {
            completed_instructions.push(interpreter.get_ip());
            interpret_instruction(&current_instruction, &mut interpreter);
        }
    }

    println!(
        "Ran for {}ms ({}ns)",
        now.elapsed().as_millis(),
        now.elapsed().as_nanos()
    );
    println!("Acc: {}", interpreter.get_accumulator());
}

fn interpret_instruction(instruction: &Instruction, interpreter: &mut Interpreter) {
    let command_with_value = instruction.line.split(" ").collect::<Vec<&str>>();
    let command = Command::from_str(command_with_value[0]).expect("Valid command");
    // Needs to be i32 for the parse to work
    let value: i32 = command_with_value[1].parse().expect("Valid interger");

    match command {
        Command::NOP => interpreter.move_pointer(1),
        Command::ACC => {
            interpreter.add(value);
            interpreter.move_pointer(1);
        }
        Command::JMP => interpreter.move_pointer(value),
    };
}
