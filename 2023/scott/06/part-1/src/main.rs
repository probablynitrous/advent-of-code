use std::{collections::BTreeMap, fs, time::Instant};

#[derive(Clone, Copy, Debug)]
struct Game {
    time: f32,
    best_distance: f32,
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Couldn't find file");

    let start = Instant::now();

    let mut games: BTreeMap<usize, Game> = BTreeMap::new();

    file.lines().enumerate().for_each(|(idx, line)| {
        if idx == 0 {
            parse_time(line, &mut games);
        } else if idx == 1 {
            parse_distance(line, &mut games);
        } else {
            todo!("There should only be two lines");
        }
    });

    let product: i32 = games.values().map(|game| get_ways_to_beat(&game)).product();

    let total_time = Instant::now() - start;

    println!("product: {product}");
    println!("Took: {:?}", total_time);
}

fn parse_time(line: &str, games: &mut BTreeMap<usize, Game>) {
    let parts = line.split(":").collect::<Vec<&str>>();
    parts
        .get(1)
        .expect("There should be times in the input")
        .trim()
        .split(" ")
        .filter(|t| t.len() > 0)
        .enumerate()
        .for_each(|(idx, time)| {
            games.insert(
                idx,
                Game {
                    time: time.parse::<f32>().expect("Couldn't parse time into i32"),
                    best_distance: -1.0,
                },
            );
        })
}

fn parse_distance(line: &str, games: &mut BTreeMap<usize, Game>) {
    let parts = line.split(":").collect::<Vec<&str>>();
    parts
        .get(1)
        .expect("There should be distances in the input")
        .trim()
        .split(" ")
        .filter(|d| d.len() > 0)
        .enumerate()
        .for_each(|(idx, distance)| {
            // Assumption here is that we parse the time first
            games.entry(idx).and_modify(|game| {
                game.best_distance = distance
                    .parse::<f32>()
                    .expect("Couldn't parse distance into i32")
            });
        });
}

fn get_ways_to_beat(game: &Game) -> i32 {
    // println!("Best distance: {}", game.best_distance);
    // println!("Total time: {}", game.time);
    //
    // let min_hold_to_beat = (game.best_distance / game.time).ceil();
    // println!("min_hold_to_beat: {min_hold_to_beat}");
    // let max_hold_to_beat = (game.time - (game.best_distance / game.time)).ceil();
    // println!("max_hold_to_beat: {max_hold_to_beat}");
    //
    // let winning_holds = max_hold_to_beat - min_hold_to_beat;
    //
    // for i in min_hold_to_beat as i32..max_hold_to_beat as i32 {
    //     println!("distance achieved: {}", i * (game.time as i32 - i));
    // }
    // println!("winning_holds: {winning_holds}");

    // Quicker way is to compute all the distances, and keep track of those that
    // are greater than what we need

    let min_hold_to_beat = (game.best_distance / game.time).ceil();
    let max_hold_to_beat = (game.time - (game.best_distance / game.time)).ceil();

    let mut winning_holds = 0;

    for i in min_hold_to_beat as i32..max_hold_to_beat as i32 {
        if i * (game.time as i32 - i) > game.best_distance as i32 {
            winning_holds += 1;
        }
    }

    return winning_holds;
}
