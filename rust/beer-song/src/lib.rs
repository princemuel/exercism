pub fn verse(n: u32) -> String {
    match n {
        n @ 0 => format!(
            "No more {0} of beer on the wall, no more {0} of beer.\n\
            Go to the store and buy some more, 99 bottles of beer on the wall.",
            get_bottle_word(n),
        ),
        n @ 1 => format!(
            "{n} {0} of beer on the wall, {n} {0} of beer.\n\
            Take it down and pass it around, no more {1} of beer on the wall.",
            get_bottle_word(n),
            get_bottle_word(n - 1)
        ),
        n @ 2 => format!(
            "{n} {1} of beer on the wall, {n} {1} of beer.\n\
            Take one down and pass it around, {0} {2} of beer on the wall.",
            n - 1,
            get_bottle_word(n),
            get_bottle_word(n - 1)
        ),
        _ => format!(
            "{n} {1} of beer on the wall, {n} {1} of beer.\n\
            Take one down and pass it around, {0} {2} of beer on the wall.",
            n - 1,
            get_bottle_word(n),
            get_bottle_word(n - 1)
        ),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(verse).collect::<Vec<_>>().join("\n\n")
}

fn get_bottle_word(n: u32) -> &'static str {
    if n == 1 { "bottle" } else { "bottles" }
}
