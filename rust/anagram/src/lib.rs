use std::collections::{BTreeMap, HashSet};
use unicode_segmentation::UnicodeSegmentation;

fn sorted_key(word: &str) -> String {
    let mut chars: Vec<_> =
        word.to_lowercase().graphemes(true).map(|g| g.to_string()).collect();

    chars.sort_unstable();
    chars.join("")
}

pub fn anagrams_for<'a>(
    word: &'a str,
    possible_anagrams: &'a [&str],
) -> HashSet<&'a str> {
    let target_key = sorted_key(word);
    let lowercased_word = word.to_lowercase();

    let mut anagram_map: BTreeMap<_, Vec<&str>> = BTreeMap::new();

    for &candidate in possible_anagrams {
        // Ignore words that are the same as the input word
        if candidate.to_lowercase() == lowercased_word {
            continue;
        }

        anagram_map.entry(sorted_key(candidate)).or_default().push(candidate);
    }

    HashSet::from_iter(
        anagram_map.get(&target_key).cloned().unwrap_or_default(),
    )
}
