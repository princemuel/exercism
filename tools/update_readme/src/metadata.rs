use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExercismMetadata {
	pub track: String,
	pub exercise: String,
	id: String,
	pub url: String,
}

pub fn parse_metadata(
	path: &Path,
) -> Result<ExercismMetadata, Box<dyn std::error::Error>> {
	let content = fs::read_to_string(path)?;

	Ok(serde_json::from_str(&content)?)
}
