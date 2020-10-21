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

fn is_valid_number(digits: &[u8]) -> bool {
    if digits.len() != 10 {
        return false;
    }

    let area_code = &digits[0..3];
    let exch_code = &digits[3..6];

    !(area_code[0] < 2 || exch_code[0] < 2)
}

pub fn number(user_number: &str) -> Option<String> {
    let mut digits = extract_digits(user_number.trim())?;
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
