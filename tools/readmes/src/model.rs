use ::std::fs;
use ::std::path::Path;

use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Exercise {
	pub name: String,
	pub url: String,
	pub local_path: String,
	pub track: String,
	pub exercise_slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
	pub track: String,
	pub exercise: String,
	id: String,
	pub url: String,
}

impl Metadata {
	pub fn parse(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
		let content = fs::read_to_string(path)?;
		Ok(serde_json::from_str(&content)?)
	}
}
