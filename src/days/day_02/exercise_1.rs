use super::PasswordWithRestrictions;

pub fn solve(passwords_with_restrictions: &Vec<PasswordWithRestrictions>) {
    let mut valid_passwords: u32 = 0;

    for password_with_restrictions in passwords_with_restrictions.iter() {
        let (min_appearances, max_appearances, letter, password) = password_with_restrictions;

        let mut appearances: u32 = 0;
        let mut valid = true;
        for character in password.chars() {
            if character == *letter {
                appearances += 1;
                if appearances > *max_appearances {
                    valid = false;
                    break;
                }
            }
        }

        valid = valid && appearances >= *min_appearances;

        if valid {
            valid_passwords += 1;
        }
    }

    println!("Solution: {}", valid_passwords);
}
