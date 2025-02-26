#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T>(first: &[T], second: &[T]) -> Comparison
where
    T: PartialEq,
{
    use Comparison::*;

    let is_sublist = |small: &[T], big: &[T]| {
        big.windows(small.len()).any(|window| window == small)
    };

    match (first, second) {
        _ if first == second => Equal,
        _ if first.is_empty() => Sublist,
        _ if second.is_empty() => Superlist,
        _ if is_sublist(second, first) => Superlist,
        _ if is_sublist(first, second) => Sublist,
        _ => Unequal,
    }
}
