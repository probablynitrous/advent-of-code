use std::fs;

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn get_last_num_in_str(input: &str) -> i64 {
    input.split(" ").last().unwrap().parse::<i64>().unwrap()
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Monkey<'operation> {
    pub items: Vec<i64>,
    // In the form of [old, operator, modifier]
    pub operation: Vec<&'operation str>,
    pub test: i64,
    pub if_true: i64,
    pub if_false: i64,
    pub inspect_count: i64,
}

impl<'operation> Monkey<'operation> {
    pub fn from_str(parts: Vec<&str>) -> Monkey {
        let starting_items = parts[1].split(":").collect::<Vec<&str>>()[1]
            .split(",")
            .map(|item| item.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let operation = parts[2].split("=").collect::<Vec<&str>>()[1]
            .split(" ")
            .collect::<Vec<&str>>();
        let test = get_last_num_in_str(parts[3]);
        let if_true = get_last_num_in_str(parts[4]);
        let if_false = get_last_num_in_str(parts[5]);

        Monkey {
            items: starting_items,
            operation,
            test,
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
        let operator = Operator::from_str(self.operation[2]);
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
    let file = get_file_as_string("test-input.txt");

    let mut monkeys = file
        .split("\n\n")
        .map(|line| {
            return Monkey::from_str(line.split("\n").collect::<Vec<&str>>());
        })
        .collect::<Vec<Monkey>>();

    // Iterate for 20 rounds
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            // Monkeys without items at the point where their turn starts are skipped
            if monkeys[m].items.len() == 0 {
                break;
            }

            for item_index in 0..monkeys[m].items.len() {
                monkeys[m].inspect_count += 1;
                let item = monkeys[m].items[item_index];
                // Operate on the item
                let operated_item = &monkeys[m].operate(item);

                // Monkey gets bored
                let with_bored_pad = operated_item / 3;

                if with_bored_pad % monkeys[m].test == 0 {
                    let if_true = monkeys[m].if_true;
                    monkeys[if_true as usize].items.push(with_bored_pad);
                } else {
                    let if_false = monkeys[m].if_false;
                    monkeys[if_false as usize].items.push(with_bored_pad);
                }
            }

            // Once we've gone through all the items, we need to empty this
            // current monkey's items since we're not doing that automatically
            monkeys[m].items = Vec::new();
        }
    }

    println!("monkeys: {:?}", monkeys);
    monkeys.sort_by(|a, b| a.inspect_count.partial_cmp(&b.inspect_count).unwrap());
    monkeys.reverse();

    println!(
        "Total: {:?}",
        monkeys[0].inspect_count * monkeys[1].inspect_count
    );
}
