pub struct Line {
    tiles: Vec<char>,
    length: usize,
}

impl Line {
    pub fn new(line: String) -> Line {
        Line {
            tiles: line.chars().collect(),
            length: line.chars().count(),
        }
    }
    pub fn get_tiles(&self) -> Vec<char> {
        self.tiles.clone()
    }

    pub fn get_tile_at(&self, index: usize) -> char {
        self.get_tiles()[index]
    }

    pub fn get_length(&self) -> usize {
        self.length
    }
}
