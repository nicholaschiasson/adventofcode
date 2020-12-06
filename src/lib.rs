pub mod days;

#[cfg(test)]
mod tests {
	use std::{env, fs};

	fn read_input(day_number: u8) -> String {
		fs::read_to_string(
			env::current_dir()
				.expect("Failed to get current directory")
				.join(format!("rsrc/input_{:0>2}.txt", day_number)),
		)
		.expect("Failed to read file")
	}

	#[cfg(test)]
	mod day_01 {
		use crate::days::day_01 as day;
		const DAY_NUM: u8 = 1;

		#[test]
		fn part_01() {
			assert_eq!(day::part_01(&super::read_input(DAY_NUM)), 100419);
		}

		#[test]
		fn part_02() {
			assert_eq!(day::part_02(&super::read_input(DAY_NUM)), 265253940);
		}
	}

	#[cfg(test)]
	mod day_02 {
		use crate::days::day_02 as day;
		const DAY_NUM: u8 = 2;

		#[test]
		fn part_01() {
			assert_eq!(day::part_01(&super::read_input(DAY_NUM)), 456);
		}

		#[test]
		fn part_02() {
			assert_eq!(day::part_02(&super::read_input(DAY_NUM)), 308);
		}
	}

	#[cfg(test)]
	mod day_03 {
		use crate::days::day_03 as day;
		const DAY_NUM: u8 = 3;

		#[test]
		fn part_01() {
			assert_eq!(day::part_01(&super::read_input(DAY_NUM)), 151);
		}

		#[test]
		fn part_02() {
			assert_eq!(day::part_02(&super::read_input(DAY_NUM)), 7540141059);
		}
	}

	#[cfg(test)]
	mod day_04 {
		use crate::days::day_04 as day;
		const DAY_NUM: u8 = 4;

		#[test]
		fn part_01() {
			assert_eq!(day::part_01(&super::read_input(DAY_NUM)), 213);
		}

		#[test]
		fn part_02() {
			assert_eq!(day::part_02(&super::read_input(DAY_NUM)), 147);
		}
	}

	#[cfg(test)]
	mod day_05 {
		use crate::days::day_05 as day;
		const DAY_NUM: u8 = 5;

		#[test]
		fn part_01() {
			assert_eq!(day::part_01(&super::read_input(DAY_NUM)), 896);
		}

		#[test]
		fn part_02() {
			assert_eq!(day::part_02(&super::read_input(DAY_NUM)), 659);
		}
	}

	#[cfg(test)]
	mod day_06 {
		use crate::days::day_06 as day;
		const DAY_NUM: u8 = 6;

		#[test]
		fn part_01() {
			assert_eq!(day::part_01(&super::read_input(DAY_NUM)), 6763);
		}

		#[test]
		fn part_02() {
			assert_eq!(day::part_02(&super::read_input(DAY_NUM)), 3512);
		}
	}
}
