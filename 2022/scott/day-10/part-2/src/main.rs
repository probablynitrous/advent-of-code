use std::fs;

const LINE_LENGTH: usize = 40;

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn draw_pixel(line: &mut Vec<&str>, cycle: i64, register: i64) {
    let draw_position = cycle % 40 - 1;

    if register == draw_position || register - 1 == draw_position || register + 1 == draw_position {
        line.push("#");
    } else {
        line.push(".");
    }
}

fn main() {
    let file = get_file_as_string("input.txt");

    let mut register: i64 = 1;
    let mut cycle = 0;
    let mut pixels: Vec<&str> = Vec::new();
    let mut screen: Vec<String> = Vec::new();

    file.lines().for_each(|line| {
        let parts = line.split(" ").collect::<Vec<&str>>();
        // Always at least one cycle
        cycle += 1;

        draw_pixel(&mut pixels, cycle, register);

        // Check if we need to flush to the screen
        if pixels.len() == LINE_LENGTH {
            screen.push(pixels.join(""));
            pixels = Vec::new();
        }

        // No more cycles for a noop
        if parts[0] == "noop" {
            return;
        }

        // Increment again for addx
        cycle += 1;
        draw_pixel(&mut pixels, cycle, register);
        register += parts[1].parse::<i64>().unwrap();

        if pixels.len() == LINE_LENGTH {
            screen.push(pixels.join(""));
            pixels = Vec::new();
        }
    });

    screen.into_iter().for_each(|line| {
        println!("{:?}", line);
    });
}
