use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Hash>(first: &[T], second: &[T]) -> Comparison {
    use Comparison::*;

    match (first, second) {
        _ if first == second => Equal,
        _ if is_sublist(second, first) => Superlist,
        _ if is_sublist(first, second) => Sublist,
        _ => Unequal,
    }
}

/// Uses Rabin-Karp-like rolling hash to check if `a` is a sublist of `b`.
fn is_sublist<T: PartialEq + Hash>(a: &[T], b: &[T]) -> bool {
    if a.is_empty() {
        return true;
    }

    let x = a.len();
    let y = b.len();

    if x > y {
        return false;
    }

    let hash_a = compute_hash(a);
    b.windows(x).map(compute_hash).any(|hash| hash == hash_a)
}

/// Computes the hash of a slice using Rust's default hasher.
fn compute_hash<T: Hash>(slice: &[T]) -> u64 {
    let mut hasher = DefaultHasher::new();
    slice.hash(&mut hasher);
    hasher.finish()
}
