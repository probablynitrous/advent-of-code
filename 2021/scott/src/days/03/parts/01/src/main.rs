use std::{path::PathBuf, time::Instant};

fn build_path(filename: &str) -> PathBuf {
    // Since we're reading from the build directory, we need to do some
    // footwork to get to the right directory
    let mut cwd = PathBuf::from(&std::env::current_exe().unwrap());

    // Step back three times so that we're at the root of the project
    cwd.pop();
    cwd.pop();
    cwd.pop();

    // Then add the file name so we can reference it
    cwd.push(filename);

    cwd
}

fn read_file_as_string(filename: &str) -> String {
    std::fs::read_to_string(filename).expect("Couldn't read file into string")
}

fn main() {
    let file = read_file_as_string("input.txt");
    let now = Instant::now();

    let mut grouping: Vec<Vec<String>> = Vec::new();

    file.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(idx, char)| {
            let char_str = char.to_string();
            if grouping.get(idx).is_some() {
                grouping.get_mut(idx).unwrap().push(char.to_string());
            } else {
                grouping.push(vec![char_str]);
            }
        })
    });

    let mut gamma: Vec<String> = Vec::new();
    let mut epsilon: Vec<String> = Vec::new();

    grouping.iter_mut().for_each(|group| {
        group.sort();

        let one_position = group
            .iter()
            .position(|value| value == "1")
            .expect("Couldn't find a 1 in the string");

        // if there are more zeroes than ones
        if one_position > group.len() / 2 {
            // Then add a zero to the gamma rate
            gamma.push("0".to_string());
            // And add a one to the epsilon rate
            epsilon.push("1".to_string());
        } else {
            gamma.push("1".to_string());
            epsilon.push("0".to_string());
        }
    });

    let gamma_rate = isize::from_str_radix(&gamma.join(""), 2)
        .expect("Couldn't parse string binary into decimal");
    let epsilon_rate = isize::from_str_radix(&epsilon.join(""), 2)
        .expect("Couldn't parse string binary into decimal");

    println!("Solution: {}", &gamma_rate * &epsilon_rate);
    println!(
        "Ran in {}ms, ({}mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
}
