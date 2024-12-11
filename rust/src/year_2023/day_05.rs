pub fn part_01(input: &str) -> u64 {
    let seeds = input
        .lines()
        .next()
        .and_then(|l| {
            l.split_once(' ').map(|(_, seeds)| {
                seeds
                    .split_whitespace()
                    .map(|seed| seed.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
        })
        .unwrap();

    input
        .split("\n\n")
        .skip(1)
        .fold(seeds, |v, m| {
            v.iter()
                .map(|&n| {
                    m.lines()
                        .skip(1)
                        .map(|l| {
                            l.split_whitespace()
                                .map(|n| n.parse::<u64>().unwrap())
                                .collect::<Vec<_>>()
                        })
                        .find(|m| (m[1]..m[1] + m[2]).contains(&n))
                        .map(|m| m[0] + n - m[1])
                        .unwrap_or(n)
                })
                .collect::<Vec<_>>()
        })
        .iter()
        .min()
        .copied()
        .unwrap()
}

pub fn part_02(input: &str) -> u64 {
    let seeds = input
        .lines()
        .next()
        .and_then(|l| {
            l.split_once(' ').map(|(_, seeds)| {
                seeds
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .chunks(2)
                    .map(|seed_range| {
                        (
                            seed_range[0].parse::<u64>().unwrap(),
                            seed_range[1].parse::<u64>().unwrap(),
                        )
                    })
                    .map(|(start, length)| (start, start + length))
                    .collect::<Vec<_>>()
            })
        })
        .unwrap();

    input
        .split("\n\n")
        .skip(1)
        .fold(seeds, |v, m| {
            v.iter()
                .flat_map(|&(start, end)| {
                    m.lines()
                        .skip(1)
                        .map(|l| {
                            l.split_whitespace()
                                .map(|n| n.parse::<u64>().unwrap())
                                .collect::<Vec<_>>()
                        })
                        .find_map(|m| match (m[1]..m[1] + m[2], m[0]..m[0] + m[2], m[2]) {
                            (src, dst, _) if src.contains(&start) && src.contains(&end) => Some(
                                vec![(dst.start + start - src.start, dst.start + end - src.start)],
                            ),
                            (src, dst, len) if src.contains(&start) => Some(vec![
                                (dst.start + start - src.start, dst.start + len),
                                (src.end, end),
                            ]),
                            (src, _, _) if src.contains(&end) => {
                                Some(vec![(start, src.start), (m[0], m[0] + end - m[1])])
                            }
                            (src, dst, _)
                                if (start..end).contains(&src.start)
                                    && (start..end).contains(&src.end) =>
                            {
                                Some(vec![
                                    (start, src.start),
                                    (dst.start, dst.end),
                                    (src.end, end),
                                ])
                            }
                            _ => None,
                        })
                        .unwrap_or(vec![(start, end)])
                })
                .collect::<Vec<_>>()
        })
        .iter()
        .map(|&(start, _)| start)
        .min()
        .unwrap()
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
            35
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            331445006
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            46
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            6472060
        );
    }
}
