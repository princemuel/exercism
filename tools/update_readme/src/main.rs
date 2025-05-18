use std::fs;

use update_readme::{args, readme, scanner};

use args::Args;
use readme::generate_readme;
use scanner::scan_exercism_directories;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();

	println!("Scanning for Exercism exercises in: {}", args.path.display());

	let exercises = scan_exercism_directories(&args.path, None)?;

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
