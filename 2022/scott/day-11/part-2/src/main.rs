use std::{collections::HashMap, fs, time::Instant};

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn get_last_num_in_str(input: &str) -> i64 {
    input.split(" ").last().unwrap().parse::<i64>().unwrap()
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    pub fn from_str(input: &str) -> Operator {
        match input {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    pub index: usize,
    pub items: Vec<i64>,
    // In the form of [old, operator, modifier]
    pub operation: Vec<String>,
    pub test_value: i64,
    pub if_true: i64,
    pub if_false: i64,
    pub inspect_count: i64,
}

impl Monkey {
    pub fn from_str(parts: Vec<&str>, index: usize) -> Monkey {
        let starting_items = parts[1].split(":").collect::<Vec<&str>>()[1]
            .split(",")
            .map(|item| item.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let operation = parts[2].split("=").collect::<Vec<&str>>()[1]
            .split(" ")
            .map(|part| part.to_string())
            .collect::<Vec<String>>();
        let test_value = get_last_num_in_str(parts[3]);
        let if_true = get_last_num_in_str(parts[4]);
        let if_false = get_last_num_in_str(parts[5]);

        Monkey {
            index,
            items: starting_items,
            operation,
            test_value,
            if_true,
            if_false,
            inspect_count: 0,
        }
    }

    pub fn operate(&self, item: i64) -> i64 {
        // In this context, "old" = item
        let left_side = if self.operation[1] == "old" {
            item
        } else {
            self.operation[1].parse::<i64>().unwrap()
        };
        let operator = Operator::from_str(&self.operation[2]);
        let right_side = if self.operation[3] == "old" {
            item
        } else {
            self.operation[3].parse::<i64>().unwrap()
        };

        match operator {
            Operator::Add => left_side + right_side,
            Operator::Multiply => left_side * right_side,
            Operator::Divide => left_side / right_side,
            Operator::Subtract => left_side - right_side,
        }
    }
}

fn main() {
    let file = get_file_as_string("input.txt");

    let now = Instant::now();

    let mut monkeys = file
        .split("\n\n")
        .enumerate()
        .map(|(idx, line)| {
            return Monkey::from_str(line.split("\n").collect::<Vec<&str>>(), idx);
        })
        .collect::<Vec<Monkey>>();

    let modifier = monkeys
        .iter()
        .map(|monkey| monkey.test_value)
        .reduce(|acc, monkey| {
            return acc * monkey;
        });

    // Iterate for 20 rounds
    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            // Monkeys without items at the point where their turn starts are skipped
            if monkeys[m].items.len() == 0 {
                continue;
            }

            // Mapping between destination the items to put there
            let mut items_to_move: HashMap<i64, Vec<i64>> = HashMap::new();

            for item_index in 0..monkeys[m].items.len() {
                monkeys[m].inspect_count += 1;
                let item = monkeys[m].items[item_index];
                // Operate on the item
                let operated_item = &monkeys[m].operate(item);

                let padded_item = operated_item % modifier.unwrap();

                if padded_item % monkeys[m].test_value == 0 {
                    let if_true = monkeys[m].if_true;
                    let existing_items = items_to_move.get(&if_true);

                    let mut to_update = if existing_items.is_some() {
                        existing_items.unwrap().clone()
                    } else {
                        Vec::new()
                    };

                    to_update.push(padded_item.clone());
                    items_to_move.insert(if_true, to_update);
                } else {
                    let if_false = monkeys[m].if_false;
                    let existing_items = items_to_move.get(&if_false);

                    let mut to_update = if existing_items.is_some() {
                        existing_items.unwrap().clone()
                    } else {
                        Vec::new()
                    };

                    to_update.push(padded_item.clone());
                    items_to_move.insert(if_false, to_update);
                }
            }

            // Update the other monkey's items
            items_to_move.into_iter().for_each(|(key, mut value)| {
                monkeys[key as usize].items.append(&mut value);
            });

            // Once we've gone through all the items, we need to empty this
            // current monkey's items since we're not doing that automatically
            monkeys[m].items = Vec::new();
        }
    }

    monkeys.sort_by(|a, b| a.inspect_count.partial_cmp(&b.inspect_count).unwrap());
    monkeys.reverse();

    let elapsed = now.elapsed();

    println!(
        "Total: {:?}",
        monkeys[0].inspect_count * monkeys[1].inspect_count
    );

    println!("took: {:?}", elapsed);
}
