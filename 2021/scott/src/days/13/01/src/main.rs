use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
    time::Instant,
};

fn build_path(filename: &str) -> PathBuf {
    // Since we're reading from the build directory, we need to do some
    // footwork to get to the right directory
    let mut cwd = PathBuf::from(&std::env::current_exe().unwrap());

    // Step back three times so that we're at the root of the project
    cwd.pop();
    cwd.pop();
    cwd.pop();

    // Then add the file name so we can reference it
    cwd.push(filename);

    cwd
}

fn get_lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(build_path(filename)).expect("Could not find file.");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| {
            l.expect("Could not read line")
        })
        .collect()
}

#[derive(Debug)]
struct Fold {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Debug, PartialEq)]
struct Dot {
    x: i64,
    y: i64,
}

 trait Dedup<T: PartialEq + Clone> {
  fn clear_duplicates(&mut self);
}

impl<T: PartialEq + Clone> Dedup<T> for Vec<T> {
  fn clear_duplicates(&mut self) {
      let mut already_seen = Vec::new();
      self.retain(|item| match already_seen.contains(item) {
          true => false,
          _ => {
              already_seen.push(item.clone());
              true
          }
      })
  }
}

impl Dot {
    // Returns an new copy of itself with the folded value
    pub fn fold(&self, fold: &Fold) -> Dot {
        let mut new_dot = Dot { x: self.x, y: self.y };
        if fold.x != -1 {
            // We only need to update the Dot if it'sbelow the fold
            if self.x > fold.x {
                new_dot.x = fold.x - (self.x - fold.x);
            }
        } else if fold.y != -1 {
            // We only need to update the Dot if it'sbelow the fold
            if self.y > fold.y {
                new_dot.y = fold.y - (self.y - fold.y);
            }
        }

        new_dot
    }
}

fn get_dots_and_folds(lines: Vec<String>) -> (Vec<Dot>, Vec<Fold>) {
    let mut dots: Vec<Dot> = vec![];
    let mut has_dots = true;
    let mut folds: Vec<Fold> = vec![];

    for line in &lines {
        if line.is_empty() {
            has_dots = false;
            continue;
        }

        if has_dots {
            let dot_coords = line.split(',').map(|c| c.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            dots.push(Dot {
                x: dot_coords[0],
                y: dot_coords[1]
            });
            continue;
        }
        let instruction = line.split(' ').collect::<Vec<&str>>();
        let fold_with_instructions = instruction.last().unwrap().split('=').collect::<Vec<&str>>();
        
        if fold_with_instructions.first().unwrap() == &"y" {
            folds.push(Fold { y:fold_with_instructions.last().unwrap().parse::<i64>().unwrap(), x: -1 });
        }

        if fold_with_instructions.first().unwrap() == &"x" {
            folds.push(Fold { x: fold_with_instructions.last().unwrap().parse::<i64>().unwrap(), y:  -1});
        }
    }

    (dots, folds)
}

fn apply_fold(dots: &[Dot], fold: &Fold) -> Vec<Dot> {
    let mut folded_dots: Vec<Dot> = vec![];
    // First we apply the fold
    for dot in dots {
        folded_dots.push(dot.fold(fold));
    }

    // Then we filter out the ones with duplicates
    folded_dots.clear_duplicates();

    folded_dots
}

fn main() {
    let lines = get_lines_from_file("input.txt");
    let now = Instant::now();

    let (dots, folds) = get_dots_and_folds(lines);
    let folded_dots = apply_fold(&dots, folds.first().unwrap());

    println!("folded_dots: {:?}", folded_dots.len());
    println!(
        "Ran in {}ms, ({}mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
}
