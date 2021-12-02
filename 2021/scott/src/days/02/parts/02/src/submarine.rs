use crate::Instruction;
use crate::Direction;

pub struct Submarine {
    aim: i64,
    x: i64,
    y: i64,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {
            aim: 0,
            x: 0,
            y: 0
        }
    }

    pub fn travel(&mut self, instruction: Instruction) {
        match instruction.get_direction() {
            Direction::Forward => {
                self.x += instruction.get_distance();
                self.y += self.aim * instruction.get_distance();
            },
            Direction::Up => self.aim -= instruction.get_distance(),
            Direction::Down => self.aim += instruction.get_distance(),
        }
    }

    pub fn get_distance(&self) -> i64 {
        self.x
    }
    
    pub fn get_depth(&self) -> i64 {
        self.y
    }
}
