use std::{cmp::Ordering, fmt::Display};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Space {
    EmptySeat,
    Floor,
    OccupiedSeat,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Space::EmptySeat => "L",
                Space::Floor => ".",
                Space::OccupiedSeat => "#",
            }
        )
    }
}

struct Position(isize, isize);

fn parse_grid(grid: &String) -> Vec<Vec<Space>> {
    grid.lines()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|c| match c {
                    b'#' => Space::OccupiedSeat,
                    b'L' => Space::EmptySeat,
                    b'.' | _ => Space::Floor,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn count_adjacent_occupancies(grid: &Vec<Vec<Space>>, position: Position, scan: bool) -> u64 {
    let Position(y, x) = position;
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            for k in 1.. {
                let y_pos = y + i * k;
                let x_pos = x + j * k;
                if (!scan && k > 1)
                    || y_pos < 0
                    || y_pos >= grid.len() as isize
                    || x_pos < 0
                    || x_pos >= grid[y_pos as usize].len() as isize
                    || (y_pos == y && x_pos == x)
                {
                    break;
                }
                match grid[y_pos as usize][x_pos as usize] {
                    Space::EmptySeat => {
                        break;
                    }
                    Space::Floor => {
                        continue;
                    }
                    Space::OccupiedSeat => {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }
    count
}

fn compute_seating(
    grid: &Vec<Vec<Space>>,
    discomfort_threshold: u64,
    scan: bool,
) -> Vec<Vec<Space>> {
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, s)| match s {
                    Space::Floor => Space::Floor,
                    Space::EmptySeat => {
                        if count_adjacent_occupancies(grid, Position(i as isize, j as isize), scan)
                            == 0
                        {
                            Space::OccupiedSeat
                        } else {
                            Space::EmptySeat
                        }
                    }
                    Space::OccupiedSeat => {
                        if count_adjacent_occupancies(grid, Position(i as isize, j as isize), scan)
                            < discomfort_threshold
                        {
                            Space::OccupiedSeat
                        } else {
                            Space::EmptySeat
                        }
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
trait Clamp: PartialOrd {
    fn clamp(self, a: Self, b: Self) -> Self;
}

impl Clamp for isize {
    fn clamp(self, a: Self, b: Self) -> Self {
        if self < a {
            a
        } else if self > b {
            b
        } else {
            self
        }
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<Space>>) -> String {
    grid.iter().fold(String::new(), |mut s, r| {
        s.extend(r.iter().map(|s| s.to_string()));
        s.extend(&['\n']);
        s
    })
}

pub fn part_01(input: &String) -> u64 {
    let mut grid = parse_grid(input);
    loop {
        let previous = grid.to_vec();
        grid = compute_seating(&grid, 4, false);
        if let Ordering::Equal = previous.iter().flatten().cmp(grid.iter().flatten()) {
            break;
        }
    }
    grid.iter().flatten().fold(0, |n, s| {
        if let Space::OccupiedSeat = s {
            n + 1
        } else {
            n
        }
    })
}

pub fn part_02(input: &String) -> u64 {
    let mut grid = parse_grid(input);
    loop {
        let previous = grid.to_vec();
        grid = compute_seating(&grid, 5, true);
        if let Ordering::Equal = previous.iter().flatten().cmp(grid.iter().flatten()) {
            break;
        }
    }
    grid.iter().flatten().fold(0, |n, s| {
        if let Space::OccupiedSeat = s {
            n + 1
        } else {
            n
        }
    })
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
            37
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            2470
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            26
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            2259
        );
    }
}
