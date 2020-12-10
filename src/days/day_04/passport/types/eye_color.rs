#[derive(Debug)]
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
    Invalid,
}

impl EyeColor {
    pub fn from_str(raw: &str) -> EyeColor {
        match raw {
            "amb" => EyeColor::Amber,
            "blu" => EyeColor::Blue,
            "brn" => EyeColor::Brown,
            "gry" => EyeColor::Grey,
            "grn" => EyeColor::Green,
            "hzl" => EyeColor::Hazel,
            "oth" => EyeColor::Other,
            _ => EyeColor::Invalid,
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            EyeColor::Invalid => false,
            _ => true,
        }
    }
}
