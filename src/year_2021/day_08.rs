pub fn part_01(input: &String) -> u64 {
	println!("{}", std::module_path!());
	todo!()
}

pub fn part_02(input: &String) -> u64 {
	todo!()
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	#[should_panic]
	fn part_01() {
		assert_eq!(super::part_01(&read_resource(relative_input_path(INPUT_PATH))), 0);
	}

	#[test]
	#[should_panic]
	fn part_02() {
		assert_eq!(super::part_02(&read_resource(relative_input_path(INPUT_PATH))), 0);
	}
}
