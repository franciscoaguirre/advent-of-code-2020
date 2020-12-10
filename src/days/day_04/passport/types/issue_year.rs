use super::super::digits;

#[derive(Debug)]
pub struct IssueYear(u32);

impl IssueYear {
    pub fn new(value: u32) -> IssueYear {
        IssueYear(value)
    }

    pub fn is_valid(&self) -> bool {
        digits(&self.0.to_string(), 10).unwrap().len() == 4 && self.0 >= 2010 && self.0 <= 2020
    }
}
