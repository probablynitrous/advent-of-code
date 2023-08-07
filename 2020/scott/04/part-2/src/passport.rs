use std::collections::HashMap;

use crate::constrained_value::ConstrainedValue;

#[derive(Debug, PartialEq)]
enum Units {
    CM, // centimeters
    IN, // inches
}

impl Units {
    fn from_str(value: &str) -> Option<Self> {
        return match value {
            "cm" => Some(Units::CM),
            "in" => Some(Units::IN),
            _ => None,
        };
    }
}

struct Height {
    unit: Units,
    value: ConstrainedValue,
}

impl Height {
    fn from(value: &str) -> bool {
        let mut value_str = value
            .chars()
            .rev()
            .take(2)
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        value_str.reverse();
        let unit_result = Units::from_str(&value_str.join(""));

        if unit_result.is_none() {
            return false;
        }

        let unit = unit_result.unwrap();

        let value_str = &value[0..value.len() - 2];

        let (min, max) = if unit == Units::CM {
            (150, 193)
        } else {
            (59, 76)
        };

        let value_result = ConstrainedValue::from(value_str, min, max);

        if value_result.is_none() {
            return false;
        }

        return true;
    }
}

struct HairColor {
    value: String,
}

impl HairColor {
    fn is_valid(value: &str) -> bool {
        if value.len() != 7 {
            return false;
        }

        // I wonder if we can get away without checking the characters for the
        // time being
        if value.chars().collect::<Vec<char>>().first() != Some(&'#') {
            return false;
        }

        return true;
    }
}

enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

impl EyeColor {
    fn from_str(value: &str) -> Option<Self> {
        return match value {
            "amb" => Some(Self::Amber),
            "blu" => Some(Self::Blue),
            "brn" => Some(Self::Brown),
            "gry" => Some(Self::Grey),
            "grn" => Some(Self::Green),
            "hzl" => Some(Self::Hazel),
            "oth" => Some(Self::Other),
            _ => None,
        };
    }
}

struct PassportId {
    value: String,
}

impl PassportId {
    fn is_valid(value: &str) -> bool {
        if value.len() != 9 {
            return false;
        }

        return true;
    }
}

pub struct Passport {
    byr: ConstrainedValue,
    iyr: ConstrainedValue,
    eyr: ConstrainedValue,
    hgt: Height,
    hcl: HairColor,
    ecl: EyeColor,
    pid: PassportId,
    cid: Option<String>, // Doesn't matter if this is None
}

impl Passport {
    pub fn is_valid(line: String) -> bool {
        let values = line.split(" ").collect::<Vec<&str>>();
        let mut passport_values: HashMap<String, String> = HashMap::new();
        values.into_iter().for_each(|v| {
            let split = v.split(":").collect::<Vec<&str>>();
            passport_values.insert(split[0].to_string(), split[1].to_string());
        });

        // This needs a BIG refactor, just trying to get it working at this point
        let byr = if passport_values.get("byr").is_some() {
            ConstrainedValue::from(passport_values.get("byr").unwrap(), 1920, 2002)
        } else {
            None
        };
        if byr.is_none() {
            return false;
        }

        let iyr = if passport_values.get("iyr").is_some() {
            ConstrainedValue::from(passport_values.get("iyr").unwrap(), 2010, 2020)
        } else {
            None
        };
        if iyr.is_none() {
            return false;
        }

        let eyr = if passport_values.get("eyr").is_some() {
            ConstrainedValue::from(passport_values.get("eyr").unwrap(), 2020, 2030)
        } else {
            None
        };
        if eyr.is_none() {
            return false;
        }

        let hgt = if passport_values.get("hgt").is_some() {
            Height::from(passport_values.get("hgt").unwrap())
        } else {
            false
        };
        if !hgt {
            return false;
        }

        let hcl = if passport_values.get("hcl").is_some() {
            HairColor::is_valid(passport_values.get("hcl").unwrap())
        } else {
            false
        };
        if !hcl {
            return false;
        }

        let ecl = if passport_values.get("ecl").is_some() {
            EyeColor::from_str(passport_values.get("ecl").unwrap())
        } else {
            None
        };
        if ecl.is_none() {
            return false;
        }

        let pid = if passport_values.get("pid").is_some() {
            PassportId::is_valid(passport_values.get("pid").unwrap())
        } else {
            false
        };
        if !pid {
            return false;
        }

        return true;
    }
}
