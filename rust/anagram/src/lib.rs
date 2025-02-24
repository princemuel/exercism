use std::collections::{BTreeMap, HashMap, HashSet};
use unicode_segmentation::UnicodeSegmentation;

fn frequency_map(word: &str) -> BTreeMap<String, usize> {
    let mut freq_map: BTreeMap<String, usize> = BTreeMap::new();

    for g in word.to_lowercase().graphemes(true) {
        *freq_map.entry(g.to_string()).or_insert(0) += 1;
    }

    freq_map
}

pub fn anagrams_for<'a>(
    word: &'a str,
    possible_anagrams: &'a [&str],
) -> HashSet<&'a str> {
    let target_key = frequency_map(word);
    let lowercased_word = word.to_lowercase();

    let mut anagram_map: HashMap<BTreeMap<String, usize>, Vec<&str>> =
        HashMap::new();

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
