use super::super::digits;

#[derive(Debug)]
pub struct PassportId(String);

impl PassportId {
    pub fn from_str(value: &str) -> PassportId {
        PassportId(value.to_string())
    }

    pub fn is_valid(&self) -> bool {
        digits(&self.0, 10).unwrap_or(Vec::new()).len() == 9
    }
}
