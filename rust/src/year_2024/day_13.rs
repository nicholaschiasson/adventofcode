use std::collections::HashSet;

type Point = (i64, i64);

fn _solve_machine((ax, ay): Point, (bx, by): Point, (px, py): Point, offset: i64) -> u64 {
    let (px, py) = (px + offset, py + offset);
    let mut min_tokens = None;
    let mut stack = vec![(0, 0, 0)];
    let mut seen = HashSet::new();
    while let Some((x, y, tokens)) = stack.pop() {
        if seen.contains(&(x, y)) {
            continue;
        }
        seen.insert((x, y));
        if x == px && y == py {
            min_tokens = min_tokens
                .map(|t| if tokens < t { tokens } else { t })
                .or(Some(tokens));
            continue;
        }
        if x < px && y < py {
            stack.push((x + ax, y + ay, tokens + 3));
            stack.push((x + bx, y + by, tokens + 1));
        }
    }
    min_tokens.unwrap_or_default()
}

fn solve_machine((ax, ay): Point, (bx, by): Point, (px, py): Point, offset: i64) -> u64 {
    let (px, py) = (px + offset, py + offset);
    let det = ax * by - ay * bx;
    let a = (px * by - py * bx) / det;
    let b = (ax * py - ay * px) / det;
    if (ax * a + bx * b, ay * a + by * b) == (px, py) {
        (a * 3 + b) as u64
    } else {
        0
    }
}

pub fn part_01(input: &str) -> u64 {
    let mut tokens_required = 0;

    for game in input.split("\n\n") {
        let game = game
            .lines()
            .map(|l| {
                l.split(',')
                    .take(2)
                    .flat_map(|v| v.split_once(['+', '=']))
                    .flat_map(|(_, n)| n.parse::<i64>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if let [a, b, p] = &game[..] {
            if let ([ax, ay], [bx, by], [px, py]) = (&a[..], &b[..], &p[..]) {
                tokens_required += solve_machine((*ax, *ay), (*bx, *by), (*px, *py), 0);
            }
        }
    }

    tokens_required
}

pub fn part_02(input: &str) -> u64 {
    let mut tokens_required = 0;

    for game in input.split("\n\n") {
        let game = game
            .lines()
            .map(|l| {
                l.split(',')
                    .take(2)
                    .flat_map(|v| v.split_once(['+', '=']))
                    .flat_map(|(_, n)| n.parse::<i64>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if let [a, b, p] = &game[..] {
            if let ([ax, ay], [bx, by], [px, py]) = (&a[..], &b[..], &p[..]) {
                tokens_required +=
                    solve_machine((*ax, *ay), (*bx, *by), (*px, *py), 10000000000000);
            }
        }
    }

    tokens_required
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
            480
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            36870
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            78101482023732
        );
    }
}
