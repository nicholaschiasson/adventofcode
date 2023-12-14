use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
	x: u64,
	y: u64,
}

impl Position {
	fn distance(&self, other: &Self) -> u64 {
		self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
	}
}

impl Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl From<(usize, usize)> for Position {
	fn from((x, y): (usize, usize)) -> Self {
		Self {
			x: x as u64,
			y: y as u64,
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Space {
	Empty,
	Round,
	Cube,
}

impl Display for Space {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Empty => '.',
				Self::Round => 'O',
				Self::Cube => '#',
			}
		)
	}
}

impl From<char> for Space {
	fn from(value: char) -> Self {
		match value {
			'.' => Self::Empty,
			'O' => Self::Round,
			'#' => Self::Cube,
			_ => panic!("oop"),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
	North,
	West,
	South,
	East,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Grid {
	spaces: Vec<Vec<Space>>,
}

impl Grid {
	fn tilt(&mut self, dir: Direction) {
		let mut blocked: Vec<isize> = vec![-1; self.width()];
		for y in 0..self.spaces.len() {
			for x in 0..self.spaces[y].len() {
				match self.spaces[y][x] {
					Space::Round if y as isize > blocked[x] => {
						self.spaces[y][x] = Space::Empty;
						self.spaces[(blocked[x] + 1) as usize][x] = Space::Round;
						blocked[x] += 1;
					},
					Space::Cube => blocked[x] = y as isize,
					_ => (),
				}
			}
		}
	}

	fn height(&self) -> usize {
		self.spaces.len()
	}

	fn width(&self) -> usize {
		self.spaces[0].len()
	}
}

impl Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in &self.spaces {
			for space in row {
				write!(f, "{space}")?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

impl FromStr for Grid {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Grid {
			spaces: s.lines().map(|l| l.chars().map(Space::from).collect()).collect(),
		})
	}
}

pub fn part_01(input: &str) -> u64 {
	let mut grid: Grid = input.parse().unwrap();
	println!("{grid}\n");
	grid.tilt(Direction::North);
	println!("{grid}");
	let grid_height = grid.height();
	grid.spaces.iter().enumerate().fold(0, |load, (i, row)| {
		load + ((grid_height - i) * row.iter().filter(|&&space| space == Space::Round).count()) as u64
	})
}

pub fn part_02(input: &str) -> u64 {
	let mut grid: Grid = input.parse().unwrap();
	println!("{grid}");
	grid.tilt(Direction::North);
	println!("{grid}");
	grid.tilt(Direction::West);
	println!("{grid}");
	grid.tilt(Direction::South);
	println!("{grid}");
	grid.tilt(Direction::East);
	println!("{grid}");
	let grid_height = grid.height();
	grid.spaces.iter().enumerate().fold(0, |load, (i, row)| {
		load + ((grid_height - i) * row.iter().filter(|&&space| space == Space::Round).count()) as u64
	})
}

#[cfg(test)]
mod tests {
	use crate::utils::{read_resource, relative_input_path};

	const INPUT_PATH: &str = module_path!();

	#[test]
	fn part_01() {
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_01")))),
			136
		);
		assert_eq!(
			super::part_01(&read_resource(relative_input_path(&format!("{INPUT_PATH}::final")))),
			110677
		);
	}

	#[test]
	fn part_02() {
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_01")))),
			64
		);
		assert_eq!(
			super::part_02(&read_resource(relative_input_path(&format!("{INPUT_PATH}::final")))),
			1
		);
	}
}
