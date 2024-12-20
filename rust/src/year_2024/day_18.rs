use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

type Point = (usize, usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Space {
    Safe,
    Corrupted,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Safe => '.',
                Self::Corrupted => '#',
            }
        )
    }
}

fn bfs(grid: &[Vec<Space>], (sx, sy): Point, (ex, ey): Point) -> Option<HashSet<Point>> {
    if !matches!(
        (
            grid.get(sy).and_then(|row| row.get(sx)),
            grid.get(ey).and_then(|row| row.get(ex)),
        ),
        (Some(&Space::Safe), Some(&Space::Safe))
    ) {
        return None;
    }

    let mut visited = HashSet::from([(sx, sy)]);
    let mut queue = VecDeque::from([(HashSet::new(), (sx, sy))]);
    while let Some((mut path, (x, y))) = queue.pop_front() {
        if let Some(Space::Safe) = grid.get(y).and_then(|row| row.get(x)) {
            path.insert((x, y));
            if x == ex && y == ey {
                return Some(path);
            }
            let mut points = vec![(x + 1, y), (x, y + 1)];
            if x > 0 {
                points.push((x - 1, y));
            }
            if y > 0 {
                points.push((x, y - 1));
            }
            for next in points {
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back((path.clone(), next));
                }
            }
        }
    }

    None
}

pub fn part_01(input: &str, (dx, dy): Point, bytes: usize) -> u64 {
    let mut grid = vec![vec![Space::Safe; dx]; dy];
    let corrupt_bytes = input
        .lines()
        .take(bytes)
        .flat_map(|byte| byte.split_once(','))
        .flat_map(|(x, y)| {
            x.parse::<usize>()
                .and_then(|x| y.parse::<usize>().map(|y| (x, y)))
        })
        .collect::<Vec<_>>();
    for (x, y) in corrupt_bytes {
        grid[x][y] = Space::Corrupted;
    }
    bfs(&grid, (0, 0), (dx - 1, dy - 1))
        .map(|path| path.len() as u64 - 1)
        .unwrap_or_default()
}

pub fn part_02(input: &str, (dx, dy): Point) -> Point {
    let mut grid = vec![vec![Space::Safe; dx]; dy];
    let corrupt_bytes = input
        .lines()
        .flat_map(|byte| byte.split_once(','))
        .flat_map(|(x, y)| {
            x.parse::<usize>()
                .and_then(|x| y.parse::<usize>().map(|y| (x, y)))
        })
        .collect::<Vec<_>>();

    let mut len = corrupt_bytes.len();
    let mut i = 0;
    let mut byte = (dx, dy);
    while len > 0 {
        len /= 2;
        let (j, range) = if bfs(&grid, (0, 0), (dx - 1, dy - 1)).is_none() {
            byte = corrupt_bytes[i - 1];
            (i - len, i - len..=i)
        } else {
            (i + len, i..=i + len)
        };
        for (j, &(x, y)) in corrupt_bytes.iter().enumerate() {
            if j > i {
                grid[y][x] = Space::Safe;
            } else {
                grid[y][x] = Space::Corrupted;
            }
        }
        for k in range {
            let (x, y) = corrupt_bytes[k];
            grid[y][x] = if i < j { Space::Corrupted } else { Space::Safe };
        }
        i = j;
    }

    byte
}

#[cfg(test)]
mod tests {
    use crate::utils::{read_resource, relative_input_path};

    const INPUT_PATH: &str = module_path!();

    #[test]
    fn part_01() {
        assert_eq!(
            super::part_01(
                &read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_1"))),
                (7, 7),
                12
            ),
            22
        );
        assert_eq!(
            super::part_01(
                &read_resource(relative_input_path(&format!("{INPUT_PATH}::final"))),
                (71, 71),
                1024
            ),
            344
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(
                &read_resource(relative_input_path(&format!("{INPUT_PATH}::practice_1"))),
                (7, 7)
            ),
            (6, 1)
        );
        assert_eq!(
            super::part_02(
                &read_resource(relative_input_path(&format!("{INPUT_PATH}::final"))),
                (71, 71)
            ),
            (46, 18)
        );
    }
}
