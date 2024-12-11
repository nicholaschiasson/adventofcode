use std::collections::HashSet;

type Grid = Vec<Vec<i8>>;
type Point = (usize, usize);

fn find_trails(grid: &Grid, (x, y): Point) -> (HashSet<Point>, u64) {
    let mut trails = 0;
    let mut ends = HashSet::new();
    if let Some(&n) = grid.get(y).and_then(|row| row.get(x)) {
        if n == 9 {
            trails = 1;
            ends.insert((x, y));
        } else {
            for i in 0..4 {
                let i = (i * 2) + 1;
                let dx = i % 3;
                let dy = i / 3;
                if x + dx > 0 && y + dy > 0 {
                    let dx = x + dx - 1;
                    let dy = y + dy - 1;
                    if let Some(&m) = grid.get(dy).and_then(|row| row.get(dx)) {
                        if m - n == 1 {
                            let (e, t) = find_trails(grid, (dx, dy));
                            trails += t;
                            ends = ends.union(&e).copied().collect::<HashSet<_>>();
                        }
                    }
                }
            }
        };
    }
    (ends, trails)
}

pub fn part_01(input: &str) -> u64 {
    let mut trailheads = Vec::new();
    let grid: Vec<Vec<i8>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .inspect(|&(x, c)| {
                    if c == '0' {
                        trailheads.push((x, y));
                    }
                })
                .map(|(_, c)| c.to_digit(10).unwrap_or(u8::MAX as u32) as i8)
                .collect()
        })
        .collect();

    trailheads
        .into_iter()
        .fold(0, |a, h| a + find_trails(&grid, h).0.len()) as u64
}

pub fn part_02(input: &str) -> u64 {
    let mut trailheads = Vec::new();
    let grid: Vec<Vec<i8>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .inspect(|&(x, c)| {
                    if c == '0' {
                        trailheads.push((x, y));
                    }
                })
                .map(|(_, c)| c.to_digit(10).unwrap_or(u8::MAX as u32) as i8)
                .collect()
        })
        .collect();

    trailheads
        .into_iter()
        .fold(0, |a, h| a + find_trails(&grid, h).1)
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
            1
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_02"
            )))),
            2
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_03"
            )))),
            4
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_04"
            )))),
            3
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_05"
            )))),
            36
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            816
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_06"
            )))),
            3
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_07"
            )))),
            13
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_08"
            )))),
            227
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_09"
            )))),
            81
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            1960
        );
    }
}
