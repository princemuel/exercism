use ::readmes::{args, readme, scanner};
use ::std::fs;
use args::Args;
use readme::ReadmeGenerator;
use scanner::scan_exercism_directories;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let generator = ReadmeGenerator::new()?;

    println!(
        "Scanning for Exercism exercises in: {}",
        args.path.display()
    );

    let exercises = scan_exercism_directories(&args.path, None)?;

    if exercises.is_empty() {
        println!("No Exercism exercises found!");
        return Ok(());
    }

    let track_stats = generator.generate_track_readmes(&exercises, &args.path)?;
    let main_readme = generator.generate_main_readme(&exercises, &track_stats)?;
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
