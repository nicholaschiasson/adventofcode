use std::fmt::Display;

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

fn cosmic_expansion(input: &str, expansion_factor: u64) -> u64 {
    let mut row_positions = Vec::new();
    let mut column_positions = Vec::new();
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            row_positions.push(expansion_factor);
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if x >= column_positions.len() {
                        column_positions.push(expansion_factor);
                    }
                    (c == '#').then(|| {
                        row_positions[y] = 1;
                        column_positions[x] = 1;
                        Position::from((x, y))
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for i in 1..row_positions.len() {
        row_positions[i] += row_positions[i - 1];
    }
    for i in 1..column_positions.len() {
        column_positions[i] += column_positions[i - 1];
    }
    let galaxies = galaxies
        .iter()
        .map(|g| Position {
            x: column_positions[g.x as usize],
            y: row_positions[g.y as usize],
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            sum += galaxies[j].distance(&galaxies[i]);
        }
    }
    sum
}

pub fn part_01(input: &str) -> u64 {
    cosmic_expansion(input, 2)
}

pub fn part_02(input: &str) -> u64 {
    cosmic_expansion(input, 1000000)
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
            374
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            9599070
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            82000210
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            842645913794
        );
    }
}
