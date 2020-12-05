use super::PasswordWithRestrictions;

pub fn solve(passwords_with_restrictions: &Vec<PasswordWithRestrictions>) {
    let mut valid_passwords = 0;

    for password_with_restrictions in passwords_with_restrictions.iter() {
        let (first_position, second_position, letter, password) = password_with_restrictions;

        let (first_index, second_index) = (first_position - 1, second_position - 1);

        let mut chars = password.chars();

        let first_char = chars.nth(first_index as usize).unwrap();
        let second_char = chars
            .nth((second_index - first_index - 1) as usize)
            .unwrap();

        if (first_char == *letter) ^ (second_char == *letter) {
            valid_passwords += 1;
        }
    }

    println!("Solution: {}", valid_passwords);
}
