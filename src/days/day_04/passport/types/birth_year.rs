use super::super::digits;

#[derive(Debug)]
pub struct BirthYear(u32);

impl BirthYear {
    pub fn new(value: u32) -> BirthYear {
        BirthYear(value)
    }

    pub fn is_valid(&self) -> bool {
        digits(&self.0.to_string(), 10).unwrap().len() == 4 && self.0 >= 1920 && self.0 <= 2002
    }
}
