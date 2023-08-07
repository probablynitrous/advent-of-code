use crate::line::*;

pub struct Toboggan {
    current_pos: usize,
    x_movement: usize,
    trees: usize,
}

impl Toboggan {
    pub fn new(current_pos: usize, x_movement: usize, trees: usize) -> Toboggan {
        Toboggan {
            current_pos,
            x_movement,
            trees,
        }
    }

    pub fn navigate(&mut self, line: &Line) {
        // If the toboggan is going to run off the end of the map, then we need to wrap back around
        // to the start.
        if (self.current_pos + self.x_movement) >= line.get_length() {
            let remainder = line.get_length() - self.current_pos;
            self.update_current_position(self.x_movement - remainder);
        } else {
            // Otherwise, we just increment the toboggan position.
            self.update_current_position(self.current_pos + self.x_movement);
        }
    }

    fn update_current_position(&mut self, new_position: usize) {
        self.current_pos = new_position;
    }

    pub fn will_hit_tree(&self, tile: char) -> bool {
        // Check to see if we've hit a tree.
        if String::from(tile) == "#" {
            return true;
        }

        return false;
    }

    pub fn has_hit_tree(&mut self) {
        self.trees += 1;
    }

    pub fn get_current_pos(&self) -> usize {
        self.current_pos.clone()
    }

    pub fn get_tree_count(&self) -> usize {
        self.trees.clone()
    }
}
