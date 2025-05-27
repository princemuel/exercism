use ::std::collections::HashMap;
use ::std::env;
use ::std::fs;
use ::std::path::Path;

use ::handlebars::Handlebars;
use ::serde_json::json;
use ::time::OffsetDateTime;
use ::time::format_description::parse;

use crate::model::{Exercise, Parsable, TrackMetadata, TrackStats};

pub struct ReadmeGenerator {
	handlebars: Handlebars<'static>,
}

impl ReadmeGenerator {
	pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
		let mut handlebars = Handlebars::new();

		// Register templates from embedded strings or files
		handlebars.register_template_string(
			"track_readme",
			include_str!("../templates/track_readme.hbs"),
		)?;
		handlebars.register_template_string(
			"main_readme",
			include_str!("../templates/main_readme.hbs"),
		)?;

		// Register helpers
		handlebars.register_helper("progress_bar", Box::new(progress_bar_helper));
		handlebars
			.register_helper("progress_emoji", Box::new(progress_emoji_helper));
		handlebars.register_helper("capitalize", Box::new(capitalize_helper));
		handlebars.register_helper("percentage", Box::new(percentage_helper));

		Ok(Self { handlebars })
	}

	pub fn generate_track_readmes(
		&self,
		exercises_by_track: &HashMap<String, Vec<Exercise>>,
		root_path: &Path,
	) -> Result<HashMap<String, TrackStats>, Box<dyn std::error::Error>> {
		let mut track_stats = HashMap::new();
		let track_metadata = load_track_metadata()?;

		let format =
			parse("[year]-[month]-[day] [hour]:[minute]:[second] UTC").unwrap();
		let timestamp = OffsetDateTime::now_utc().format(&format).unwrap();

		for (track_name, exercises) in exercises_by_track {
			let track_slug =
				exercises.first().map(|e| e.track.clone()).unwrap_or_else(|| {
					track_name.to_lowercase().replace(' ', "-").replace("++", "pp")
				});

			let track_dir = root_path.join(&track_slug);
			if !track_dir.exists() {
				eprintln!(
					"Warning: Track directory '{}' does not exist, skipping",
					track_dir.display()
				);
				continue;
			}

			let metadata = track_metadata.get(&track_slug).cloned();
			let track_data = self
				.prepare_track_data(track_name, exercises, &track_slug, &metadata);

			let content = self.handlebars.render("track_readme", &track_data)?;

			let readme_path = track_dir.join("README.md");
			fs::write(&readme_path, content)?;

			println!(
				"📄 Generated README for {}: {}",
				track_name,
				readme_path.display()
			);

			track_stats.insert(
				track_name.clone(),
				TrackStats {
					exercise_count: exercises.len(),
					last_updated: timestamp.clone(),
					track_slug: track_slug.clone(),
					track_url: format!(
						"https://exercism.org/tracks/{}",
						track_slug
					),
					metadata,
				},
			);
		}

		Ok(track_stats)
	}

	pub fn generate_main_readme(
		&self,
		exercises_by_track: &HashMap<String, Vec<Exercise>>,
		track_stats: &HashMap<String, TrackStats>,
	) -> Result<String, Box<dyn std::error::Error>> {
		let main_data = self.prepare_main_data(exercises_by_track, track_stats);
		Ok(self.handlebars.render("main_readme", &main_data)?)
	}

	fn prepare_track_data(
		&self,
		track_name: &str,
		exercises: &[Exercise],
		track_slug: &str,
		metadata: &Option<TrackMetadata>,
	) -> serde_json::Value {
		let mut exercises_with_paths = Vec::new();

		for exercise in exercises {
			let solution_path =
				if exercise.local_path.starts_with(&format!("./{}/", track_slug)) {
					exercise
						.local_path
						.strip_prefix(&format!("./{}/", track_slug))
						.unwrap_or("README.md")
						.to_string()
				} else {
					format!("{}/README.md", exercise.exercise_slug)
				};

			exercises_with_paths.push(json!({
                "name": exercise.name,
                "url": exercise.url,
                "solution_path": solution_path,
                "is_concept": exercise.name.contains("Concept") || exercise.url.contains("/concept/")
            }));
		}

		let concept_count = exercises_with_paths
			.iter()
			.filter(|e| e["is_concept"].as_bool().unwrap_or(false))
			.count();
		let practice_count = exercises_with_paths.len() - concept_count;

		json!({
			"track_name": track_name,
			"track_slug": track_slug,
			"exercises": exercises_with_paths,
			"exercise_count": exercises.len(),
			"concept_count": concept_count,
			"practice_count": practice_count,
			"has_metadata": metadata.is_some(),
			"metadata": metadata.as_ref().map(|m| json!({
				"category": m.category,
				"total": m.total,
				"completed": m.completed,
				"percentage": (m.completed as f64 / m.total as f64 * 100.0).round() as usize
			}))
		})
	}

	fn prepare_main_data(
		&self,
		exercises_by_track: &HashMap<String, Vec<Exercise>>,
		track_stats: &HashMap<String, TrackStats>,
	) -> serde_json::Value {
		let total_exercises: usize =
			exercises_by_track.values().map(|v| v.len()).sum();
		let total_tracks = exercises_by_track.len();

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

		let mut category_list: Vec<_> = categories
			.into_iter()
			.map(|(name, count)| json!({"name": name, "count": count}))
			.collect();
		category_list.sort_by(|a, b| {
			b["count"].as_u64().unwrap_or(0).cmp(&a["count"].as_u64().unwrap_or(0))
		});

		let mut track_list: Vec<_> = exercises_by_track.keys()
            .map(|track_name| {
                let exercises = &exercises_by_track[track_name];
                let stats = &track_stats[track_name];

                let (category, progress_info) = if let Some(meta) = &stats.metadata {
                    let percentage = (meta.completed as f64 / meta.total as f64 * 100.0).round() as usize;
                    (meta.category.clone(), format!("{}% ({}/{})", percentage, meta.completed, meta.total))
                } else {
                    ("Unknown".to_string(), format!("{} local", exercises.len()))
                };

                json!({
                    "name": track_name,
                    "slug": stats.track_slug,
                    "category": category,
                    "progress_info": progress_info,
                    "exercise_count": exercises.len(),
                    "track_url": stats.track_url,
                    "last_updated": stats.last_updated.split_whitespace().next().unwrap_or("Unknown")
                })
            })
            .collect();
		track_list.sort_by(|a, b| {
			a["name"].as_str().unwrap_or("").cmp(b["name"].as_str().unwrap_or(""))
		});

		json!({
			"total_exercises": total_exercises,
			"total_tracks": total_tracks,
			"total_available": total_available,
			"total_completed": total_completed,
			"overall_completion": if total_available > 0 {
				Some((total_completed as f64 / total_available as f64 * 100.0).round() as usize)
			} else { None },
			"categories": category_list,
			"tracks": track_list
		})
	}
}

// Helper functions for handlebars
fn progress_bar_helper(
	h: &handlebars::Helper,
	_: &Handlebars,
	_: &handlebars::Context,
	_: &mut handlebars::RenderContext,
	out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
	let percentage =
		h.param(0).and_then(|v| v.value().as_u64()).unwrap_or(0) as usize;

	let filled = percentage / 10;
	let empty = 10 - filled;
	let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));

	out.write(&bar)?;
	Ok(())
}

fn progress_emoji_helper(
	h: &handlebars::Helper,
	_: &Handlebars,
	_: &handlebars::Context,
	_: &mut handlebars::RenderContext,
	out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
	let percentage =
		h.param(0).and_then(|v| v.value().as_u64()).unwrap_or(0) as usize;

	let emoji = match percentage {
		0..=10 => "🔴",
		11..=25 => "🟡",
		26..=50 => "🟠",
		51..=75 => "🔵",
		76..=90 => "🟢",
		91..=100 => "🏆",
		_ => "🎯",
	};

	out.write(emoji)?;
	Ok(())
}

fn capitalize_helper(
	h: &handlebars::Helper,
	_: &Handlebars,
	_: &handlebars::Context,
	_: &mut handlebars::RenderContext,
	out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
	if let Some(word) = h.param(0).and_then(|v| v.value().as_str()) {
		let mut chars = word.chars();
		let capitalized = match chars.next() {
			Some(first) => {
				first.to_uppercase().collect::<String>() + chars.as_str()
			},
			None => String::new(),
		};
		out.write(&capitalized)?;
	}
	Ok(())
}

fn percentage_helper(
	h: &handlebars::Helper,
	_: &Handlebars,
	_: &handlebars::Context,
	_: &mut handlebars::RenderContext,
	out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
	let completed = h.param(0).and_then(|v| v.value().as_f64()).unwrap_or(0.0);
	let total = h.param(1).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
	let percentage = (completed / total * 100.0).round() as usize;
	out.write(&percentage.to_string())?;
	Ok(())
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
