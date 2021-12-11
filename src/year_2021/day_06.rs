/**
 * Attempt 1
 * Naive
 */
// use std::str::FromStr;

// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
// struct Lanternfish {
// 	counter: i32,
// }

// impl Lanternfish {
// 	fn spawn() -> Self {
// 		Self { counter: 8 }
// 	}

// 	fn tick(&mut self) -> bool {
// 		self.counter -= 1;
// 		if self.counter < 0 {
// 			self.counter = 6;
// 			return true;
// 		}
// 		false
// 	}
// }

// impl FromStr for Lanternfish {
// 	type Err = ();

// 	fn from_str(s: &str) -> Result<Self, Self::Err> {
// 		Ok(Self {
// 			counter: s.parse().unwrap(),
// 		})
// 	}
// }

/**
 * Attempt 2
 * Still billions of iterations
 */
// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
// struct Lanternfish {
// 	counter: i32,
// 	days: i32,
// }

// impl Lanternfish {
// 	fn new(counter: i32, days: i32) -> Self {
// 		Self { counter, days }
// 	}

// 	fn spawn(days: i32) -> Self {
// 		Self::new(8, days)
// 	}

// 	fn tick(&mut self) -> Vec<Lanternfish> {
// 		let mut days = self.days - self.counter - 1;
// 		let mut spawn = Vec::new();
// 		if days >= 0 {
// 			spawn.push(Self::spawn(days));
// 			loop {
// 				days -= 7;
// 				if days < 0 {
// 					break;
// 				}
// 				spawn.push(Self::spawn(days));
// 			}
// 		}
// 		spawn
// 	}
// }

/**
 * Attempt 3
 * Sought help - get it now
 * Use an array just to keep track of how many fish there are by number of days until they reproduce
 */
struct LanternfishColony {
	counts: Vec<u64>,
	pregnancy: usize,
}

impl LanternfishColony {
	fn new(counters: Vec<usize>, growth: usize, pregnancy: usize) -> Self {
		let mut counts = vec![0; growth + pregnancy];
		for c in counters {
			counts[c] += 1;
		}
		Self { counts, pregnancy }
	}

	fn spawn(&mut self, days: i32) -> &Self {
		for _ in 0..days {
			self.counts[self.pregnancy] += self.counts[0];
			self.counts.rotate_left(1);
		}
		self
	}

	fn sum(&self) -> u64 {
		self.counts.iter().sum()
	}
}

pub fn part_01(input: &String) -> u64 {
	LanternfishColony::new(input.split(',').map(|f| f.parse::<usize>().unwrap()).collect(), 2, 7)
		.spawn(80)
		.sum()
}

pub fn part_02(input: &String) -> u64 {
	LanternfishColony::new(input.split(',').map(|f| f.parse::<usize>().unwrap()).collect(), 2, 7)
		.spawn(256)
		.sum()
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	#[should_panic]
	fn part_01() {
		assert_eq!(super::part_01(&read_resource(relative_input_path(INPUT_PATH))), 356190);
	}

	#[test]
	#[should_panic]
	fn part_02() {
		assert_eq!(super::part_02(&read_resource(relative_input_path(INPUT_PATH))), 1617359101538);
	}
}
