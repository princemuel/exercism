use ::std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let s_str = candidate.to_lowercase().replace(['-', ' '], "");
    let s_set: HashSet<char> = s_str.chars().collect();

    s_str.len() == s_set.len()
}
