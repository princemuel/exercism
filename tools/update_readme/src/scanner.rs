use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::formatter::{format_exercise_name, format_track_name};
use crate::metadata::parse_metadata;
use crate::model::Exercise;

pub fn scan_exercism_directories(
	root: &Path,
	filter_tracks: Option<&Vec<String>>,
) -> Result<HashMap<String, Vec<Exercise>>, Box<dyn std::error::Error>> {
	let mut exercises_by_track: HashMap<String, Vec<Exercise>> = HashMap::new();

	fn scan_recursive(
		dir: &Path,
		exercises_by_track: &mut HashMap<String, Vec<Exercise>>,
		filter_tracks: Option<&Vec<String>>,
	) -> Result<(), Box<dyn std::error::Error>> {
		if !dir.is_dir() {
			return Ok(());
		}

		// Check if this directory has a .exercism/metadata.json file
		let metadata_path = dir.join(".exercism").join("metadata.json");

		if metadata_path.exists() {
			match parse_metadata(&metadata_path) {
				Ok(metadata) => {
					// Filter by tracks if specified
					if let Some(tracks) = filter_tracks {
						if !tracks.contains(&metadata.track) {
							return Ok(());
						}
					}

					let exercise = Exercise {
						name: format_exercise_name(&metadata.exercise),
						url: metadata.url,
					};

					exercises_by_track
						.entry(format_track_name(&metadata.track))
						.or_default()
						.push(exercise);
				},
				Err(e) => {
					eprintln!(
						"Warning: Failed to parse {}: {}",
						metadata_path.display(),
						e
					);
				},
			}
		}

		// Recursively scan subdirectories
		if let Ok(entries) = fs::read_dir(dir) {
			for entry in entries.flatten() {
				let path = entry.path();
				if path.is_dir()
					&& !path
						.file_name()
						.unwrap_or_default()
						.to_string_lossy()
						.starts_with('.')
				{
					scan_recursive(&path, exercises_by_track, filter_tracks)?;
				}
			}
		}

		Ok(())
	}

	scan_recursive(root, &mut exercises_by_track, filter_tracks)?;

	// Sort exercises within each track
	for exercises in exercises_by_track.values_mut() {
		exercises.sort_by(|a, b| a.name.cmp(&b.name));
	}

	Ok(exercises_by_track)
}
