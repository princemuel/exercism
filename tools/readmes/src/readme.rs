use ::std::collections::HashMap;
use ::std::env;
use ::std::fs;
use ::std::path::Path;

use ::time::OffsetDateTime;
use ::time::format_description::parse;

use crate::model::{Exercise, Parsable, TrackMetadata, TrackStats};

pub fn generate_track_readmes(
	exercises_by_track: &HashMap<String, Vec<Exercise>>,
	root_path: &Path,
) -> Result<HashMap<String, TrackStats>, Box<dyn std::error::Error>> {
	let mut track_stats = HashMap::new();

	// Load track metadata from tracks.json
	let track_metadata = load_track_metadata()?;

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
		let metadata = track_metadata.get(&track_slug).cloned();
		let track_readme_content =
			generate_track_readme(track_name, exercises, &track_slug, &metadata);

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
				metadata,
			},
		);
	}

	Ok(track_stats)
}

fn load_track_metadata()
-> Result<HashMap<String, TrackMetadata>, Box<dyn std::error::Error>> {
	let home_dir = env::var("HOME").or_else(|_| env::var("USERPROFILE"))?;
	let json_path =
		Path::new(&home_dir).join("exercism").join("database").join("tracks.json");

	if !json_path.exists() {
		eprintln!(
			"Warning: tracks.json not found at {}, proceeding without metadata",
			json_path.display()
		);
		return Ok(HashMap::new());
	}

	let tracks = Vec::<TrackMetadata>::parse(&json_path)?;

	let metadata: HashMap<_, _> =
		tracks.into_iter().map(|track| (track.title.clone(), track)).collect();

	println!(
		"📊 Loaded metadata for {} tracks from {}",
		metadata.len(),
		json_path.display()
	);

	Ok(metadata)
}

fn generate_track_readme(
	track_name: &str,
	exercises: &[Exercise],
	track_slug: &str,
	metadata: &Option<TrackMetadata>,
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

	if let Some(meta) = metadata {
		let completion_percentage =
			(meta.completed as f64 / meta.total as f64 * 100.0).round() as usize;
		let progress_bar = progress_bar(completion_percentage);

		content
			.push_str(&format!("- **Category:** {}\n", capitalize(&meta.category)));
		content.push_str(&format!(
			"- **Total Exercises Available:** {}\n",
			meta.total
		));
		content.push_str(&format!(
			"- **Exercises Completed:** {} / {} ({}%)\n",
			meta.completed, meta.total, completion_percentage
		));
		content.push_str(&format!(
			"- **Progress:** {} {}\n",
			progress_bar,
			progress_emoji(completion_percentage)
		));
		content.push_str(&format!(
			"- **Solutions Found Locally:** {}\n",
			exercises.len()
		));
	} else {
		content.push_str(&format!(
			"- **Solutions Found Locally:** {}\n",
			exercises.len()
		));
	}

	content.push_str(&format!(
		"- **Track:** [{}](https://exercism.org/tracks/{})\n",
		track_name, track_slug
	));

	// Get difficulty distribution
	let mut concept_exercises = 0;
	let mut practice_exercises = 0;

	for exercise in exercises {
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
			"- **Practice Exercises:** {}\n",
			practice_exercises
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
		let solution_path = if local_path.starts_with(&format!("./{}/", track_slug))
		{
			local_path
				.strip_prefix(&format!("./{}/", track_slug))
				.unwrap_or("README.md")
		} else {
			&format!("{}/README.md", exercise.exercise_slug)
		};

		content.push_str(&format!(
			"| {} | [View on Exercism]({}) | [View Solution]({}) |\n",
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

	// Calculate aggregate stats from metadata
	let mut total_available = 0;
	let mut total_completed = 0;
	let mut categories = HashMap::new();

	for stats in track_stats.values() {
		if let Some(meta) = &stats.metadata {
			total_available += meta.total;
			total_completed += meta.completed;
			*categories.entry(meta.category.clone()).or_insert(0) += 1;
		}
	}

	content.push_str("Welcome to my collection of solutions for [Exercism](https://exercism.org/) coding exercises!\n\n");

	// Global statistics with metadata
	content.push_str("## 📊 Overview\n\n");
	content.push_str(&format!(
		"- **Total Exercises Completed:** {}\n",
		total_completed
	));
	content.push_str(&format!(
		"- **Total Exercises Available:** {}\n",
		total_available
	));
	if total_available > 0 {
		let overall_completion = (total_completed as f64 / total_available as f64
			* 100.0)
			.round() as usize;
		content.push_str(&format!(
			"- **Overall Completion:** {}% {}\n",
			overall_completion,
			progress_emoji(overall_completion)
		));
	}
	content.push_str(&format!("- **Programming Languages:** {}\n", total_tracks));
	content
		.push_str(&format!("- **Solutions Found Locally:** {}\n", total_exercises));

	// Category breakdown
	if !categories.is_empty() {
		content.push_str("- **Track Categories:**\n");
		let mut sorted_categories: Vec<_> = categories.iter().collect();
		sorted_categories.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
		for (category, count) in sorted_categories {
			content.push_str(&format!(
				"  - {}: {} tracks\n",
				capitalize(category),
				count
			));
		}
	}

	content.push_str("- **My Exercism Profile:** [View Profile](https://exercism.org/profiles/princemuel)\n\n");

	// Sort tracks alphabetically
	let mut tracks: Vec<_> = exercises_by_track.keys().collect();
	tracks.sort();

	// Tracks section
	content.push_str("## 🗂️ Tracks\n\n");
	content.push_str("Click on any track to view detailed solutions:\n\n");

	content
		.push_str("| Track | Category | Progress | Exercises | Last Updated |\n");
	content.push_str("|-------|----------|----------|-----------|-------------|\n");

	for track in &tracks {
		let exercises = &exercises_by_track[*track];
		let stats = &track_stats[*track];
		let track_slug = &stats.track_slug;

		let (category, progress_info) = if let Some(meta) = &stats.metadata {
			let completion_percentage = (meta.completed as f64 / meta.total as f64
				* 100.0)
				.round() as usize;
			let progress_display = format!(
				"{}% ({}/{})",
				completion_percentage, meta.completed, meta.total
			);

			(capitalize(&meta.category), progress_display)
		} else {
			("Unknown".to_string(), format!("{} local", exercises.len()))
		};

		content.push_str(&format!(
			"| [{}]({}/README.md) | {} | {} | {} | {} |\n",
			track,
			track_slug,
			category,
			progress_info,
			exercises.len(),
			stats.last_updated.split_whitespace().next().unwrap_or("Unknown")
		));
	}

	content.push_str("\n## 🎯 Quick Stats by Track\n\n");

	content.push_str(
		"| Track | Category | Completion | Local Solutions | Exercism Link |\n",
	);
	content.push_str(
		"|-------|----------|------------|-----------------|---------------|\n",
	);

	for track in tracks {
		let exercises = &exercises_by_track[track];
		let stats = &track_stats[track];

		let (category, completion_info) = if let Some(meta) = &stats.metadata {
			let completion_percentage = (meta.completed as f64 / meta.total as f64
				* 100.0)
				.round() as usize;
			let completion_display = format!(
				"{}% ({}/{})",
				completion_percentage, meta.completed, meta.total
			);
			(capitalize(&meta.category), completion_display)
		} else {
			("Unknown".to_string(), "N/A".to_string())
		};

		content.push_str(&format!(
			"| [{}]({}/README.md) | {} | {} | {} | [View Track]({}) |\n",
			track,
			stats.track_slug,
			category,
			completion_info,
			exercises.len(),
			stats.track_url
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

// fn progress_indicator(count: usize) -> String {
// 	match count {
// 		0 => "⭕".to_string(),
// 		1..=5 => "🟡".to_string(),
// 		6..=15 => "🟠".to_string(),
// 		16..=30 => "🔵".to_string(),
// 		31..=50 => "🟢".to_string(),
// 		_ => "🏆".to_string(),
// 	}
// }

fn progress_bar(percentage: usize) -> String {
	let filled = percentage / 10;
	let empty = 10 - filled;
	format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn progress_emoji(percentage: usize) -> &'static str {
	match percentage {
		0..=10 => "🔴",
		11..=25 => "🟡",
		26..=50 => "🟠",
		51..=75 => "🔵",
		76..=90 => "🟢",
		91..=100 => "🏆",
		_ => "🎯",
	}
}

fn capitalize(word: &str) -> String {
	let mut chars = word.chars();
	match chars.next() {
		Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
		None => String::new(),
	}
}
