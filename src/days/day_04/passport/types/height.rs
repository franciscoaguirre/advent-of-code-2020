#[derive(Debug)]
pub struct Height(String);

impl Height {
    pub fn from_str(value: &str) -> Height {
        Height(value.to_string())
    }

    pub fn is_valid(&self) -> bool {
        if self.0.contains("cm") {
            let centimeters = &self.0[..self.0.len() - 2].parse::<u32>().unwrap_or(0);
            centimeters >= &Height::centimeters_valid_range().0
                && centimeters <= &Height::centimeters_valid_range().1
        } else if self.0.contains("in") {
            let inches = &self.0[..self.0.len() - 2].parse::<u32>().unwrap_or(0);
            inches >= &Height::inches_valid_range().0 && inches <= &Height::inches_valid_range().1
        } else {
            false
        }
    }

    fn centimeters_valid_range() -> (u32, u32) {
        (150, 193)
    }

    fn inches_valid_range() -> (u32, u32) {
        (59, 76)
    }
}
