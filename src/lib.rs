fn extract_digits(string: &str) -> Option<Vec<u8>> {
    let mut digits = Vec::new();

    for chr in string.chars() {
        match chr {
            digit if chr.is_ascii_digit() => {
                digits.push(digit.to_digit(10).map(|d| d as u8).expect("Shouldn't fail"))
            }

            '(' | ')' | '-' | '.' | ' ' | '+' => continue,

            _ => return None,
        }
    }

    Some(digits)
}

