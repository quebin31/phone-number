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

fn is_valid_number(digits: &[u8]) -> bool {
    if digits.len() != 10 {
        return false;
    }

    let area_code = &digits[0..3];
    let exch_code = &digits[3..6];

    !(area_code[0] < 2 || exch_code[0] < 2)
}

pub fn number(user_number: &str) -> Option<String> {
    let user_number = user_number.trim();
    if !valid_format(user_number) {
        return None;
    }

    let mut digits = extract_digits(user_number);
    let no_digits = digits.len();

    let valid = match no_digits {
        10 => is_valid_number(&digits),

        11 => {
            if digits[0] != 1 {
                false
            } else {
                digits.remove(0);
                is_valid_number(&digits)
            }
        }

        _ => false,
    };

    if valid {
        Some(digits.into_iter().map(|d| (d + 48) as char).collect())
    } else {
        None
    }
}
