use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Chunk {
    Angle,
    Brace,
    Paranthesis,
    Square,
}

impl Chunk {
    fn incorrect_points(&self) -> u64 {
        match self {
            Self::Angle => 25137,
            Self::Brace => 1197,
            Self::Paranthesis => 3,
            Self::Square => 57,
        }
    }

    fn incomplete_points(&self) -> u64 {
        match self {
            Self::Angle => 4,
            Self::Brace => 3,
            Self::Paranthesis => 1,
            Self::Square => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Enclosing {
    Closing(Chunk),
    Opening(Chunk),
}

impl Display for Enclosing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Opening(Chunk::Angle) => '<',
                Self::Opening(Chunk::Brace) => '{',
                Self::Opening(Chunk::Paranthesis) => '(',
                Self::Opening(Chunk::Square) => '[',
                Self::Closing(Chunk::Angle) => '>',
                Self::Closing(Chunk::Brace) => '}',
                Self::Closing(Chunk::Paranthesis) => ')',
                Self::Closing(Chunk::Square) => ']',
            }
        )
    }
}

impl From<char> for Enclosing {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Opening(Chunk::Angle),
            '{' => Self::Opening(Chunk::Brace),
            '(' => Self::Opening(Chunk::Paranthesis),
            '[' => Self::Opening(Chunk::Square),
            '>' => Self::Closing(Chunk::Angle),
            '}' => Self::Closing(Chunk::Brace),
            ')' => Self::Closing(Chunk::Paranthesis),
            ']' => Self::Closing(Chunk::Square),
            _ => panic!("Invalid chunk enclosing character '{}'", c),
        }
    }
}

pub fn part_01(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut s = vec![];
            for c in l.chars() {
                match Enclosing::from(c) {
                    Enclosing::Closing(c) => {
                        if let Some(chunk) = s.pop() {
                            if c != chunk {
                                return c.incorrect_points();
                            }
                        } else {
                            return c.incorrect_points();
                        }
                    }
                    Enclosing::Opening(c) => s.push(c),
                }
            }
            0
        })
        .sum()
}

pub fn part_02(input: &str) -> u64 {
    let mut valid = input
        .lines()
        .filter_map(|l| {
            let mut s = vec![];
            for c in l.chars() {
                match Enclosing::from(c) {
                    Enclosing::Closing(c) => {
                        if let Some(chunk) = s.pop() {
                            if c != chunk {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    }
                    Enclosing::Opening(c) => s.push(c),
                }
            }
            Some(
                s.iter()
                    .rev()
                    .fold(0, |p, &c| p * 5 + c.incomplete_points()),
            )
        })
        .collect::<Vec<_>>();
    valid.sort();
    valid[valid.len() / 2]
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
            26397
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            344193
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            288957
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            3241238967
        );
    }
}
