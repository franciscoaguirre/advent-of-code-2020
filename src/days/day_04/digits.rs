pub fn digits(string: &str, radix: u32) -> Result<Vec<u32>, &'static str> {
    let possible_digits: Vec<Option<u32>> =
        string.chars().map(|digit| digit.to_digit(radix)).collect();
    if possible_digits
        .iter()
        .any(|possible_digit| possible_digit.is_none())
    {
        Err("Could not get digits")
    } else {
        let digits: Vec<u32> = possible_digits
            .iter()
            .map(|possible_digit| possible_digit.unwrap())
            .collect();
        Ok(digits)
    }
}
