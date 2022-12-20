use std::fs;

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn main() {
    let file = get_file_as_string("input.txt");

    let mut register: i64 = 1;
    let mut cycle = 1;
    let mut sum = 0;

    file.lines().for_each(|line| {
        let parts = line.split(" ").collect::<Vec<&str>>();

        // One cycle for processing
        cycle += 1;

        // Check if we've hit an intersting cycle
        if cycle == 20 || (cycle - 20) % 40 == 0 && cycle <= 220 {
            sum += cycle * register;
        }

        // No more cycles for a noop
        if parts[0] == "noop" {
            return;
        }

        // Increment again
        cycle += 1;
        register += parts[1].parse::<i64>().unwrap();

        if cycle == 20 || (cycle - 20) % 40 == 0 && cycle <= 220 {
            sum += cycle * register;
        }
    });

    println!("sum: {:?}", sum);
}
