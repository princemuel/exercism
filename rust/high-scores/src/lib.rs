#[derive(Debug)]
pub struct HighScores<'a> {
	scores: &'a [u32],
}

impl<'a> HighScores<'a> {
	pub fn new(scores: &'a [u32]) -> Self {
		Self { scores }
	}

	pub fn scores(&self) -> &[u32] {
		self.scores
	}

	pub fn latest(&self) -> Option<u32> {
		self.scores.last().copied()
	}

	pub fn personal_best(&self) -> Option<u32> {
		self.scores.iter().max().copied()
	}

	pub fn personal_top_three(&self) -> Vec<u32> {
		self.scores
			.iter()
			.fold([u32::MIN; 3], |[a, b, c], &x| match x {
				x if x >= a => [x, a, b],
				x if x >= b => [a, x, b],
				x if x >= c => [a, b, x],
				_ => [a, b, c],
			})
			.iter()
			.take(self.scores.len())
			.copied()
			.collect()
	}
}
