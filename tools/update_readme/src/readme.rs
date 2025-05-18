use std::collections::HashMap;

use time::{OffsetDateTime, format_description::parse};

use crate::model::Exercise;

pub fn generate_readme(
	exercises_by_track: &HashMap<String, Vec<Exercise>>,
) -> String {
	let mut content = String::new();
	content.push_str("# Exercism Exercises' Solutions\n\n");

	// Sort tracks alphabetically
	let mut tracks: Vec<_> = exercises_by_track.keys().collect();
	tracks.sort();

	for track in tracks {
		let exercises = &exercises_by_track[track];
		content.push_str(&format!("## {}\n\n", track));

		content.push_str(&format!("These are my solutions to the Exercism exercises for the [{}](https://exercism.org/tracks/{}) track\n\n",
                                track,
                                track.to_lowercase().replace(' ', "-").replace("++", "pp")));

		// Generate exercise list with links
		for exercise in exercises {
			content.push_str(&format!("- [{}]({})\n", exercise.name, exercise.url));
		}

		content.push('\n');
	}

	let format =
		parse("[year]-[month]-[day] [hour]:[minute]:[second] UTC").unwrap();
	let timestamp = OffsetDateTime::now_utc().format(&format).unwrap();

	// Add generation timestamp
	content.push_str(&format!("---\n*Generated on {}*\n", timestamp));

	content
}
