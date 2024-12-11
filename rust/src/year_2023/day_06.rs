pub fn part_01(input: &str) -> u64 {
    input
        .split_once('\n')
        .map(|(times, distances)| {
            times
                .split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|t| t.parse::<f64>().expect("parse time"))
                .zip(
                    distances
                        .split_once(':')
                        .unwrap()
                        .1
                        .split_whitespace()
                        .map(|d| d.parse::<f64>().expect("parse distance")),
                )
                .map(|(time, distance)| {
                    // Some disgusting wal-mart brand quadratic formula hack
                    (time
                        - 2.0
                            * (((time
                                - (time.powf(2.0)
                                    - (4.0 * (((distance / 2.0).floor() * 2.0) + 1.0)))
                                    .sqrt())
                                / 2.0)
                                .floor()
                                + 1.0)
                        + 1.0) as u64
                })
                .product()
        })
        .unwrap()
}

pub fn part_02(input: &str) -> u64 {
    input
        .split_once('\n')
        .map(|(time, distance)| {
            (
                time.chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>(),
                distance
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>(),
            )
        })
        .map(|(time, distance)| {
            (
                time.split_once(':').unwrap().1.parse::<f32>().unwrap(),
                distance.split_once(':').unwrap().1.parse::<f32>().unwrap(),
            )
        })
        .map(|(time, distance)| {
            (time
                - 2.0
                    * (((time
                        - (time.powf(2.0) - (4.0 * (((distance / 2.0).floor() * 2.0) + 1.0)))
                            .sqrt())
                        / 2.0)
                        .floor()
                        + 1.0)
                + 1.0) as u64
        })
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
            288
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            1624896
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            71503
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            32583852
        );
    }
}
