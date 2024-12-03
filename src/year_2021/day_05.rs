use std::{fmt, str::FromStr};

#[derive(Clone, Copy, Debug)]
struct Point(i32, i32);

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{},{}", self.0, self.1)
	}
}

impl FromStr for Point {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let p = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
		Ok(Self(p[0], p[1]))
	}
}

#[derive(Clone, Copy, Debug)]
struct Line(Point, Point);

impl Line {
	fn is_horizontal_or_vertical(&self) -> bool {
		self.0 .0 == self.1 .0 || self.0 .1 == self.1 .1
	}

	fn range(&self) -> Vec<Point> {
		let mut v = vec![self.0];
		while let Some(Point(x, y)) = v.last() {
			if *x == self.1 .0 && *y == self.1 .1 {
				break;
			}
			let mut p = Point(*x, *y);
			if *x != self.1 .0 {
				p.0 += (self.1 .0 - *x).signum();
			}
			if *y != self.1 .1 {
				p.1 += (self.1 .1 - *y).signum();
			}
			v.push(p);
		}
		v
	}
}

impl fmt::Display for Line {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} -> {}", self.0, self.1)
	}
}

impl FromStr for Line {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let p = s.split(" -> ").map(|p| p.parse::<Point>().unwrap()).collect::<Vec<_>>();
		Ok(Self(p[0], p[1]))
	}
}

#[derive(Clone, Debug)]
struct Chart {
	lines: Vec<Line>,
	use_diagonals: bool,
}

impl Chart {
	fn diagram(&self) -> Vec<Vec<i32>> {
		let boundary = 1
			+ self
				.lines
				.iter()
				.fold(0, |m, l| m.max(l.0 .0.max(l.1 .0).max(l.0 .1.max(l.1 .1))));
		let mut points = vec![vec![0; boundary as usize]; boundary as usize];
		for l in &self.lines {
			if self.use_diagonals || l.is_horizontal_or_vertical() {
				for Point(x, y) in l.range() {
					points[y as usize][x as usize] += 1;
				}
			}
		}
		points
	}

	fn num_points_with_danger(&self, danger: i32) -> i32 {
		self.diagram().iter().flatten().filter(|&p| *p >= danger).count() as i32
	}

	fn with_diagonal(&mut self) -> &Self {
		self.use_diagonals = true;
		self
	}
}

impl fmt::Display for Chart {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			self
				.diagram()
				.iter()
				.map(|l| l
					.iter()
					.map(|&p| if p == 0 { String::from(".") } else { p.to_string() })
					.collect::<Vec<_>>()
					.join(""))
				.collect::<Vec<_>>()
				.join("\n")
		)
	}
}

impl FromStr for Chart {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self {
			lines: s.lines().map(|l| l.parse::<Line>().unwrap()).collect::<Vec<_>>(),
			use_diagonals: false,
		})
	}
}

pub fn part_01(input: &String) -> u64 {
	input
		.parse::<Chart>()
		.expect("failed to parse chart of vents")
		.num_points_with_danger(2) as u64
}

pub fn part_02(input: &String) -> u64 {
	input
		.parse::<Chart>()
		.expect("failed to parse chart of vents")
		.with_diagonal()
		.num_points_with_danger(2) as u64
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))),
			5
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{}::final", INPUT_PATH)))),
			5145
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
			16518
		);
	}
}
