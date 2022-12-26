use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::str::FromStr;

use crate::utils::modulo;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Hand {
	Rock,
	Paper,
	Scissors,
}

impl Hand {
	fn play(&self, other: &Self) -> u64 {
		*self as u64
			+ 1 + match self.cmp(other) {
			Ordering::Less => 0,
			Ordering::Equal => 3,
			Ordering::Greater => 6,
		}
	}
}

impl From<i8> for Hand {
	fn from(value: i8) -> Self {
		match modulo(value, 3) {
			0 => Self::Rock,
			1 => Self::Paper,
			2 => Self::Scissors,
			_ => panic!("Impossible"),
		}
	}
}

impl Add<i8> for Hand {
	type Output = Hand;

	fn add(self, rhs: i8) -> Self::Output {
		Self::from((self as i8) + rhs)
	}
}

impl Sub<i8> for Hand {
	type Output = Hand;

	fn sub(self, rhs: i8) -> Self::Output {
		Self::from((self as i8) - rhs)
	}
}

impl FromStr for Hand {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"A" | "X" => Ok(Self::Rock),
			"B" | "Y" => Ok(Self::Paper),
			"C" | "Z" => Ok(Self::Scissors),
			_ => Err("Invalid character".to_string()),
		}
	}
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).expect("partial comparison success")
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(match (self, other) {
			(Self::Paper, Self::Rock) | (Self::Scissors, Self::Paper) | (Self::Rock, Self::Scissors) => Ordering::Greater,
			(Self::Rock, Self::Paper) | (Self::Paper, Self::Scissors) | (Self::Scissors, Self::Rock) => Ordering::Less,
			_ => Ordering::Equal,
		})
	}
}

enum Strategy {
	Win,
	Lose,
	Draw,
}

struct StrategyGuide(Hand, Strategy);

impl FromStr for StrategyGuide {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut guide = s.split_whitespace();
		let opponent_hand: Hand = guide.next().expect("opponent choice").parse().expect("valid opponent choice");
		Ok(StrategyGuide(
			opponent_hand,
			match guide.next().expect("strategy") {
				"X" => Strategy::Lose,
				"Y" => Strategy::Draw,
				"Z" => Strategy::Win,
				_ => panic!("invalid strategy"),
			},
		))
	}
}

pub fn part_01(input: &String) -> u64 {
	input
		.lines()
		.map(|l| {
			l.split_whitespace()
				.take(2)
				.map(|h| h.parse().expect("parse hand"))
				.collect::<Vec<Hand>>()
		})
		.map(|h| h.get(1).expect("player choice").play(&h.get(0).expect("opponent choice")))
		.sum()
}

pub fn part_02(input: &String) -> u64 {
	input
		.lines()
		.map(|l| l.parse::<StrategyGuide>().expect("valid strategy guide"))
		.map(|StrategyGuide(h, s)| {
			match s {
				Strategy::Win => h + 1,
				Strategy::Lose => h - 1,
				Strategy::Draw => h,
			}
			.play(&h)
		})
		.sum()
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			15
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			8933
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			12
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			11998
		);
	}
}
