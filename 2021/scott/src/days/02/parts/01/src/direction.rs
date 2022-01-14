#[derive(Clone, Copy)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

impl Direction {
    pub fn from_str(direction: &str) -> Option<Self> {
        match direction {
            "forward" => Some(Self::Forward),
            "down" => Some(Self::Down),
            "up" => Some(Self::Up),
            _ => None,
        }
    }
}
