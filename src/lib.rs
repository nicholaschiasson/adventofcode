pub mod year_2019;
pub mod year_2020;

#[cfg(test)]
mod tests {
	use std::{env, fs};

	fn read_input(year_number: u16, day_number: u8) -> String {
		fs::read_to_string(
			env::current_dir()
				.expect("Failed to get current directory")
				.join(format!("rsrc/{}/input_{:0>2}.txt", year_number, day_number)),
		)
		.expect("Failed to read file")
	}

	#[cfg(test)]
	mod year_2019 {
		const YEAR_NUM: u16 = 2019;

		#[cfg(test)]
		mod day_01 {
			use crate::tests::read_input;
			use crate::year_2019::day_01 as day;
			const DAY_NUM: u8 = 1;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 3393938);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 5088037);
			}
		}
	}

	#[cfg(test)]
	mod year_2020 {
		const YEAR_NUM: u16 = 2020;

		#[cfg(test)]
		mod day_01 {
			use crate::tests::read_input;
			use crate::year_2020::day_01 as day;
			const DAY_NUM: u8 = 1;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 100419);
			}

			#[test]
			fn part_02() {
				assert_eq!(
					day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)),
					265253940
				);
			}
		}

		#[cfg(test)]
		mod day_02 {
			use crate::tests::read_input;
			use crate::year_2020::day_02 as day;
			const DAY_NUM: u8 = 2;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 456);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 308);
			}
		}

		#[cfg(test)]
		mod day_03 {
			use crate::tests::read_input;
			use crate::year_2020::day_03 as day;
			const DAY_NUM: u8 = 3;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 151);
			}

			#[test]
			fn part_02() {
				assert_eq!(
					day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)),
					7540141059
				);
			}
		}

		#[cfg(test)]
		mod day_04 {
			use crate::tests::read_input;
			use crate::year_2020::day_04 as day;
			const DAY_NUM: u8 = 4;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 213);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 147);
			}
		}

		#[cfg(test)]
		mod day_05 {
			use crate::tests::read_input;
			use crate::year_2020::day_05 as day;
			const DAY_NUM: u8 = 5;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 896);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 659);
			}
		}

		#[cfg(test)]
		mod day_06 {
			use crate::tests::read_input;
			use crate::year_2020::day_06 as day;
			const DAY_NUM: u8 = 6;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 6763);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 3512);
			}
		}

		#[cfg(test)]
		mod day_07 {
			use crate::tests::read_input;
			use crate::year_2020::day_07 as day;
			const DAY_NUM: u8 = 7;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 177);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 34988);
			}
		}

		#[cfg(test)]
		mod day_08 {
			use crate::tests::read_input;
			use crate::year_2020::day_08 as day;
			const DAY_NUM: u8 = 8;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 1949);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 2092);
			}
		}

		#[cfg(test)]
		mod day_09 {
			use crate::tests::read_input;
			use crate::year_2020::day_09 as day;
			const DAY_NUM: u8 = 9;

			#[test]
			fn part_01() {
				assert_eq!(
					day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)),
					3199139634
				);
			}

			#[test]
			fn part_02() {
				assert_eq!(
					day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)),
					438559930
				);
			}
		}

		#[cfg(test)]
		mod day_10 {
			use crate::tests::read_input;
			use crate::year_2020::day_10 as day;
			const DAY_NUM: u8 = 10;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 1914);
			}

			#[test]
			fn part_02() {
				assert_eq!(
					day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)),
					9256148959232
				);
			}
		}
	}
}
