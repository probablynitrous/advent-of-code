// Just a u64 with some added constraints
pub struct ConstrainedValue {
    value: u64,
}

impl ConstrainedValue {
    pub fn from(value: &str, min: u64, max: u64) -> Option<ConstrainedValue> {
        let value_result = value.parse();

        if !value_result.is_ok() {
            return None;
        }

        let num_value: u64 = value_result.unwrap();

        if num_value < min || num_value > max {
            return None;
        }

        return Some(ConstrainedValue { value: num_value });
    }
}
