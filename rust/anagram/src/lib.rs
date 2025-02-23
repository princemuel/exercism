use std::collections::HashSet;

pub fn anagrams_for<'a>(
    word: &str,
    possible_anagrams: &[&str],
) -> HashSet<&'a str> {
    HashSet::from_iter([""])
}
