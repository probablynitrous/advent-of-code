use std::fs;

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn main() {
    let file = get_file_as_string("input.txt");

    let total_overlaps = file
        .split("\n")
        .map(|line| {
            line.split(",")
                .flat_map(|combo| combo.split("-").flat_map(str::parse::<i64>))
                .collect::<Vec<i64>>()
        })
        .filter(|v| v.len() > 0)
        .map(|quads| {
            let first_min = quads[0];
            let first_max = quads[1];
            let second_min = quads[2];
            let second_max = quads[3];

            if second_max <= first_max && second_max >= first_min {
                return Some(());
            }

            if first_max <= second_max && first_max >= second_min {
                return Some(());
            }

            return None;
        })
        .filter(|v| v.is_some())
        .collect::<Vec<Option<()>>>()
        .len();

    println!("total_overlaps: {:?}", total_overlaps);
}
