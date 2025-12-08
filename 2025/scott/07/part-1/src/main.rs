use std::time::Instant;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Beam {
    position: Position,
}

impl Beam {
    fn extend(&mut self) {
        self.position.y += 1;
    }
}

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    let start = Instant::now();

    // Reads beams left-to-right
    let mut beams = vec![Beam {
        position: Position { x: 0, y: 0 },
    }];

    let mut beam_split_count = 0;

    let line_count = file.lines().collect::<Vec<&str>>().len();

    for (y, line) in file.lines().enumerate() {
        let iter_start = Instant::now();
        if y == 0 {
            let starting_point = line
                .chars()
                .position(|c| c == 'S')
                .expect("Failed to find starting position");

            beams.get_mut(0).unwrap().position = Position {
                x: starting_point,
                y: 1,
            };

            continue;
        }

        let mut splitter_x_positions = Vec::new();
        line.chars().enumerate().for_each(|(i, c)| {
            if c == '^' {
                splitter_x_positions.push(i);
            }
        });

        if splitter_x_positions.is_empty() {
            beams.iter_mut().for_each(|b| b.extend());

            continue;
        }

        let mut new_beams = Vec::new();
        for beam in beams {
            if let Some(splitter_pos) = splitter_x_positions
                .iter()
                .find(|pos| pos == &&beam.position.x)
            {
                // Assuming the beams don't reach the end
                new_beams.push(Beam {
                    position: Position {
                        x: splitter_pos - 1,
                        y: beam.position.y,
                    },
                });
                new_beams.push(Beam {
                    position: Position {
                        x: splitter_pos + 1,
                        y: beam.position.y,
                    },
                });

                beam_split_count += 1;
            } else {
                new_beams.push(beam);
            }
        }

        let iter_end = Instant::now() - iter_start;

        println!(
            "line {} of {} complete and took {:?}",
            y + 1,
            line_count,
            iter_end
        );
        beams = new_beams;
    }

    let total_time = Instant::now() - start;

    println!("Beam split {} times", beam_split_count);
    println!("Took {:?}", total_time);
}
