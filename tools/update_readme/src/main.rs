use std::fs;

use clap::Parser;
use update_readme::{args, readme, scanner};

use args::Args;
use readme::generate_readme;
use scanner::scan_exercism_directories;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();

	let filter_tracks: Option<Vec<String>> = args
		.tracks
		.as_ref()
		.map(|t| t.split(',').map(|s| s.trim().to_string()).collect());

	println!("Scanning for Exercism exercises in: {}", args.path.display());

	let exercises = scan_exercism_directories(&args.path, filter_tracks.as_ref())?;

	if exercises.is_empty() {
		println!("No Exercism exercises found!");
		return Ok(());
	}

	let readme_content = generate_readme(&exercises);
	fs::write(&args.output, readme_content)?;

	println!(
		"✅ Generated README with {} exercises across {} tracks",
		exercises.values().map(|v| v.len()).sum::<usize>(),
		exercises.len()
	);
	println!("📄 Output written to: {}", args.output.display());

	Ok(())
}
