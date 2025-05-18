use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "update_readme")]
#[command(about = "Scans all the exercism directories and generates a README")]
pub struct Args {
	/// Root directory to scan for Exercism exercises
	#[arg(short, long, default_value = ".")]
	pub path: PathBuf,

	/// Output README file path
	#[arg(short, long, default_value = "README.md")]
	pub output: PathBuf,

	/// Only scan specific tracks (comma-separated)
	#[arg(short, long)]
	pub tracks: Option<String>,
}
