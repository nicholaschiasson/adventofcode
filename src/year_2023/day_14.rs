use std::collections::HashMap;
use std::ops::Range;
use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    x: u64,
    y: u64,
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Space {
    #[default]
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
    fn get_outer_range(&self, dir: Direction) -> Range<usize> {
        match dir {
            Direction::North | Direction::South => 0..self.width(),
            Direction::West | Direction::East => 0..self.height(),
        }
    }

    fn get_inner_range(&self, dir: Direction) -> Vec<usize> {
        match dir {
            Direction::North => (0..self.height()).collect(),
            Direction::West => (0..self.width()).collect(),
            Direction::South => (0..self.height()).rev().collect(),
            Direction::East => (0..self.width()).rev().collect(),
        }
    }

    fn get_position(&self, dir: Direction, i: usize, j: usize) -> (usize, usize) {
        match dir {
            Direction::North | Direction::South => (i, j),
            Direction::West | Direction::East => (j, i),
        }
    }

    fn incr_rock_position(&self, dir: Direction, rock_position: usize) -> usize {
        match dir {
            Direction::North | Direction::West => rock_position + 1,
            Direction::South | Direction::East => rock_position - 1,
        }
    }

    fn get_rock_position(&self, dir: Direction, x: usize, y: usize, r: usize) -> (usize, usize) {
        match dir {
            Direction::North | Direction::South => (x, r),
            Direction::West | Direction::East => (r, y),
        }
    }

    fn tilt(&mut self, dir: Direction) {
        let mut first_index = None;
        for i in self.get_outer_range(dir) {
            let mut rock: Option<usize> = None;
            for j in self.get_inner_range(dir) {
                let (x, y) = self.get_position(dir, i, j);
                match (self.spaces[y][x], first_index.get_or_insert(j)) {
                    (Space::Cube, _) => rock = Some(j),
                    (Space::Round, rock_init) => {
                        if let Some(r) = rock
                            .map(|r| self.incr_rock_position(dir, r))
                            .or(Some(*rock_init))
                        {
                            let (r_x, r_y) = self.get_rock_position(dir, x, y, r);
                            self.spaces[y][x] = Space::Empty;
                            self.spaces[r_y][r_x] = Space::Round;
                            rock = Some(r);
                        };
                    }
                    _ => (),
                }
            }
        }
    }

    fn load(&self) -> u64 {
        let height = self.height();
        self.spaces.iter().enumerate().fold(0, |load, (i, row)| {
            load + ((height - i) * row.iter().filter(|&&space| space == Space::Round).count())
                as u64
        })
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
            spaces: s
                .lines()
                .map(|l| l.chars().map(Space::from).collect())
                .collect(),
        })
    }
}

pub fn part_01(input: &str) -> u64 {
    let mut grid: Grid = input.parse().unwrap();
    grid.tilt(Direction::North);
    grid.load()
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
    let its = (1000000000 - cycle_start) % (grid_set.len() - cycle_start) + cycle_start;

    grid_set
        .iter()
        .find(|&(_, &i)| i == its)
        .map(|(g, _)| g)
        .unwrap()
        .load()
}

#[cfg(test)]
mod tests {
    use crate::utils::{read_resource, relative_input_path};

    const INPUT_PATH: &str = module_path!();

    #[test]
    fn part_01() {
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            136
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            110677
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            64
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            90551
        );
    }
}
