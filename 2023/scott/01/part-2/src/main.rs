use regex::{Regex, RegexSet};
use std::{fs, time::Instant};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Match {
    value: String,
    starting_index: usize,
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Couldn't find file");
    let start = Instant::now();

    let patterns = [
        "[0-9]", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let regex_set = RegexSet::new(patterns).expect("Couldn't build regex");
    // Compile individual regexes for extracting matches
    let regexes: Vec<_> = regex_set
        .patterns()
        .iter()
        .map(|pattern| Regex::new(pattern).unwrap())
        .collect();

    let lines = file.lines().collect::<Vec<&str>>();

    let value: u32 = lines
        .into_iter()
        .map(|line| {
            let mut matches = regex_set
                .matches(line)
                .into_iter()
                .map(|index| &regexes[index])
                .flat_map(|value_regex| {
                    // There's a chance we could find more than one here, so
                    // we have to take care of that too
                    return value_regex.captures_iter(line).map(|cap| {
                        let cap_match = cap.get(0).expect("couldn't get capture");
                        Match {
                            value: cap_match.as_str().to_string(),
                            starting_index: cap_match.start(),
                        }
                    });
                })
                .collect::<Vec<Match>>();
            matches.sort_by(|a, b| a.starting_index.cmp(&b.starting_index));

            let first_digit = parse_word(&matches.first().expect("Couldn't get first value").value);
            let last_digit = parse_word(&matches.last().expect("Couldn't get last value").value);

            return first_digit * 10 + last_digit;
        })
        .sum();

    let total_time = Instant::now() - start;

    println!("value: {value}");
    println!("took: {total_time:?}");
}

fn parse_word(word: &str) -> u32 {
    // This is just the individual digit so we can parse and move on
    if word.len() == 1 {
        return word.parse::<u32>().expect("Couldn't parse digit");
    }

    return match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => todo!(),
    };
}
