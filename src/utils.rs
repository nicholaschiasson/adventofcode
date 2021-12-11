use num::Integer;
use std::{env, fs, path::PathBuf};

pub fn modulo<T>(a: T, b: T) -> T
where
	T: Copy + Integer + std::ops::Rem<Output = T>,
{
	((a % b) + b) % b
}

pub fn read_resource(relative_path: PathBuf) -> String {
	println!(
		"{:?}",
		env::current_dir()
			.expect("Failed to get current directory")
			.join("rsrc")
			.join(&relative_path)
	);
	fs::read_to_string(
		env::current_dir()
			.expect("Failed to get current directory")
			.join("rsrc")
			.join(relative_path),
	)
	.expect("Failed to read file")
	.trim()
	.to_string()
}

pub fn relative_input_path(module_path: &str) -> PathBuf {
	module_path
		.split("::")
		.skip(1) // removing leading module name which is crate name
		.collect::<Vec<_>>()
		.split_last() // removing tests module name
		.unwrap()
		.1
		.iter()
		.fold(PathBuf::from("inputs"), |p, s| p.join(s)) // prepend with 'inputs'
		.with_extension("txt")
}
