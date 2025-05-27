use ::std::collections::HashMap;
use ::std::fs;
use ::std::path::Path;

use ::time::OffsetDateTime;
use ::time::format_description::parse;

use crate::model::Exercise;

#[derive(Debug, Clone)]
pub struct TrackStats {
	pub exercise_count: usize,
	pub last_updated: String,
	pub track_slug: String,
	pub track_url: String,
}

pub fn generate_track_readmes(
	exercises_by_track: &HashMap<String, Vec<Exercise>>,
	root_path: &Path,
) -> Result<HashMap<String, TrackStats>, Box<dyn std::error::Error>> {
	let mut track_stats = HashMap::new();

	let format =
		parse("[year]-[month]-[day] [hour]:[minute]:[second] UTC").unwrap();
	let timestamp = OffsetDateTime::now_utc().format(&format).unwrap();

	for (track_name, exercises) in exercises_by_track {
		// Get the actual track slug from the first exercise's track field
		let track_slug = if let Some(first_exercise) = exercises.first() {
			first_exercise.track.clone()
		} else {
			track_name.to_lowercase().replace(' ', "-").replace("++", "pp")
		};

		let track_dir = root_path.join(&track_slug);

		// Check if track directory exists, if not warn and skip
		if !track_dir.exists() {
			eprintln!(
				"Warning: Track directory '{}' does not exist, skipping README generation",
				track_dir.display()
			);
			continue;
		}

		let track_readme_path = track_dir.join("README.md");
		let track_readme_content =
			generate_track_readme(track_name, exercises, &track_slug);

		fs::write(&track_readme_path, track_readme_content)?;

		println!(
			"📄 Generated README for {}: {}",
			track_name,
			track_readme_path.display()
		);

		// Collect stats for this track
		track_stats.insert(
			track_name.clone(),
			TrackStats {
				exercise_count: exercises.len(),
				last_updated: timestamp.clone(),
				track_slug: track_slug.clone(),
				track_url: format!("https://exercism.org/tracks/{}", track_slug),
			},
		);
	}

	Ok(track_stats)
}

fn generate_track_readme(
	track_name: &str,
	exercises: &[Exercise],
	track_slug: &str,
) -> String {
	let mut content = String::new();

	// Header with track info
	content.push_str(&format!("# {} Solutions\n\n", track_name));

	// Track description and stats
	content.push_str(&format!(
		"My solutions to the [{}](https://exercism.org/tracks/{}) track on Exercism.\n\n",
		track_name, track_slug
	));

	// Statistics section
	content.push_str("## 📊 Statistics\n\n");
	content.push_str(&format!(
		"- **Total Exercises Completed:** {}\n",
		exercises.len()
	));
	content.push_str(&format!(
		"- **Track:** [{}](https://exercism.org/tracks/{})\n",
		track_name, track_slug
	));

	// Get difficulty distribution if available
	let mut concept_exercises = 0;
	let mut practice_exercises = 0;

	for exercise in exercises {
		// This is a simple heuristic - you might want to enhance this based on actual exercise metadata
		if exercise.name.contains("Concept") || exercise.url.contains("/concept/") {
			concept_exercises += 1;
		} else {
			practice_exercises += 1;
		}
	}

	if concept_exercises > 0 {
		content
			.push_str(&format!("- **Concept Exercises:** {}\n", concept_exercises));
	}
	if practice_exercises > 0 {
		content.push_str(&format!(
			"- **Practice Exercises:** {practice_exercises}\n",
		));
	}

	content.push('\n');

	// Exercises table
	content.push_str("## 🏋️ Exercises\n\n");
	content.push_str("| Exercise | Exercism Link | Solution |\n");
	content.push_str("|----------|---------------|----------|\n");

	for exercise in exercises {
		let Exercise { name, url, local_path, .. } = exercise;
		// Convert the local path to be relative from the track directory
		// Remove the track prefix from the path since we're now inside the track dir
		let solution_path = if local_path.starts_with(&format!("./{}/", track_slug))
		{
			local_path
				.strip_prefix(&format!("./{}/", track_slug))
				.unwrap_or("README.md")
		} else {
			// If path doesn't match expected format, just use the exercise name
			&format!("{}/README.md", exercise.exercise_slug)
		};

		content.push_str(&format!(
			"| {} | [View on Exercism]({}) | [Solution]({}) |\n",
			name, url, solution_path
		));
	}

	content.push('\n');

	// Footer
	let format =
		parse("[year]-[month]-[day] [hour]:[minute]:[second] UTC").unwrap();
	let timestamp = OffsetDateTime::now_utc().format(&format).unwrap();

	content.push_str("---\n\n");
	content.push_str("## 🔗 Links\n\n");
	content.push_str("- [Back to Main README](../README.md)\n");
	content.push_str(&format!(
		"- [Exercism {} Track](https://exercism.org/tracks/{})\n",
		track_name, track_slug
	));
	content.push_str(
		"- [My Exercism Profile](https://exercism.org/profiles/princemuel)\n\n",
	);

	content.push_str(&format!("*Last updated: {timestamp}*\n"));

	content
}

pub fn generate_main_readme(
	exercises_by_track: &HashMap<String, Vec<Exercise>>,
	track_stats: &HashMap<String, TrackStats>,
) -> String {
	let mut content = String::new();
	content.push_str("# 🏋️ Exercism Solutions\n\n");

	// Overview section
	let total_exercises: usize = exercises_by_track.values().map(|v| v.len()).sum();
	let total_tracks = exercises_by_track.len();

	content.push_str("Welcome to my collection of solutions for [Exercism](https://exercism.org/) coding exercises!\n\n");

	// Global statistics
	content.push_str("## 📊 Overview\n\n");
	content.push_str(&format!(
		"- **Total Exercises Completed:** {total_exercises}\n",
	));
	content.push_str(&format!("- **Programming Languages:** {}\n", total_tracks));
	content.push_str("- **My Exercism Profile:** [View Profile](https://exercism.org/profiles/princemuel)\n\n");

	// Sort tracks alphabetically
	let mut tracks: Vec<_> = exercises_by_track.keys().collect();
	tracks.sort();

	// Tracks section
	content.push_str("## 🗂️ Tracks\n\n");
	content.push_str("Click on any track to view detailed solutions:\n\n");

	content.push_str("| Track | Exercises | Last Updated | Progress |\n");
	content.push_str("|-------|-----------|--------------|----------|\n");

	for track in &tracks {
		let exercises = &exercises_by_track[*track];
		let stats = &track_stats[*track];
		let track_slug = &stats.track_slug;

		// Create a simple progress bar
		let progress_bar = create_progress_indicator(exercises.len());

		content.push_str(&format!(
			"| [{}]({}/README.md) | {} | {} | {} |\n",
			track,
			track_slug,
			exercises.len(),
			stats.last_updated.split_whitespace().next().unwrap_or("Unknown"), // Just show date
			progress_bar
		));
	}

	content.push_str("\n## 🎯 Quick Stats by Track\n\n");

	content.push_str("| Track | Exercises | Exercism Link | Details |\n");
	content.push_str("|-------|-----------|---------------|----------|\n");

	for track in tracks {
		let exercises = &exercises_by_track[track];
		let stats = &track_stats[track];

		content.push_str(&format!(
			"| [{}]({}/README.md) | {} | [View Track]({}) | [Detailed Stats]({}/README.md) |\n",
			track,
			stats.track_slug,
			exercises.len(),
			stats.track_url,
			stats.track_slug
		));
	}

	// Add some helpful sections
	content.push_str("## 🚀 Getting Started\n\n");
	content.push_str("Each track contains:\n");
	content.push_str("- Individual exercise solutions with explanations\n");
	content.push_str("- Links to the original Exercism problems\n");
	content.push_str("- Track-specific statistics and progress\n\n");

	content.push_str("## 📝 About Exercism\n\n");
	content.push_str("[Exercism](https://exercism.org/) is a free platform for learning programming languages through ");
	content.push_str("practice exercises and mentorship. Each track provides a structured path to learn a language ");
	content.push_str("with both concept exercises (teaching specific concepts) and practice exercises (applying knowledge).\n\n");

	// Footer with generation info
	let format =
		parse("[year]-[month]-[day] [hour]:[minute]:[second] UTC").unwrap();
	let timestamp = OffsetDateTime::now_utc().format(&format).unwrap();

	content.push_str("---\n\n");
	content.push_str(&format!(
		"*This README was automatically generated on {}*\n",
		timestamp
	));
	content.push_str("*To update, run the exercism-readme-generator tool*\n");

	content
}

fn create_progress_indicator(exercise_count: usize) -> String {
	// Simple progress indicator based on exercise count
	match exercise_count {
		0 => "⭕".to_string(),
		1..=5 => "🟡".to_string(),
		6..=15 => "🟠".to_string(),
		16..=30 => "🔵".to_string(),
		31..=50 => "🟢".to_string(),
		_ => "🏆".to_string(),
	}
}
