pub fn is_valid(code: &str) -> bool {
    let chars: Vec<char> =
        code.chars().filter(|c| !c.is_whitespace()).collect();

    if chars.len() < 2 {
        return false;
    }

    if !chars.iter().all(char::is_ascii_digit) {
        return false;
    }

    let sum: u32 = chars
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, c)| {
            let mut digit = c.to_digit(10).unwrap();

            if idx % 2 == 1 {
                digit *= 2;

                if digit > 9 {
                    digit -= 9;
                }
            }

            digit
        })
        .sum();

    sum % 10 == 0
}
