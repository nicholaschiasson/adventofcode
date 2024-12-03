struct Slope {
    down: i32,
    right: i32,
}

fn count_collisions(grid: &String, slope: &Slope) -> u64 {
    let mut position = 0;
    grid.lines()
        .enumerate()
        .filter_map(|(i, r)| {
            if i as i32 % slope.down == 0 {
                Some(r)
            } else {
                None
            }
        })
        .fold(0, |trees, row| {
            let previous_position = position;
            position += slope.right;
            trees
                + match row.chars().nth(previous_position as usize % row.len()) {
                    Some('#') => 1,
                    _ => 0,
                }
        })
}

pub fn part_01(input: &String) -> u64 {
    count_collisions(input, &Slope { down: 1, right: 3 })
}

pub fn part_02(input: &String) -> u64 {
    let slopes = [
        Slope { down: 1, right: 1 },
        Slope { down: 1, right: 3 },
        Slope { down: 1, right: 5 },
        Slope { down: 1, right: 7 },
        Slope { down: 2, right: 1 },
    ];
    slopes
        .iter()
        .fold(1, |product, slope| product * count_collisions(input, slope))
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
            7
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            151
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            336
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            7540141059
        );
    }
}
