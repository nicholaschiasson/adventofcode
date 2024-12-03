use std::{collections::VecDeque, fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn incr_pos(&self, x: isize, y: isize) -> (isize, isize) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct DumbVec((isize, isize), Direction);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Angle {
    Forward,
    Backward,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Space {
    #[default]
    Empty,
    Mirror(Angle),
    Splitter(Axis),
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Space::Empty,
            '\\' => Space::Mirror(Angle::Backward),
            '/' => Space::Mirror(Angle::Forward),
            '-' => Space::Splitter(Axis::Horizontal),
            '|' => Space::Splitter(Axis::Vertical),
            _ => panic!("get that outta my space '{value}'"),
        }
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Space::Empty => '.',
                Space::Mirror(Angle::Backward) => '\\',
                Space::Mirror(Angle::Forward) => '/',
                Space::Splitter(Axis::Horizontal) => '-',
                Space::Splitter(Axis::Vertical) => '|',
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Tile {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    space: Space,
}

impl Tile {
    fn energize(&mut self, DumbVec((x, y), dir): DumbVec) -> Option<Vec<DumbVec>> {
        match dir {
            Direction::Up => {
                if self.up {
                    return None;
                }
                self.up = true;
            }
            Direction::Down => {
                if self.down {
                    return None;
                }
                self.down = true;
            }
            Direction::Left => {
                if self.left {
                    return None;
                }
                self.left = true;
            }
            Direction::Right => {
                if self.right {
                    return None;
                }
                self.right = true;
            }
        }
        self.up = self.up || dir == Direction::Up;
        self.down = self.down || dir == Direction::Down;
        self.left = self.left || dir == Direction::Left;
        self.right = self.right || dir == Direction::Right;
        match (self.space, dir) {
            (Space::Empty, _)
            | (Space::Splitter(Axis::Horizontal), Direction::Left | Direction::Right)
            | (Space::Splitter(Axis::Vertical), Direction::Up | Direction::Down) => {
                Some(vec![DumbVec(dir.incr_pos(x, y), dir)])
            }
            (Space::Splitter(Axis::Horizontal), Direction::Up | Direction::Down) => Some(vec![
                DumbVec((x - 1, y), Direction::Left),
                DumbVec((x + 1, y), Direction::Right),
            ]),
            (Space::Splitter(Axis::Vertical), Direction::Left | Direction::Right) => Some(vec![
                DumbVec((x, y - 1), Direction::Up),
                DumbVec((x, y + 1), Direction::Down),
            ]),
            (Space::Mirror(Angle::Forward), Direction::Up) => {
                Some(vec![DumbVec((x + 1, y), Direction::Right)])
            }
            (Space::Mirror(Angle::Forward), Direction::Down) => {
                Some(vec![DumbVec((x - 1, y), Direction::Left)])
            }
            (Space::Mirror(Angle::Forward), Direction::Left) => {
                Some(vec![DumbVec((x, y + 1), Direction::Down)])
            }
            (Space::Mirror(Angle::Forward), Direction::Right) => {
                Some(vec![DumbVec((x, y - 1), Direction::Up)])
            }
            (Space::Mirror(Angle::Backward), Direction::Up) => {
                Some(vec![DumbVec((x - 1, y), Direction::Left)])
            }
            (Space::Mirror(Angle::Backward), Direction::Down) => {
                Some(vec![DumbVec((x + 1, y), Direction::Right)])
            }
            (Space::Mirror(Angle::Backward), Direction::Left) => {
                Some(vec![DumbVec((x, y - 1), Direction::Up)])
            }
            (Space::Mirror(Angle::Backward), Direction::Right) => {
                Some(vec![DumbVec((x, y + 1), Direction::Down)])
            }
        }
    }

    fn energy(&self) -> u64 {
        (self.up as u64) + (self.down as u64) + (self.left as u64) + (self.right as u64)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self.space, self.energy()) {
                (Space::Empty, 0) => self.space.to_string(),
                (Space::Empty, e) => e.to_string(),
                (Space::Mirror(_) | Space::Splitter(_), _) => self.space.to_string(),
            }
        )
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Tile {
            space: value.into(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn energize(&mut self, start: DumbVec) -> u64 {
        let mut queue = VecDeque::from([start]);
        while let Some(DumbVec((x, y), dir)) = queue.pop_front() {
            if !(0..self.tiles.len() as isize).contains(&y)
                || !(0..self.tiles[0].len() as isize).contains(&x)
            {
                continue;
            }
            if let Some(charges) = self.tiles[y as usize][x as usize].energize(DumbVec((x, y), dir))
            {
                queue.extend(charges);
            }
        }
        self.tiles
            .iter()
            .map(|row| row.iter().filter(|tile| tile.energy() > 0).count() as u64)
            .sum()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            tiles: s
                .lines()
                .map(|line| line.chars().map(Tile::from).collect())
                .collect(),
        })
    }
}

pub fn part_01(input: &str) -> u64 {
    let mut grid = input.parse::<Grid>().unwrap();
    grid.energize(DumbVec((0, 0), Direction::Right))
}

pub fn part_02(input: &str) -> u64 {
    let grid = input.parse::<Grid>().unwrap();
    let mut max = 0;
    let height = grid.tiles.len() as isize;
    let width = grid.tiles[0].len() as isize;
    for i in 0..height {
        max = max.max(grid.clone().energize(DumbVec((0, i), Direction::Right)));
        max = max.max(
            grid.clone()
                .energize(DumbVec((width - 1, i), Direction::Left)),
        );
    }
    for i in 0..width {
        max = max.max(grid.clone().energize(DumbVec((i, 0), Direction::Down)));
        max = max.max(
            grid.clone()
                .energize(DumbVec((i, height - 1), Direction::Up)),
        );
    }
    max
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
            46
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            8249
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            51
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            8444
        );
    }
}
