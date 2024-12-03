use std::collections::HashSet;
use std::collections::VecDeque;

const HIGHEST_POINT: u32 = 9;

fn neighbours((y, x): (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    let min_y = 0.max(y as isize - 1) as usize;
    let min_x = 0.max(x as isize - 1) as usize;
    let max_y = (height - 1).min(y + 1);
    let max_x = (width - 1).min(x + 1);
    let mut neighbours = vec![];
    if min_y != y {
        neighbours.push((min_y, x));
    }
    if max_y != y {
        neighbours.push((max_y, x));
    }
    if min_x != x {
        neighbours.push((y, min_x));
    }
    if max_x != x {
        neighbours.push((y, max_x));
    }
    neighbours
}

pub fn part_01(input: &String) -> u64 {
    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = map.len();
    let width = map[0].len();
    let mut risk = 0;
    for i in 0..height {
        for j in 0..width {
            if neighbours((i, j), height, width)
                .iter()
                .all(|&(y, x)| map[i][j] < map[y][x])
            {
                risk += map[i][j] + 1;
            }
        }
    }
    risk as u64
}

pub fn part_02(input: &String) -> u64 {
    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = map.len();
    let width = map[0].len();
    let mut low_points = vec![];
    for i in 0..height {
        for j in 0..width {
            if neighbours((i, j), height, width)
                .iter()
                .all(|&(y, x)| map[i][j] < map[y][x])
            {
                low_points.push((i, j));
            }
        }
    }
    let mut basins = low_points
        .iter()
        .map(|&point| {
            let mut checked = HashSet::new();
            let mut queue = VecDeque::from([point]);
            let mut size = 0;
            while let Some((y, x)) = queue.pop_front() {
                checked.insert((y, x));
                if map[y][x] < HIGHEST_POINT {
                    queue.extend(
                        neighbours((y, x), height, width)
                            .iter()
                            .filter(|&p| !checked.contains(p) && !queue.contains(p))
                            .collect::<Vec<_>>(),
                    );
                    size += 1;
                }
            }
            size
        })
        .collect::<Vec<_>>();
    basins.sort_by(|a, b| b.cmp(a));
    basins.iter().take(3).product::<i32>() as u64
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
            15
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            537
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            1134
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            1142757
        );
    }
}
