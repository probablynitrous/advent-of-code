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

#[derive(Clone, Copy, Debug)]
struct MachineCode {
    command: Command,
    value: i32,
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

    fn reset_accumulator(&mut self) {
        self.accumulator = 0;
    }

    fn reset_pointer(&mut self) {
        self.instruction_pointer = 0;
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    code: MachineCode,
    line_number: i32,
}

#[derive(Debug, Clone, Copy)]
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

// Get a list of all nop and jmp instructions (and their line number)
// Loop through them, alternating the instruction and then running the code
// If we duplicate a command, then onto the next instruction
// If we hit the end of the code, we've found a winner.

fn main() {
    let lines = get_lines_from_file("../input.txt");
    // Our source of truth for instructions that we can take a copy of and manipulate further
    let mut instructions: Vec<Instruction> = vec![];
    let mut interpreter = Interpreter {
        accumulator: 0,
        instruction_pointer: 0,
    };
    let mut has_code = false;
    let mut nop_jmp_instructions: Vec<Instruction> = vec![];
    let mut ip = 0;

    let now = Instant::now();

    for (index, line) in lines.iter().enumerate() {
        instructions.push(Instruction {
            code: convert_instruction(line),
            line_number: index as i32,
        });
        if line.contains("jmp") || line.contains("nop") {
            nop_jmp_instructions.push(Instruction {
                code: convert_instruction(line),
                line_number: index as i32,
            });
        }
    }

    // Global counter runs this section
    while !has_code {
        let mut completed_instructions: Vec<i32> = vec![];
        let swapping_instruction = nop_jmp_instructions
            .get(ip)
            .expect("Valid instruction pointer");
        let mut mutated_instructions: Vec<Instruction> = vec![];
        for instruction in &instructions {
            if instruction.line_number == swapping_instruction.line_number {
                mutated_instructions.push(Instruction {
                    code: MachineCode {
                        command: swap_command(instruction.code.command.to_owned()),
                        value: instruction.code.value,
                    },
                    line_number: instruction.line_number,
                });
            } else {
                mutated_instructions.push(Instruction {
                    code: instruction.code,
                    line_number: instruction.line_number,
                });
            }
        }
        let mut has_seen_instruction = false;

        interpreter.reset_pointer();
        interpreter.reset_accumulator();

        // Interpreter runs this section
        while !has_seen_instruction && !has_code {
            if interpreter.get_ip() > (mutated_instructions.len() as i32) - 1 {
                has_code = true;
            } else {
                let current_instruction: Instruction = mutated_instructions
                    .get(interpreter.get_ip() as usize)
                    .expect("Valid instruction")
                    .to_owned();

                if completed_instructions.len() > 0
                    && completed_instructions
                        .iter()
                        .any(|&i| i == interpreter.get_ip())
                {
                    has_seen_instruction = true;
                } else {
                    completed_instructions.push(interpreter.get_ip());
                    interpret_instruction(&current_instruction, &mut interpreter);
                }
            }
        }

        ip += 1;
    }

    println!(
        "Ran for {}ms ({}mis)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
    println!("Acc: {}", interpreter.get_accumulator());
}

fn convert_instruction(instruction: &String) -> MachineCode {
    let command_with_value = instruction.split(" ").collect::<Vec<&str>>();
    let command = Command::from_str(command_with_value[0]).expect("Valid command");
    // Needs to be i32 for the parse to work
    let value: i32 = command_with_value[1].parse().expect("Valid interger");

    MachineCode { command, value }
}

fn swap_command(command: Command) -> Command {
    match command {
        Command::NOP => Command::JMP,
        Command::JMP => Command::NOP,
        Command::ACC => command,
    }
}

fn interpret_instruction(instruction: &Instruction, interpreter: &mut Interpreter) {
    match instruction.code.command {
        Command::NOP => interpreter.move_pointer(1),
        Command::ACC => {
            interpreter.add(instruction.code.value);
            interpreter.move_pointer(1);
        }
        Command::JMP => interpreter.move_pointer(instruction.code.value),
    };
}
