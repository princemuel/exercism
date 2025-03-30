pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .filter_map(|i| Some(generate_verse(start_bottles.checked_sub(i)?)))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn generate_verse(n: u32) -> String {
    let current_word = num_to_word(n);
    let next_num = if n == 1 { 0 } else { n - 1 };
    let next_word = num_to_word(next_num);

    let first =
        format!("{} green {} hanging on the wall,\n", current_word, bottle_word(n));
    let third = "And if one green bottle should accidentally fall,".to_string();
    let fourth = format!(
        "There'll be {} green {} hanging on the wall.",
        next_word.to_lowercase(),
        bottle_word(next_num)
    );

    format!("{}{}\n{}", first.repeat(2), third, fourth)
}

fn num_to_word(n: u32) -> String {
    match n {
        0 => "No".to_string(),
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        10 => "Ten".to_string(),
        _ => panic!("Number out of range"),
    }
}

fn bottle_word(n: u32) -> &'static str {
    if n == 1 { "bottle" } else { "bottles" }
}
