use crate::Direction;

#[derive(Copy, Clone)]
pub struct Instruction {
    direction: Direction,
    distance: i64,
}

impl Instruction {
    pub fn new(direction: Direction, distance: i64) -> Instruction {
        Instruction {
            direction,
            distance,
        }
    }
    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn get_distance(&self) -> i64 {
        self.distance
    }
}
