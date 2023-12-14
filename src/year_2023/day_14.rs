use std::collections::HashMap;
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
		match dir {
			Direction::North => {
				for x in 0..self.width() {
					let mut rock: Option<usize> = None;
					for y in 0..self.height() {
						match self.spaces[y][x] {
							Space::Cube => rock = Some(y),
							Space::Round => {
								if let Some(r) = rock.map(|r| r + 1).or(Some(0)) {
									self.spaces[y][x] = Space::Empty;
									self.spaces[r][x] = Space::Round;
									rock = Some(r);
								};
							},
							_ => (),
						}
					}
				}
			},
			Direction::West => {
				for y in 0..self.height() {
					let mut rock: Option<usize> = None;
					for x in 0..self.width() {
						match self.spaces[y][x] {
							Space::Cube => rock = Some(x),
							Space::Round => {
								if let Some(r) = rock.map(|r| r + 1).or(Some(0)) {
									self.spaces[y][x] = Space::Empty;
									self.spaces[y][r] = Space::Round;
									rock = Some(r);
								};
							},
							_ => (),
						}
					}
				}
			},
			Direction::South => {
				for x in 0..self.width() {
					let mut rock: Option<usize> = None;
					for y in (0..self.height()).rev() {
						match self.spaces[y][x] {
							Space::Cube => rock = Some(y),
							Space::Round => {
								if let Some(r) = rock.map(|r| r - 1).or(Some(self.height() - 1)) {
									self.spaces[y][x] = Space::Empty;
									self.spaces[r][x] = Space::Round;
									rock = Some(r);
								};
							},
							_ => (),
						}
					}
				}
			},
			Direction::East => {
				for y in 0..self.height() {
					let mut rock: Option<usize> = None;
					for x in (0..self.width()).rev() {
						match self.spaces[y][x] {
							Space::Cube => rock = Some(x),
							Space::Round => {
								if let Some(r) = rock.map(|r| r - 1).or(Some(self.width() - 1)) {
									self.spaces[y][x] = Space::Empty;
									self.spaces[y][r] = Space::Round;
									rock = Some(r);
								};
							},
							_ => (),
						}
					}
				}
			},
		}
	}

	fn spin_cycle(&mut self) {
		self.tilt(Direction::North);
		self.tilt(Direction::West);
		self.tilt(Direction::South);
		self.tilt(Direction::East);
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
	grid.tilt(Direction::North);
	let grid_height = grid.height();
	grid.spaces.iter().enumerate().fold(0, |load, (i, row)| {
		load + ((grid_height - i) * row.iter().filter(|&&space| space == Space::Round).count()) as u64
	})
}

pub fn part_02(input: &str) -> u64 {
	let mut grid: Grid = input.parse().unwrap();
	let mut grid_set: HashMap<Grid, usize> = HashMap::new();
	let mut cycle_start: usize = 0;
	for i in 0.. {
		if let Some(&j) = grid_set.get(&grid) {
			cycle_start = j;
			break;
		} else {
			grid_set.insert(grid.clone(), i);
		}
		grid.spin_cycle();
	}
	let its = (1000000000 - cycle_start) % (grid_set.len() - cycle_start as usize) + cycle_start;

	let grid = grid_set.iter().find(|&(_, &i)| i == its).map(|(g, _)| g).unwrap();
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
			90551
		);
	}
}
