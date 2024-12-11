use std::collections::HashMap;

pub fn part_01(input: &str) -> u64 {
    let bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    input
        .lines()
        .enumerate()
        .filter_map(|(i, l)| l.split_once(": ").map(|(_, g)| (i as u64, g)))
        .filter_map(|(i, game)| {
            game.split("; ")
                .all(|set| {
                    set.split(", ").all(|cubes| {
                        cubes
                            .split_once(' ')
                            .map(
                                |(count, color)| match (count.parse::<i32>(), bag.get(color)) {
                                    (Ok(in_set), Some(&in_bag)) => in_set <= in_bag,
                                    _ => false,
                                },
                            )
                            .unwrap_or(false)
                    })
                })
                .then_some(i + 1)
        })
        .sum()
}

pub fn part_02(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|l| l.split_once(": ").map(|(_, g)| (g)))
        .map(|game| {
            game.split("; ")
                .fold(HashMap::new(), |mut fewest, set| {
                    for cubes in set.split(", ") {
                        if let Some((Some(count), color)) = cubes
                            .split_once(' ')
                            .map(|(count, color)| (count.parse::<u64>().ok(), color))
                        {
                            fewest
                                .entry(color)
                                .and_modify(|e| {
                                    if *e < count {
                                        *e = count;
                                    }
                                })
                                .or_insert(count);
                        }
                    }
                    fewest
                })
                .values()
                .product::<u64>()
        })
        .sum()
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
            8
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            2617
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            2286
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            59795
        );
    }
}
