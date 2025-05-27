pub fn format_track_name(track: &str) -> String {
	match track {
		"awk" => "AWK".to_string(),
		"sqlite" => "SQLite".to_string(),
		"vimscript" => "VimScript".to_string(),
		"javascript" => "JavaScript".to_string(),
		"typescript" => "TypeScript".to_string(),
		"cpp" => "C++".to_string(),
		"csharp" => "C#".to_string(),
		"fsharp" => "F#".to_string(),
		"objective-c" => "Objective-C".to_string(),
		"common-lisp" => "Common Lisp".to_string(),
		"visual-basic" => "Visual Basic".to_string(),
		"wasm" => "WebAssembly".to_string(),
		"x86-64-assembly" => "x86-64 Assembly".to_string(),
		_ => {
			// Capitalize first letter and replace hyphens with spaces
			let mut chars: Vec<char> = track.chars().collect();
			if !chars.is_empty() {
				chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
			}
			chars.iter().collect::<String>().replace(['-', '_'], " ")
		},
	}
}

pub fn format_exercise_name(exercise: &str) -> String {
	exercise
		.split('-')
		.map(|word| {
			let mut chars: Vec<char> = word.chars().collect();
			if !chars.is_empty() {
				chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
			}
			chars.iter().collect::<String>()
		})
		.collect::<Vec<String>>()
		.join(" ")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_format_track_name() {
		assert_eq!(format_track_name("javascript"), "JavaScript");
		assert_eq!(format_track_name("typescript"), "TypeScript");
		assert_eq!(format_track_name("rust"), "Rust");
		assert_eq!(format_track_name("common-lisp"), "Common Lisp");
	}

	#[test]
	fn test_format_exercise_name() {
		assert_eq!(format_exercise_name("hello-world"), "Hello World");
		assert_eq!(format_exercise_name("two-fer"), "Two Fer");
		assert_eq!(format_exercise_name("amusement-park"), "Amusement Park");
	}
}
