use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

impl Direction {
    const fn len() -> usize {
        Self::Left as usize + 1
    }
}

impl Add<i32> for Direction {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        match (self as i32 + rhs).rem_euclid(Self::len() as i32) {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            3 => Self::Left,
            _ => panic!("oh god how did we get here"),
        }
    }
}

impl AddAssign<i32> for Direction {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl Sub<i32> for Direction {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        self + (-rhs)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Obstruction,
    Guard(Direction),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => '.',
                Self::Obstruction => '#',
                Self::Guard(Direction::Up) => '^',
                Self::Guard(Direction::Right) => '>',
                Self::Guard(Direction::Down) => 'v',
                Self::Guard(Direction::Left) => '<',
            },
        )
    }
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Obstruction),
            '^' => Ok(Self::Guard(Direction::Up)),
            '>' => Ok(Self::Guard(Direction::Right)),
            'v' => Ok(Self::Guard(Direction::Down)),
            '<' => Ok(Self::Guard(Direction::Left)),
            _ => Err(format!("Invalid tile '{value}'")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position(isize, isize);

impl Position {
    fn step(self, direction: Direction) -> Self {
        use Direction::*;
        self + match direction {
            Up => Self(-1, 0),
            Right => Self(0, 1),
            Down => Self(1, 0),
            Left => Self(0, -1),
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, Self(y, x): Self) -> Self::Output {
        Self(self.0 + y, self.1 + x)
    }
}

#[derive(Clone, Debug)]
struct Grid(Vec<Vec<Tile>>, Position, Direction);

impl Grid {
    fn get(&self, Position(y, x): Position) -> Option<Tile> {
        if y < 0 || x < 0 {
            return None;
        }
        if let Some(row) = self.0.get(y as usize) {
            return row.get(x as usize).copied();
        }
        None
    }

    fn set(&mut self, Position(y, x): Position, tile: Tile) {
        if y < 0 || x < 0 {
            return;
        }
        if let Some(Some(t)) = self
            .0
            .get_mut(y as usize)
            .map(|row| row.get_mut(x as usize))
        {
            *t = tile
        }
    }

    const fn start(&self) -> (Position, Direction) {
        (self.1, self.2)
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut position = None;
        let mut direction = None;
        let grid = s
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.chars()
                    .enumerate()
                    .flat_map(|(col, c)| match c.try_into() {
                        Ok(Tile::Guard(dir)) => {
                            position = Some(Position(row as isize, col as isize));
                            direction = Some(dir);
                            Ok(Tile::Empty)
                        }
                        t => t,
                    })
                    .collect()
            })
            .collect();
        match (position, direction) {
            (Some(p), Some(d)) => Ok(Self(grid, p, d)),
            (None, _) => Err("Guard not found".to_string()),
            (_, None) => Err("Invalid guard starting direction".to_string()),
        }
    }
}

pub fn part_01(input: &str) -> u64 {
    let grid = input.parse::<Grid>();

    if let Ok(grid) = grid {
        let (mut position, mut direction) = grid.start();
        let mut visited = vec![vec![false; grid.width()]; grid.height()];

        loop {
            visited[position.0 as usize][position.1 as usize] = true;
            let next = position.step(direction);
            match grid.get(next) {
                Some(Tile::Obstruction) => direction += 1,
                Some(Tile::Empty) => position = next,
                _ => break,
            }
        }

        return visited.into_iter().fold(0, |sum, row| {
            sum + row.into_iter().filter(|&t| t).count() as u64
        });
    }

    0
}

pub fn part_02(input: &str) -> u64 {
    let grid = input.parse::<Grid>();

    if let Ok(mut grid) = grid {
        let (mut position, mut direction) = grid.start();
        let init = position;
        let width = grid.width();
        let height = grid.height();
        let mut visited = vec![vec![false; width]; height];
        let mut obstacles = 0;
        let mut simulated = vec![vec![vec![false; Direction::len()]; width]; height];

        loop {
            visited[position.0 as usize][position.1 as usize] = true;
            let next = position.step(direction);
            match grid.get(next) {
                Some(Tile::Obstruction) => direction += 1,
                Some(Tile::Empty) => {
                    if next != init && !visited[next.0 as usize][next.1 as usize] {
                        grid.set(next, Tile::Obstruction);
                        for row in &mut simulated {
                            for col in row {
                                for dir in col {
                                    *dir = false;
                                }
                            }
                        }
                        let mut pos = position;
                        let mut dir = direction;
                        loop {
                            let (y, x, d) = (pos.0 as usize, pos.1 as usize, dir as usize);
                            if simulated[y][x][d] {
                                obstacles += 1;
                                break;
                            }
                            simulated[y][x][d] = true;
                            let n = pos.step(dir);
                            match grid.get(n) {
                                Some(Tile::Obstruction) => dir += 1,
                                Some(Tile::Empty) => pos = n,
                                _ => break,
                            }
                        }
                        grid.set(next, Tile::Empty);
                    }
                    position = next;
                }
                _ => break,
            }
        }

        return obstacles;
    }

    0
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
            41
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            4964
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            6
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            1740
        );
    }
}
