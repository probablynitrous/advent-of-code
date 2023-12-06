use std::{fs, time::Instant};

#[derive(Clone, Copy, Debug)]
struct Game {
    time: f64,
    best_distance: f64,
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Couldn't find file");

    let start = Instant::now();

    let mut game: Game = Game {
        time: -1.0,
        best_distance: -1.0,
    };

    file.lines().enumerate().for_each(|(idx, line)| {
        if idx == 0 {
            get_race_time(line, &mut game);
        } else if idx == 1 {
            get_race_best_distance(line, &mut game);
        } else {
            todo!("There should only be two lines");
        }
    });

    let ways_to_beat = get_ways_to_beat(&game);

    let total_time = Instant::now() - start;

    println!("ways to beat: {ways_to_beat}");
    println!("Took: {:?}", total_time);
}

fn get_race_time(line: &str, game: &mut Game) {
    let parts = line.split(":").collect::<Vec<&str>>();
    let race_time = parts
        .get(1)
        .expect("There should be times in the input")
        .trim()
        .split(" ")
        .filter(|t| t.len() > 0)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<f64>()
        .expect("Couldn't parse race time");

    game.time = race_time;
}

fn get_race_best_distance(line: &str, game: &mut Game) {
    let parts = line.split(":").collect::<Vec<&str>>();
    let best_distance = parts
        .get(1)
        .expect("There should be distances in the input")
        .trim()
        .split(" ")
        .filter(|d| d.len() > 0)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<f64>()
        .expect("Couldn't parse race best distance");

    game.best_distance = best_distance;
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
        if i as f64 * (game.time - i as f64) > game.best_distance {
            winning_holds += 1;
        }
    }

    return winning_holds;
}
