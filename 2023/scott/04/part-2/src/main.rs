use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::task::Wake;
use std::time::Instant;

#[derive(Clone)]
struct Card {
    contents: String,
    amount: i32,
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Error reading file");

    let start = Instant::now();

    let mut available_cards: BTreeMap<usize, Card> =
        BTreeMap::from_iter(file.lines().enumerate().map(|(idx, card)| {
            (
                idx,
                Card {
                    contents: card.to_string(),
                    amount: 1,
                },
            )
        }));

    let cloned_cards = available_cards.clone();

    cloned_cards.keys().for_each(|key| {
        let cloned_cards_again = available_cards.clone();

        let card_to_check = cloned_cards_again
            .get(&key)
            .expect("Couldn't find any cards for that key");

        let (winning_numbers, your_numbers) = get_both_numbers(&card_to_check.contents);

        let total_winnings = your_numbers.len() - (&your_numbers - &winning_numbers).len();

        // If we don't win, don't do anything
        if total_winnings == 0 {
            return;
        }

        // Otherwise, add more cards to the appropriate keys
        for i in key + 1..key + 1 + total_winnings {
            let current_card = available_cards.get(&i).unwrap().clone();

            available_cards.insert(
                i,
                Card {
                    contents: current_card.contents,
                    amount: current_card.amount + card_to_check.amount,
                },
            );
        }
    });

    let total_cards: i32 = available_cards.values().map(|card| card.amount).sum();

    let total_time = Instant::now() - start;

    println!("Total cards: {total_cards}");
    println!("Took: {total_time:?}");
}

fn get_both_numbers(card: &String) -> (HashSet<i32>, HashSet<i32>) {
    let both_numbers = card.split(":").collect::<Vec<&str>>()[1]
        .split("|")
        .collect::<Vec<&str>>();

    let winning_numbers: HashSet<i32> = HashSet::from_iter(
        both_numbers
            .get(0)
            .expect("Missing winning numbers")
            .trim()
            .split(" ")
            .filter_map(|num| {
                if num.len() == 0 {
                    return None;
                }

                return Some(num.parse::<i32>().expect("Couldn't parse number"));
            }),
    );

    let your_numbers: HashSet<i32> = HashSet::from_iter(
        both_numbers
            .get(1)
            .expect("Missing your numbers")
            .trim()
            .split(" ")
            .filter_map(|num| {
                if num.len() == 0 {
                    return None;
                }

                return Some(num.parse::<i32>().expect("Couldn't parse number"));
            }),
    );

    return (winning_numbers, your_numbers);
}
