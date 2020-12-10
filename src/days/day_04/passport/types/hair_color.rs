use super::super::digits;

#[derive(Debug)]
pub struct HairColor(String);

impl HairColor {
    pub fn from_str(raw: &str) -> HairColor {
        HairColor(raw.to_string())
    }

    pub fn is_valid(&self) -> bool {
        self.0.starts_with("#") && digits(&self.0[1..], 16).unwrap_or(Vec::new()).len() == 6
    }
}
