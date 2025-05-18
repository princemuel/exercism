use std::path::PathBuf;

pub struct Args {
	pub path: PathBuf,
	pub output: PathBuf,
	pub tracks: Option<String>,
}

impl Args {
	pub fn parse() -> Self {
		Args {
			path: PathBuf::from("."),
			output: PathBuf::from("README.md"),
			tracks: None,
		}
	}
}

impl Default for Args {
	fn default() -> Self {
		Self::parse()
	}
}
