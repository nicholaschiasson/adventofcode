use std::collections::{HashMap, VecDeque};

pub fn part_01(input: &str) -> u64 {
    let word = b"XMAS";
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let mut queue = VecDeque::new();
    let mut occurrences = 0;

    for (irow, row) in grid.iter().enumerate() {
        let irow = irow as isize;
        for (icol, &col) in row.iter().enumerate() {
            let icol = icol as isize;
            if col == word[0] {
                const ROWS: isize = 3;
                const COLUMNS: isize = 3;
                const NEIGHBOURS: isize = ROWS * COLUMNS - 1;
                for i in 0..(NEIGHBOURS / 2) {
                    let x = (i % COLUMNS) - 1;
                    let y = ((i - x) / ROWS) - 1;
                    queue.push_back((1, (irow, icol), (irow + y, icol + x)));
                    queue.push_back((1, (irow, icol), (irow - y, icol - x)));
                }
            }
        }
    }

    while let Some((i, (y1, x1), (y2, x2))) = queue.pop_front() {
        if i >= word.len() {
            occurrences += 1;
            continue;
        }

        if y2 < 0 || x2 < 0 {
            continue;
        }

        if let Some(Some(&c)) = grid.get(y2 as usize).map(|r| r.get(x2 as usize)) {
            if c == word[i] {
                queue.push_back((i + 1, (y2, x2), (y2 + (y2 - y1), x2 + (x2 - x1))));
            }
        }
    }

    occurrences
}

pub fn part_02(input: &str) -> u64 {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let mut queue = VecDeque::new();
    let mut occurrences = HashMap::new();

    for (irow, row) in grid.iter().enumerate() {
        let irow = irow as isize;
        for (icol, &col) in row.iter().enumerate() {
            let icol = icol as isize;
            if col == b'A' {
                const ROWS: isize = 3;
                const COLUMNS: isize = 3;
                const NEIGHBOURS: isize = ROWS * COLUMNS - 1;
                // Step by 2 to skip vertical and horizontal
                for i in (0..(NEIGHBOURS / 2)).step_by(2) {
                    let x = (i % COLUMNS) - 1;
                    let y = ((i - x) / ROWS) - 1;
                    queue.push_back(((irow, icol), (irow + y, icol + x), (irow - y, icol - x)));
                }
            }
        }
    }

    while let Some(((y0, x0), (y1, x1), (y2, x2))) = queue.pop_front() {
        if y1 < 0 || x1 < 0 || y2 < 0 || x2 < 0 {
            continue;
        }

        match (
            grid.get(y1 as usize).map(|r| r.get(x1 as usize)),
            grid.get(y2 as usize).map(|r| r.get(x2 as usize)),
        ) {
            (Some(Some(b'M')), Some(Some(b'S'))) | (Some(Some(b'S')), Some(Some(b'M'))) => {
                *occurrences.entry((y0, x0)).or_insert(0) += 1;
            }
            _ => (),
        }
    }

    occurrences.into_values().map(|o| o / 2).sum()
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
            4
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_02"
            )))),
            18
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            2646
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_02"
            )))),
            9
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            2000
        );
    }
}
