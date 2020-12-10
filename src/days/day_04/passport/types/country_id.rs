#[derive(Debug)]
pub struct CountryId(String);

impl CountryId {
    pub fn from_str(value: &str) -> CountryId {
        CountryId(value.to_string())
    }
}
