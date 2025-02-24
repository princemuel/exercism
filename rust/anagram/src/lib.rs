use std::collections::{HashMap, HashSet};
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(
    word: &'a str,
    possible_anagrams: &'a [&str],
) -> HashSet<&'a str> {
    let target_key = frequency_map(word);
    let lowercased_word = word.to_lowercase();

    let mut anagram_map: HashMap<_, Vec<&str>> = HashMap::new();

    for &candidate in possible_anagrams {
        if candidate.to_lowercase() == lowercased_word {
            continue; // Ignore identical words
        }

        anagram_map
            .entry(frequency_map(candidate))
            .or_default()
            .push(candidate);
    }

    HashSet::from_iter(
        anagram_map.get(&target_key).cloned().unwrap_or_default(),
    )
}

fn frequency_map(word: &str) -> Vec<(String, usize)> {
    let mut freq_map: HashMap<String, usize> = HashMap::new();
    for g in word.to_lowercase().graphemes(true) {
        *freq_map.entry(g.to_string()).or_insert(0) += 1;
    }
    let mut freq_vec: Vec<_> = freq_map.into_iter().collect();
    freq_vec.sort(); // Ensure consistent ordering for hashing
    freq_vec
}
