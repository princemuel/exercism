use ::std::error::Error;
use ::std::fs;
use ::std::path::Path;

use ::serde::de::DeserializeOwned;
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

#[derive(Debug, Clone)]
pub struct TrackStats {
	pub exercise_count: usize,
	pub last_updated: String,
	pub track_slug: String,
	pub track_url: String,
	pub metadata: Option<TrackMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackMetadata {
	pub title: String,
	pub category: String,
	pub total: usize,
	pub completed: usize,
}

pub trait Parsable: Sized {
	fn parse(path: &Path) -> Result<Self, Box<dyn Error>>;
}

impl<T> Parsable for T
where
	T: DeserializeOwned,
{
	fn parse(path: &Path) -> Result<Self, Box<dyn Error>> {
		let content = fs::read_to_string(path)?;
		let parsed = serde_json::from_str(&content)?;
		Ok(parsed)
	}
}
