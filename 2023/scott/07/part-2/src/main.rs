use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand<'a> {
    cards: Vec<&'a str>,
    value: u32,
    initial_bet: u32,
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        // If the values are different, we can just get the Ordering of those
        if self.value != other.value {
            return self.value.cmp(&other.value);
        }

        let mut ordering: Option<Ordering> = None;

        // Otherwise, when the hands are valued the same
        self.cards.iter().enumerate().for_each(|(idx, c)| {
            // Skip through in the event that we now have an ordering
            if ordering.is_some() {
                return;
            }

            // Check each of the characters against each other to see
            // who wins
            let self_value = convert_card_to_value(c);
            let other_value =
                convert_card_to_value(other.cards.get(idx).expect("Couldn't find card in other"));

            if self_value != other_value {
                ordering = Some(self_value.cmp(&other_value));
            }
        });

        return ordering.unwrap();
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Couldn't find file");

    let mut hands = build_all_hands(&file);

    // Sorted from last to first (to make it easy to compute the values)
    hands.sort_by(|a, b| a.cmp(&b));

    for hand in hands.iter() {
        println!("Hand: {:?}", hand);
    }

    let total_winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.initial_bet * (idx as u32 + 1))
        .sum();

    println!("total_winnings: {:?}", total_winnings);
}

fn build_all_hands(file: &str) -> Vec<Hand> {
    file.lines()
        .map(|play| {
            let play_split = play.split(" ").collect::<Vec<&str>>();

            let cards = play_split
                .get(0)
                .expect("Couldn't get cards from hand")
                .split("")
                .filter(|c| c.len() > 0)
                .collect::<Vec<&str>>();

            return Hand {
                cards: cards.clone(),
                initial_bet: play_split
                    .get(1)
                    .unwrap()
                    .parse::<u32>()
                    .expect("Couldn't parse bet into integer"),
                value: compute_hand_value(&cards),
            };
        })
        .collect::<Vec<Hand>>()
}

fn compute_hand_value(cards: &Vec<&str>) -> u32 {
    let base_hashset: HashSet<&&str> = HashSet::from_iter(cards.iter());

    // 5 of a kind check to begin with (in the event we have 5 jokers)
    if base_hashset.len() == 1 {
        return 7;
    }

    // Track the number of times each card comes up
    let mut card_occurrence_map: HashMap<&str, u32> = HashMap::new();

    cards.iter().for_each(|card| {
        if let Some(card_count) = card_occurrence_map.get(card) {
            card_occurrence_map.insert(card, card_count + 1);
            return;
        }

        card_occurrence_map.insert(card, 1);
    });

    // if there's a joker in the map, then we want to find the key with the
    // best chance of giving us a better score, bump that, and then remove
    // the J key to ensure the value computation is correct
    if card_occurrence_map
        .clone()
        .keys()
        .find(|&k| k == &"J")
        .is_some()
    {
        let mut highest_occurrence = 0;
        let mut highest_occurrence_key = "Z";

        // Find the key with the highest occurrence
        card_occurrence_map.iter().for_each(|(&k, &v)| {
            // We don't want to find J and map everything into that
            if k == "J" {
                return;
            }

            if v > highest_occurrence {
                highest_occurrence = v;
                highest_occurrence_key = k;
            }
        });

        // get the number of J's - the assumption here is that highest_occurrence_key != "J"
        // otherwise there's pain...
        let cloned_map = card_occurrence_map.clone();
        let total_jokers = cloned_map
            .get("J")
            .expect("Couldn't find jokers when there should've been");

        let _ = card_occurrence_map.remove("J");

        card_occurrence_map
            .entry(highest_occurrence_key)
            .and_modify(|v| *v += *total_jokers);
    }

    let card_set: HashSet<&&str> = HashSet::from_iter(card_occurrence_map.keys());

    // High card
    if card_set.len() == 5 {
        return 1;
    };

    // TODO: clean this up at some point because there are a _lot_ of loops

    // 5 of a kind
    if card_set.len() == 1 {
        return 7;
    }

    if card_set.len() == 2 {
        // If one of the counts is three, then we have a Full house
        if card_occurrence_map.values().find(|&v| v == &3).is_some() {
            return 5;
        }

        // Otherwise, we have a 4 of a kind
        return 6;
    }

    if card_set.len() == 3 {
        // Three of a kind
        if card_occurrence_map.values().find(|&v| v == &3).is_some() {
            return 4;
        }

        // Otherwise, we have a two pair
        return 3;
    }

    // Single pair
    return 2;
}

fn convert_card_to_value(card: &str) -> u32 {
    if let Ok(parsed) = card.parse::<u32>() {
        return parsed;
    }

    return match card {
        "A" => 13,
        "K" => 12,
        "Q" => 11,
        "T" => 10,
        "J" => 1,
        _ => panic!("Discovered a card shouldn't exist"),
    };
}
