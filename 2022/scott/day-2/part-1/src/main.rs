use std::fs;

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn get_score_from_selection(selection: &str) -> i64 {
    match selection {
        "A" => 1,
        "X" => 1,
        "B" => 2,
        "Y" => 2,
        "C" => 3,
        "Z" => 3,
        _ => 0,
    }
}

// A = X =  rock = 1
// B = Y =  paper = 2
// C = Z = scissors = 3

// Returns the winner, 1 for the first hand and 2 for the second hand
// Returns the score associated with the win
// Returns -1 for the winner if the result is a tie
fn compare_hands(first_hand: &str, second_hand: &str) -> (i64, i64) {
    if get_score_from_selection(first_hand) == get_score_from_selection(second_hand) {
        return (-1, get_score_from_selection(second_hand));
    }

    if second_hand.eq("X") {
        if first_hand.eq("B") {
            return (1, 1);
        }

        return (2, 1);
    }

    if second_hand.eq("Y") {
        if first_hand.eq("C") {
            return (1, 2);
        }
        return (2, 2);
    }

    if second_hand.eq("Z") {
        if first_hand.eq("A") {
            return (1, 3);
        }
        return (2, 3);
    }

    return (0, 0);
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

            let (winner, winning_score) = compare_hands(hands[0], hands[1]);

            if winner == -1 {
                return winning_score + 3;
            }

            if winner == 1 {
                return winning_score;
            }

            if winner == 2 {
                return winning_score + 6;
            }

            return 0;
        })
        .sum();

    println!("score: {:?}", score);
}
