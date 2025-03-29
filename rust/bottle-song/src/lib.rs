pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..=take_down)
        .map(|i| verse(start_bottles - i))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn verse(n: u32) -> String {
    match n {
        0 => "No green bottles hanging on the wall.".to_string(),
        _ => format!(
            "{0} green {1} hanging on the wall,\n\
             {0} green {1} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {2} green {3} hanging on the wall.",
            number_to_words(n),
            bottle_word(n),
            number_to_words(n - 1),
            bottle_word(n - 1)
        ),
    }
}

fn number_to_words(n: u32) -> String {
    let units = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight",
        "nine",
    ];

    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty",
        "ninety",
    ];

    let word = match n {
        0..=9 => units[n as usize].to_string(),
        10..=19 => teens[(n - 10) as usize].to_string(),
        20..=99 => {
            let t = tens[(n / 10) as usize];
            let u = units[(n % 10) as usize];

            if n % 10 == 0 { t.to_string() } else { format!("{t} {u}") }
        },
        _ => "".to_string(),
    };

    capitalize_first(&word)
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
        None => String::with_capacity(0),
    }
}

fn bottle_word(n: u32) -> &'static str {
    if n == 1 { "bottle" } else { "bottles" }
}
