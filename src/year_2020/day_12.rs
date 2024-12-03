use std::{convert::TryFrom, str::FromStr};

use crate::utils::modulo;

#[derive(Clone, Copy, Debug)]
struct Vector(Direction, Point);

impl Vector {
    fn move_forward(self, distance: i64) -> Self {
        let Self(d, _) = self;
        self.move_direction(d, distance)
    }
    fn move_direction(self, direction: Direction, distance: i64) -> Self {
        let Self(d, Point(x, y)) = self;
        Self(
            d,
            match direction {
                Direction::North => Point(x, y - distance),
                Direction::East => Point(x + distance, y),
                Direction::South => Point(x, y + distance),
                Direction::West => Point(x - distance, y),
            },
        )
    }
    fn perform_action(self, action: Action) -> Self {
        match action {
            Action::Forward(d) => self.move_forward(d),
            Action::Move(dir, dis) => self.move_direction(dir, dis),
            Action::Rotate(r) => self.rotate(r),
        }
    }
    fn rotate(self, rotation: Rotation) -> Self {
        let Self(d, p) = self;
        Self(
            Direction::try_from(modulo(
                d as i64
                    + match rotation {
                        Rotation::Left(degrees) => -degrees,
                        Rotation::Right(degrees) => degrees,
                    },
                4,
            ))
            .unwrap_or_else(|_| panic!("Failed to rotate {:?}", rotation)),
            p,
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct ShipState(Point, Point);

impl ShipState {
    fn move_forward(self, distance: i64) -> Self {
        let Self(Point(x, y), Point(wx, wy)) = self;
        Self(
            Point(x + (wx * distance), y + (wy * distance)),
            Point(wx, wy),
        )
    }
    fn move_waypoint(self, direction: Direction, distance: i64) -> Self {
        let Self(p, Point(x, y)) = self;
        Self(
            p,
            match direction {
                Direction::North => Point(x, y + distance),
                Direction::East => Point(x + distance, y),
                Direction::South => Point(x, y - distance),
                Direction::West => Point(x - distance, y),
            },
        )
    }
    fn perform_action(self, action: Action) -> Self {
        match action {
            Action::Forward(d) => self.move_forward(d),
            Action::Move(dir, dis) => self.move_waypoint(dir, dis),
            Action::Rotate(r) => self.orbit_waypoint(r),
        }
    }
    fn orbit_waypoint(self, rotation: Rotation) -> Self {
        let Self(p, Point(x, y)) = self;
        let hypotenuse = (((x * x) + (y * y)) as f64).sqrt();
        let (fx, fy) = (x as f64, y as f64);
        let angle = fy.atan2(fx)
            + (match rotation {
                Rotation::Left(degrees) => degrees * 90,
                Rotation::Right(degrees) => -degrees * 90,
            } as f64)
                .to_radians();
        Self(
            p,
            Point(
                (hypotenuse * angle.cos()).round() as i64,
                (hypotenuse * angle.sin()).round() as i64,
            ),
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct Point(i64, i64);

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl TryFrom<i64> for Direction {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::North),
            1 => Ok(Direction::East),
            2 => Ok(Direction::South),
            3 => Ok(Direction::West),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Left(i64),
    Right(i64),
}

#[derive(Clone, Copy, Debug)]
enum Action {
    Forward(i64),
    Move(Direction, i64),
    Rotate(Rotation),
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..1] {
            "F" => Ok(Action::Forward(
                s[1..].parse().expect("Failed to parse forward action"),
            )),
            "N" => Ok(Action::Move(
                Direction::North,
                s[1..].parse().expect("Failed to parse forward action"),
            )),
            "E" => Ok(Action::Move(
                Direction::East,
                s[1..].parse().expect("Failed to parse forward action"),
            )),
            "S" => Ok(Action::Move(
                Direction::South,
                s[1..].parse().expect("Failed to parse forward action"),
            )),
            "W" => Ok(Action::Move(
                Direction::West,
                s[1..].parse().expect("Failed to parse forward action"),
            )),
            "L" => Ok(Action::Rotate(Rotation::Left(
                s[1..]
                    .parse::<i64>()
                    .expect("Failed to parse forward action")
                    / 90,
            ))),
            "R" => Ok(Action::Rotate(Rotation::Right(
                s[1..]
                    .parse::<i64>()
                    .expect("Failed to parse forward action")
                    / 90,
            ))),
            _ => Err(()),
        }
    }
}

pub fn part_01(input: &String) -> u64 {
    let Vector(_, Point(x, y)) = input
        .lines()
        .map(|l| l.parse::<Action>().expect("Failed to parse action"))
        .fold(Vector(Direction::East, Point(0, 0)), |v, a| {
            v.perform_action(a)
        });
    (x.abs() + y.abs()) as u64
}

pub fn part_02(input: &String) -> u64 {
    let ShipState(Point(x, y), _) = input
        .lines()
        .map(|l| l.parse::<Action>().expect("Failed to parse action"))
        .fold(ShipState(Point(0, 0), Point(10, 1)), |v, a| {
            v.perform_action(a)
        });
    (x.abs() + y.abs()) as u64
}

#[cfg(test)]
mod tests {
    use crate::utils::{read_resource, relative_input_path};

    const INPUT_PATH: &str = module_path!();

    #[test]
    fn part_01() {
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            25
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            796
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            286
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            39446
        );
    }
}
