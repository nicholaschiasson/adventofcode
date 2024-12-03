use std::{collections::HashSet, fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Axis {
	Horizontal(i32),
	Vertical(i32),
}

impl FromStr for Axis {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.split_whitespace().last().unwrap().split_once('=').unwrap() {
			("x", x) => Ok(Self::Horizontal(x.parse::<i32>().unwrap())),
			("y", y) => Ok(Self::Vertical(y.parse::<i32>().unwrap())),
			_ => Err(()),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Point(i32, i32);

impl FromStr for Point {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some((x, y)) = s
			.split_once(',')
			.map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
		{
			Ok(Self(x, y))
		} else {
			Err(())
		}
	}
}

#[derive(Clone, Debug)]
struct Paper {
	dots: HashSet<Point>,
}

impl Paper {
	fn fold(&self, axis: &Axis) -> Self {
		Self {
			dots: self
				.dots
				.iter()
				.map(|&d| match (*axis, d) {
					(Axis::Horizontal(x_f), Point(x, y)) => Point(if x > x_f { x - (2 * (x - x_f)) } else { x }, y),
					(Axis::Vertical(y_f), Point(x, y)) => Point(x, if y > y_f { y - (2 * (y - y_f)) } else { y }),
				})
				.collect(),
		}
	}
}

impl Display for Paper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let max_x = self.dots.iter().map(|&Point(x, _)| x).max().unwrap();
		let max_y = self.dots.iter().map(|&Point(_, y)| y).max().unwrap();
		let mut s = String::new();
		for y in 0..=max_y {
			for x in 0..=max_x {
				s.push(if self.dots.contains(&Point(x, y)) { '#' } else { '.' });
			}
			s.push('\n');
		}
		write!(f, "{}", s)
	}
}

impl FromStr for Paper {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self {
			dots: HashSet::<Point>::from_iter(s.lines().map(|l| l.parse::<Point>().expect("failed to parse dot"))),
		})
	}
}

pub fn part_01(input: &String) -> u64 {
	let paper = input.split("\n\n").take(1).next().unwrap().parse::<Paper>().unwrap();
	let folds = input
		.split("\n\n").nth(1)
		.unwrap()
		.lines()
		.map(|l| l.parse::<Axis>().unwrap())
		.collect::<Vec<_>>();
	folds.iter().take(1).fold(paper, |p, f| p.fold(f)).dots.len() as u64
}

pub fn part_02(input: &String) -> u64 {
	let paper = input.split("\n\n").take(1).next().unwrap().parse::<Paper>().unwrap();
	let folds = input
		.split("\n\n").nth(1)
		.unwrap()
		.lines()
		.map(|l| l.parse::<Axis>().unwrap())
		.collect::<Vec<_>>();
	folds.iter().fold(paper, |p, f| p.fold(f)).dots.len() as u64
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			17
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			710
		);
	}

	/**
	 * PART 2 ACTUALLY BREAKS CONVENTION FOR THESE TESTS AND REQUIRES A STRING AS THE ANSWER.
	 * YOU'RE SUPPOSED TO PRINT OUT THE PAPER AFTER COMPLETING ALL THE FOLDS AND READ THE LETTERS DRAWN BY THE DOTS.
	 * SINCE THAT BREAKS MY SYSTEM, AND SINCE I DON'T WANT TO PARSE DOTS INTO LETTERS, WE CAN JUST USE THE COUNT AS THE ANSWER, LIKE PART 1.
	 * THE FINAL ANSWER FOR ME IS 'EPLGRULR'.
	 */
	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			16
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			97
		);
	}
}
