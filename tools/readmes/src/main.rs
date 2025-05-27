use ::std::fs;

use readmes::{args, readme, scanner};

use args::Args;
use readme::{generate_main_readme, generate_track_readmes};
use scanner::scan_exercism_directories;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();

	println!("Scanning for Exercism exercises in: {}", args.path.display());

	let exercises = scan_exercism_directories(&args.path, None)?;

	if exercises.is_empty() {
		println!("No Exercism exercises found!");
		return Ok(());
	}

	// Generate individual track README files
	let track_stats = generate_track_readmes(&exercises, &args.path)?;

	// Generate main README with links to track READMEs
	let main_readme = generate_main_readme(&exercises, &track_stats);
	fs::write(&args.output, main_readme)?;

	let total_exercises: usize = exercises.values().map(|v| v.len()).sum();
	println!(
		"✅ Generated main README and {} track READMEs with {} exercises across {} tracks",
		exercises.len(),
		total_exercises,
		exercises.len()
	);
	println!("📄 Main README written to: {}", args.output.display());

	Ok(())
}
