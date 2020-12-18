pub mod utils;
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
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 265253940);
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
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 7540141059);
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
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 3199139634);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 438559930);
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
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 9256148959232);
			}
		}

		#[cfg(test)]
		mod day_11 {
			use crate::tests::read_input;
			use crate::year_2020::day_11 as day;
			const DAY_NUM: u8 = 11;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 2470);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 2259);
			}
		}

		#[cfg(test)]
		mod day_12 {
			use crate::tests::read_input;
			use crate::year_2020::day_12 as day;
			const DAY_NUM: u8 = 12;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 796);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 39446);
			}
		}

		#[cfg(test)]
		mod day_13 {
			use crate::tests::read_input;
			use crate::year_2020::day_13 as day;
			const DAY_NUM: u8 = 13;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 4808);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 741745043105674);
			}
		}

		#[cfg(test)]
		mod day_14 {
			use crate::tests::read_input;
			use crate::year_2020::day_14 as day;
			const DAY_NUM: u8 = 14;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 8332632930672);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 4753238784664);
			}
		}

		#[cfg(test)]
		mod day_15 {
			use crate::tests::read_input;
			use crate::year_2020::day_15 as day;
			const DAY_NUM: u8 = 15;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&String::from("0,3,6")), 436);
				assert_eq!(day::part_01(&String::from("1,3,2")), 1);
				assert_eq!(day::part_01(&String::from("2,1,3")), 10);
				assert_eq!(day::part_01(&String::from("1,2,3")), 27);
				assert_eq!(day::part_01(&String::from("2,3,1")), 78);
				assert_eq!(day::part_01(&String::from("3,2,1")), 438);
				assert_eq!(day::part_01(&String::from("3,1,2")), 1836);
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 517);
			}

			#[test]
			fn part_02() {
				// // Keep these all commented out since they each take ~50 seconds to finish
				// assert_eq!(day::part_02(&String::from("0,3,6")), 175594);
				// assert_eq!(day::part_02(&String::from("1,3,2")), 2578);
				// assert_eq!(day::part_02(&String::from("2,1,3")), 3544142);
				// assert_eq!(day::part_02(&String::from("1,2,3")), 261214);
				// assert_eq!(day::part_02(&String::from("2,3,1")), 6895259);
				// assert_eq!(day::part_02(&String::from("3,2,1")), 18);
				// assert_eq!(day::part_02(&String::from("3,1,2")), 362);
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 1047739);
			}
		}

		#[cfg(test)]
		mod day_16 {
			use crate::tests::read_input;
			use crate::year_2020::day_16 as day;
			const DAY_NUM: u8 = 16;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 25895);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 5865723727753);
			}
		}

		#[cfg(test)]
		mod day_17 {
			use crate::tests::read_input;
			use crate::year_2020::day_17 as day;
			const DAY_NUM: u8 = 17;

			#[test]
			fn part_01() {
				assert_eq!(day::part_01(&read_input(super::YEAR_NUM, DAY_NUM)), 269);
			}

			#[test]
			fn part_02() {
				assert_eq!(day::part_02(&read_input(super::YEAR_NUM, DAY_NUM)), 1380);
			}
		}
	}
}
