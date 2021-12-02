use crate::Instruction;
use crate::Direction;

pub struct Submarine {
    x: i64,
    y: i64,
}

impl Submarine {
    pub fn new(start_x: i64, start_y: i64) -> Submarine {
        Submarine {
            x: start_x,
            y: start_y
        }
    }

    pub fn travel(&mut self, instruction: Instruction) {
        match instruction.get_direction() {
            Direction::Forward => self.x += instruction.get_distance(),
            Direction::Up => self.y -= instruction.get_distance(),
            Direction::Down => self.y += instruction.get_distance(),
        }
    }

    pub fn get_distance(&self) -> i64 {
        self.x
    }
    
    pub fn get_depth(&self) -> i64 {
        self.y
    }
}
