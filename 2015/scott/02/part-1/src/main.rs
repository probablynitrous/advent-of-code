use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Couldn't read file");

    let wrapping_paper_required: u32 = file
        .split("\n")
        .map(|line| {
            if line.len() == 0 {
                return 0;
            }

            let dimensions = line
                .split("x")
                .map(|dimension| dimension.parse().unwrap())
                .collect::<Vec<u32>>();

            return compute_wrapping_required(
                dimensions.get(0).unwrap(),
                dimensions.get(1).unwrap(),
                dimensions.get(2).unwrap(),
            );
        })
        .sum();

    println!("Total amount: {}", wrapping_paper_required);
}

fn compute_wrapping_required(length: &u32, width: &u32, height: &u32) -> u32 {
    let faces = [length * width, width * height, height * length];

    let buffer = faces
        .iter()
        .min()
        .expect("Shouldn't fail to get smallest face");
    let surface_area: u32 = faces.iter().map(|face| face * 2).sum();

    return surface_area + buffer;
}
