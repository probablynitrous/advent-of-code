use disjoint::DisjointSet;
use std::time::Instant;

const MAX_ITERATIONS: usize = 1000;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn from_line(line: &str) -> Self {
        let parts = line.split(",").collect::<Vec<&str>>();

        return Self {
            x: parts[0].parse::<i64>().unwrap(),
            y: parts[1].parse::<i64>().unwrap(),
            z: parts[2].parse::<i64>().unwrap(),
        };
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct JunctionBox {
    position: Position,
}

fn main() {
    let file = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    let start = Instant::now();

    let boxes = file
        .lines()
        .map(|line| JunctionBox {
            position: Position::from_line(line),
        })
        .collect::<Vec<JunctionBox>>();

    let mut junctions_and_distances: Vec<((&JunctionBox, &JunctionBox), f64)> = Vec::new();

    for box_1 in boxes.iter() {
        for box_2 in boxes.iter() {
            if box_1 < box_2 {
                // Avoid duplicates using comparison
                junctions_and_distances
                    .push(((box_1, box_2), compute_euclidean_distance(box_1, box_2)));
            }
        }
    }

    junctions_and_distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut circuits = DisjointSet::with_len(boxes.len());
    junctions_and_distances
        .iter()
        .take(MAX_ITERATIONS)
        .for_each(|(junction_boxes, _)| {
            circuits.join(
                boxes
                    .iter()
                    .position(|junction_box| junction_box.position == junction_boxes.0.position)
                    .unwrap(),
                boxes
                    .iter()
                    .position(|junction_box| junction_box.position == junction_boxes.1.position)
                    .unwrap(),
            );
        });

    circuits.sets().sort_by(|a, b| b.len().cmp(&a.len()));
    let mut sets = circuits.sets().clone();
    sets.sort_by(|a, b| b.len().cmp(&a.len()));
    let product = sets
        .iter()
        .take(3)
        .map(|s| u64::try_from(s.len()).unwrap())
        .product::<u64>();

    let total_time = Instant::now() - start;

    println!("Product {}", product);
    println!("Took {:?}", total_time);
}

fn compute_euclidean_distance(box_1: &JunctionBox, box_2: &JunctionBox) -> f64 {
    return (((box_1.position.x - box_2.position.x).pow(2)
        + (box_1.position.y - box_2.position.y).pow(2)
        + (box_1.position.z - box_2.position.z).pow(2)) as f64)
        .sqrt();
}

#[cfg(test)]
mod tests {
    use crate::{JunctionBox, Position, compute_euclidean_distance};

    #[test]
    fn test_compute() {
        assert_eq!(
            compute_euclidean_distance(
                &JunctionBox {
                    position: Position { x: 5, y: 6, z: 2 }
                },
                &JunctionBox {
                    position: Position {
                        x: -7,
                        y: 11,
                        z: -13
                    }
                }
            ),
            19.849433241279208
        );

        assert_eq!(
            compute_euclidean_distance(
                &JunctionBox {
                    position: Position {
                        x: 162,
                        y: 817,
                        z: 812
                    }
                },
                &JunctionBox {
                    position: Position {
                        x: 57,
                        y: 618,
                        z: 57
                    }
                }
            ),
            787.814064357828
        );
    }
}
