use std::{
    cmp::Ordering,
    ops::{Add, Mul},
    str::FromStr,
};

#[derive(Clone, Copy, Debug)]
struct Point(isize, isize);

impl Point {
    fn to_space(self, Self(sx, sy): Self) -> Self {
        Point((self.0 % sx + sx) % sx, (self.1 % sy + sy) % sy)
    }

    fn get_quadrant(&self, Self(sx, sy): Self) -> Option<usize> {
        let Point(x, y) = self.to_space(Point(sx, sy));
        match (x.cmp(&(sx / 2)), y.cmp(&(sy / 2))) {
            (Ordering::Equal, _) | (_, Ordering::Equal) => None,
            (Ordering::Less, Ordering::Less) => Some(0),
            (Ordering::Greater, Ordering::Less) => Some(1),
            (Ordering::Less, Ordering::Greater) => Some(2),
            (Ordering::Greater, Ordering::Greater) => Some(3),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, Self(rhx, rhy): Self) -> Self::Output {
        Self(self.0 + rhx, self.1 + rhy)
    }
}

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((Ok(x), Ok(y))) = s
            .split_once(',')
            .map(|(x, y)| (x.parse::<isize>(), y.parse::<isize>()))
        {
            Ok(Self(x, y))
        } else {
            Err(format!("Cannot parse string from '{s}'"))
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn predict_position(&self, time: isize) -> Point {
        self.position + (self.velocity * time)
    }
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((Some(Ok(position)), Some(Ok(velocity)))) = s.split_once(' ').map(|(p, v)| {
            (
                p.split_once('=').map(|(_, v)| v.parse::<Point>()),
                v.split_once('=').map(|(_, v)| v.parse::<Point>()),
            )
        }) {
            Ok(Self { position, velocity })
        } else {
            Err(format!("Cannot parse robot from '{s}'"))
        }
    }
}

pub fn part_01(input: &str) -> u64 {
    let robots: Vec<Robot> = input.lines().flat_map(|l| l.parse()).collect();

    let mut space = Point(0, 0);
    for robot in &robots {
        if robot.position.0 >= space.0 {
            space.0 = robot.position.0 + 1;
        }
        if robot.position.1 >= space.1 {
            space.1 = robot.position.1 + 1;
        }
    }

    let mut quadrants = [0; 4];
    for robot in &robots {
        let p = robot.predict_position(100);
        let q = p.get_quadrant(space);
        if let Some(q) = q {
            quadrants[q] += 1;
        }
    }

    quadrants.iter().product()
}

pub fn part_02(input: &str) -> u64 {
    let robots: Vec<Robot> = input.lines().flat_map(|l| l.parse()).collect();

    let mut space = Point(0, 0);
    for robot in &robots {
        if robot.position.0 >= space.0 {
            space.0 = robot.position.0 + 1;
        }
        if robot.position.1 >= space.1 {
            space.1 = robot.position.1 + 1;
        }
    }

    let mut seconds = 0;
    'outer: loop {
        let mut grid = vec![vec![0; space.0 as usize]; space.1 as usize];
        for robot in &robots {
            let Point(x, y) = robot.predict_position(seconds).to_space(space);
            let (x, y) = (x as usize, y as usize);
            grid[y][x] += 1;
            if grid[y][x] > 1 {
                seconds += 1;
                continue 'outer;
            }
        }
        break;
    }
    seconds as u64
}

#[cfg(test)]
mod tests {
    use crate::utils::{read_resource, relative_input_path};

    const INPUT_PATH: &str = module_path!();

    #[test]
    fn part_01() {
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_1"
            )))),
            12
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            233709840
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            6620
        );
    }
}
