pub fn reply(message: &str) -> &'static str {
    let trimmed = message.trim();

    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }

    let mut flags = 0u8;
    const HAS_QUESTION: u8 = 1;
    const HAS_UPPER: u8 = 2;
    const HAS_LOWER: u8 = 4;

    for c in trimmed.chars().rev() {
        match c {
            '?' => flags |= HAS_QUESTION,
            c if c.is_uppercase() => flags |= HAS_UPPER,
            c if c.is_lowercase() => flags |= HAS_LOWER,
            _ => {}
        }

        // Early termination when we have enough info
        if flags & HAS_LOWER != 0 {
            return if flags & HAS_QUESTION != 0 { "Sure." } else { "Whatever." };
        }
    }

    match flags {
        f if f & HAS_QUESTION != 0 && f & HAS_UPPER != 0 => "Calm down, I know what I'm doing!",
        f if f & HAS_QUESTION != 0 => "Sure.",
        f if f & HAS_UPPER != 0 => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
