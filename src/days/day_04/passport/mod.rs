mod types;

use types::{
    BirthYear, CountryId, ExpirationYear, EyeColor, HairColor, Height, IssueYear, PassportId,
};

use super::digits::digits;

#[derive(Default, Debug)]
pub struct Passport {
    birth_year: Option<BirthYear>,
    issue_year: Option<IssueYear>,
    expiration_year: Option<ExpirationYear>,
    height: Option<Height>,
    hair_color: Option<HairColor>,
    eye_color: Option<EyeColor>,
    passport_id: Option<PassportId>,
    country_id: Option<CountryId>,
}

impl Passport {
    pub fn from_str(raw: &str) -> Passport {
        raw.split_whitespace()
            .fold(Default::default(), |mut passport, field| {
                let key_value_pair: Vec<&str> = field.split(':').collect();
                match key_value_pair[0] {
                    "byr" => {
                        passport.birth_year =
                            Some(BirthYear::new(key_value_pair[1].parse::<u32>().unwrap()));
                        passport
                    }
                    "iyr" => {
                        passport.issue_year =
                            Some(IssueYear::new(key_value_pair[1].parse::<u32>().unwrap()));
                        passport
                    }
                    "eyr" => {
                        passport.expiration_year = Some(ExpirationYear::new(
                            key_value_pair[1].parse::<u32>().unwrap(),
                        ));
                        passport
                    }
                    "hgt" => {
                        passport.height = Some(Height::from_str(key_value_pair[1]));
                        passport
                    }
                    "hcl" => {
                        passport.hair_color = Some(HairColor::from_str(key_value_pair[1]));
                        passport
                    }
                    "ecl" => {
                        passport.eye_color = Some(EyeColor::from_str(key_value_pair[1]));
                        passport
                    }
                    "pid" => {
                        passport.passport_id = Some(PassportId::from_str(key_value_pair[1]));
                        passport
                    }
                    "cid" => {
                        passport.country_id = Some(CountryId::from_str(key_value_pair[1]));
                        passport
                    }
                    _ => passport,
                }
            })
    }

    pub fn has_required_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    pub fn is_valid(&self) -> bool {
        self.has_required_fields()
            && self.birth_year.as_ref().unwrap().is_valid()
            && self.issue_year.as_ref().unwrap().is_valid()
            && self.expiration_year.as_ref().unwrap().is_valid()
            && self.height.as_ref().unwrap().is_valid()
            && self.hair_color.as_ref().unwrap().is_valid()
            && self.eye_color.as_ref().unwrap().is_valid()
            && self.passport_id.as_ref().unwrap().is_valid()
    }
}
