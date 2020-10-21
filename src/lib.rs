fn valid_format(string: &str) -> bool {
    let regex = regex::Regex::new(
        "^(\\+?1 *)?\\(?[2-9][0-9]{2}\\)?(-| *|\\.)[2-9][0-9]{2}(-| *|\\.)[0-9]{4}$",
    )
    .unwrap();

    regex.is_match(string)
}

fn extract_digits(string: &str) -> Vec<u8> {
    string
        .chars()
        .filter_map(|c| {
            if c.is_ascii_digit() {
                c.to_digit(10).map(|d| d as u8)
            } else {
                None
            }
        })
        .collect()
}

pub fn number(user_number: &str) -> Option<String> {
    let user_number = user_number.trim();

    if !valid_format(user_number) {
        None
    } else {
        let digits = extract_digits(user_number);
        let digits_len = digits.len();

        let cleaned = digits
            .into_iter()
            .skip(digits_len - 10)
            .map(|d| (d + 48) as char)
            .collect();

        Some(cleaned)
    }
}
