use super::Passport;

pub fn solve(passports: &Vec<Passport>) {
    let mut valid_passports = 0;

    for passport in passports.iter() {
        if passport.has_required_fields() {
            valid_passports += 1;
        }
    }

    println!("Solution: {}", valid_passports);
}
