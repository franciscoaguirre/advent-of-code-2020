use super::super::digits;

#[derive(Debug)]
pub struct ExpirationYear(u32);

impl ExpirationYear {
    pub fn new(value: u32) -> ExpirationYear {
        ExpirationYear(value)
    }

    pub fn is_valid(&self) -> bool {
        digits(&self.0.to_string(), 10).unwrap().len() == 4 && self.0 >= 2020 && self.0 <= 2030
    }
}
