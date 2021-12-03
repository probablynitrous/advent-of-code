use std::{
    collections::BTreeMap,
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

#[derive(Clone, Debug)]
enum Bit {
    Zero,
    One
}

impl Bit {
    pub fn from_char(value: &char) -> Option<Bit> {
        match value {
            '0' => Some(Bit::Zero),
            '1' => Some(Bit::One),
            _ => None
        }
    }

    pub fn to_str(bit: &Bit) -> &str {
        match bit  {
            Bit::Zero => "0",
            Bit::One => "1"
        }
    }
}

#[derive(Clone, Debug)]
struct BitCount {
    bit: Bit,
    count: i64,
}

#[derive(Debug)]
struct BitAggregate {
    zero: BitCount,
    one: BitCount,
}

impl BitAggregate {
    pub fn add_value(&mut self, value: Bit) {
        match value {
            Bit::Zero => self.zero.count += 1,
            Bit::One => self.one.count += 1,
        }
    }

    pub fn get_most_common(&self) -> &BitCount {
        if &self.zero.count > &self.one.count {
            return &self.zero;
        }

        return &self.one;
    }

    pub fn get_least_common(&self) -> &BitCount {
        if &self.zero.count < &self.one.count {
            return &self.zero;
        }

        return &self.one;
    }
}


fn main() {
    let lines = get_lines_from_file("input.txt");
    let now = Instant::now();

    let mut bits: BTreeMap<i64, BitAggregate> = BTreeMap::new();

    for line in lines {
        let line_bits = line.chars().collect::<Vec<char>>(); 

        for (idx, value) in line_bits.iter().enumerate() {
            if bits.get(&(idx as i64)).is_none() {
                bits.insert(idx.try_into().unwrap(), BitAggregate {
                    zero: BitCount {
                        bit: Bit::Zero,
                        count: 0
                    },
                    one: BitCount {
                        bit: Bit::One,
                        count: 0
                    },
                });
            }

            bits.get_mut(&(idx as i64))
                .unwrap()
                .add_value(
                    Bit::from_char(value).expect("Invalid binary")
                );
        }
    }

    let mut gamma_rate_col: Vec<&str> = vec![];
    let mut epsilon_rate_col: Vec<&str> = vec![];

    for bit_aggregate in bits.values() {
        gamma_rate_col.push(Bit::to_str(&bit_aggregate.get_most_common().bit));
        epsilon_rate_col.push(Bit::to_str(&bit_aggregate.get_least_common().bit));
    }
    
    let gamma_rate = isize::from_str_radix(&gamma_rate_col.join(""), 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon_rate_col.join(""), 2).unwrap();


    println!("Solution: {}", &gamma_rate * &epsilon_rate);
    println!(
        "Ran in {}ms, ({}mic)",
        now.elapsed().as_millis(),
        now.elapsed().as_micros()
    );
}
