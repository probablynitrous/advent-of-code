use regex::Regex;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::Instant,
};

fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Could not find file.");
    let buf = BufReader::new(file);
    let no_bags_regex: Regex = Regex::new(r"^.*(no other bags).*$").expect("Could not build regex");

    buf.lines()
        .map(|l| l.expect("Could not parse line."))
        .filter(|l| !no_bags_regex.is_match(l))
        .collect()
}

#[derive(Debug)]
struct ChildBag {
    count: usize,
    bag: Bag,
}

#[derive(Debug)]
struct Bag {
    color: String,
    child_bags: Vec<ChildBag>,
}

// This is horrendously inefficient, but at least it works. I _think_ it's 2n^3 which is quiet the
// achievement. Don't think I've written anything less efficient.
// A better method would be to implement a linked list or some sort of proper tree
// I might come back at some point and fix this. But it's as presented for the time being.
fn main() {
    let lines = get_lines_from_file("../input.txt");
    let mut bags: Vec<Bag> = vec![];
    let now = Instant::now();
    for line in lines {
        let bag_rules: Vec<&str> = line.split("contain").collect::<Vec<&str>>();
        let container_bag_bags = bag_rules[0].trim().clone().to_owned();
        let container_bag: Vec<&str> = container_bag_bags.split("bags").collect::<Vec<&str>>();
        let container = container_bag[0].trim().clone().to_owned();

        let mut top_bag = Bag {
            color: container,
            child_bags: vec![],
        };

        let children_bags_bags = bag_rules[1].trim().clone().to_owned();
        let children_bags: Vec<&str>;
        // Multiple child bags
        if children_bags_bags.contains(",") {
            children_bags = children_bags_bags.split(",").collect::<Vec<&str>>();
            for child in children_bags {
                let count_and_color = child.split(" ").collect::<Vec<&str>>();
                let count: String;
                let color_phrase: String;
                if count_and_color[0] != "" {
                    count = count_and_color[0].to_owned();
                    color_phrase = count_and_color[1..].join(" ");
                } else {
                    count = count_and_color[1].to_owned();
                    color_phrase = count_and_color[2..].join(" ");
                }

                // To isolate color
                let color_phrase_split = color_phrase.split(" ").collect::<Vec<&str>>();
                let color = color_phrase_split[0..color_phrase_split.len() - 1].join(" ");

                let temp_bag = Bag {
                    color,
                    child_bags: vec![],
                };

                top_bag.child_bags.push(ChildBag {
                    bag: temp_bag,
                    count: count.parse::<usize>().expect("Could not parse count"),
                });
            }
        }
        // Just one child bag
        else {
            children_bags = children_bags_bags.split(".").collect::<Vec<&str>>();
            let count_and_color = children_bags[0].split(" ").collect::<Vec<&str>>();
            // First element in the array is the count, the rest is the colour
            let count = count_and_color[0].to_owned();
            let color = count_and_color[1..count_and_color.len() - 1].join(" ");
            let temp_bag = Bag {
                color,
                child_bags: vec![],
            };

            top_bag.child_bags.push(ChildBag {
                bag: temp_bag,
                count: count.parse::<usize>().expect("Could not parse count"),
            });
        }

        bags.push(top_bag);
    }

    let mut count: usize = 0;

    let gold_bag = bags
        .iter()
        .find(|&bag| &bag.color == "shiny gold")
        .expect("Couldn't find child bag")
        .to_owned();

    for child_bag in &gold_bag.child_bags {
        count += child_bag.count * get_child_bag_count(child_bag, &bags);
    }
    println!(
        "Ran in {}ms ({}ns)",
        now.elapsed().as_millis(),
        now.elapsed().as_nanos()
    );
    println!("count: {}", count);
}

fn get_child_bag_count(bag_as_child: &ChildBag, bags: &Vec<Bag>) -> usize {
    let color = &bag_as_child.bag.color;
    let bag_optional = bags.iter().find(|&b| &b.color == color);

    let bag: &Bag;

    match bag_optional {
        Some(b) => bag = b,
        None => return 1,
    }

    let mut count: usize = 1;

    for child_bag in &bag.child_bags {
        count += child_bag.count * get_child_bag_count(&child_bag, bags)
    }

    return count;
}
