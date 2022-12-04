use std::fs;

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn get_winning_move(input: &str) -> &str {
    match input {
        "A" => "B",
        "B" => "C",
        "C" => "A",
        _ => "",
    }
}

fn get_losing_move(input: &str) -> &str {
    match input {
        "A" => "C",
        "B" => "A",
        "C" => "B",
        _ => "",
    }
}

fn get_score_from_selection(selection: &str) -> i64 {
    match selection {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0,
    }
}

enum Instruction {
    Lose,
    Draw,
    Win,
}

impl Instruction {
    fn from_string(input: &str) -> Instruction {
        match input {
            "X" => Instruction::Lose,
            "Y" => Instruction::Draw,
            "Z" => Instruction::Win,
            &_ => todo!() // In theory we shouldn't hit this
        }
    }
}

// A = X =  rock = 1
// B = Y =  paper = 2
// C = Z = scissors = 3

// Returns the winner, 1 for the first hand and 2 for the second hand
// Returns the score associated with the win
// Returns -1 for the winner if the result is a tie
fn compute_round_score(first_hand: &str, instruction: &str) -> i64 {
    match Instruction::from_string(instruction) {
        Instruction::Draw => 3 + get_score_from_selection(first_hand),
        Instruction::Lose => 0 + get_score_from_selection(get_losing_move(first_hand)),
        Instruction::Win => 6 + get_score_from_selection(get_winning_move(first_hand))
    }
}

fn main() {
    let file = get_file_as_string("input.txt");

    let score: i64 = file
        .split("\n")
        .map(|line| {
            if line == "" {
                return 0;
            }

            let hands = line.split(" ").collect::<Vec<&str>>();

            return compute_round_score(hands[0], hands[1])
        })
        .sum();

    println!("score: {:?}", score);
}
