use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
	match list.first() {
		None => String::with_capacity(0),
		Some(word) => list
			.windows(2)
			.map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
			.chain(once(format!("And all for the want of a {}.", word)))
			.collect(),
	}
}
