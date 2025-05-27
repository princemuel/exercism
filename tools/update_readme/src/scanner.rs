use ::std::collections::HashMap;
use ::std::fs;
use ::std::path::Path;

use crate::formatter::{format_exercise_name, format_track_name};
use crate::model::Exercise;

pub fn scan_exercism_directories(
	root: &Path,
	filter_tracks: Option<&Vec<String>>,
) -> Result<HashMap<String, Vec<Exercise>>, Box<dyn std::error::Error>> {
	let mut track_exercises = HashMap::new();

	fn scan_recursive(
		root_path: &Path,
		dir: &Path,
		track_exercises: &mut HashMap<String, Vec<Exercise>>,
		filter_tracks: Option<&Vec<String>>,
	) -> Result<(), Box<dyn std::error::Error>> {
		if !dir.is_dir() {
			return Ok(());
		}

		// Check if this directory has a .exercism/exercise.json file
		let path = dir.join(".exercism").join("exercise.json");

		if path.exists() {
			match Exercise::parse(&path) {
				Ok(exercise) => {
					// Filter by tracks if specified
					if let Some(tracks) = filter_tracks {
						if !tracks.contains(&exercise.track) {
							return Ok(());
						}
					}

					// Calculate relative path from root to this exercise directory
					let relative_path = dir
						.strip_prefix(root_path)
						.unwrap_or(dir)
						.to_string_lossy()
						.replace('\\', "/"); // Ensure forward slashes for markdown links

					let exercise = Exercise {
						name: format_exercise_name(&exercise.name),
						url: exercise.url,
						local_path: format!("./{}/README.md", relative_path),
						track: exercise.track.clone(), // Add track info to exercise
						exercise_slug: exercise.name.clone(), // Add original slug
					};

					track_exercises
						.entry(format_track_name(&exercise.track))
						.or_default()
						.push(exercise);
				},
				Err(e) => {
					eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
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
					scan_recursive(
						root_path, &path, track_exercises, filter_tracks,
					)?;
				}
			}
		}

		Ok(())
	}

	scan_recursive(root, root, &mut track_exercises, filter_tracks)?;

	// Sort exercises within each track
	for exercises in track_exercises.values_mut() {
		exercises.sort_by(|a, b| a.name.cmp(&b.name));
	}

	Ok(track_exercises)
}
