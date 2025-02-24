use std::collections::{HashMap, HashSet};
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(
    word: &'a str,
    possible_anagrams: &'a [&str],
) -> HashSet<&'a str> {
    let normalized_target = word.to_lowercase();
    let target_signature = char_frequency_map(&normalized_target);

    let mut anagrams = HashSet::new();

    for &candidate in possible_anagrams {
        let normalized_candidate = candidate.to_lowercase();

        if normalized_candidate != normalized_target
            && char_frequency_map(&normalized_candidate) == target_signature
        {
            anagrams.insert(candidate);
        }
    }

    anagrams
}

fn char_frequency_map(word: &str) -> HashMap<String, usize> {
    let mut frequency_map = HashMap::new();

    for grapheme in word.graphemes(true) {
        *frequency_map.entry(grapheme.to_string()).or_insert(0) += 1;
    }

    frequency_map
}
