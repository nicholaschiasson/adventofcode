pub fn part_01(input: &str) -> u64 {
    input
        .lines()
        .scan(None, |s, x| {
            let next = x.parse::<i32>().expect("failed to parse integer");
            let res = if let Some(prev) = *s {
                next > prev
            } else {
                false
            };
            *s = Some(next);
            Some(res)
        })
        .filter(|x| *x)
        .count() as u64
}

pub fn part_02(input: &str) -> u64 {
    let parsed = input
        .lines()
        .map(|x| x.parse::<i32>().expect("failed to parse integer"));
    parsed
        .clone()
        .zip(parsed.clone().skip(1))
        .zip(parsed.clone().skip(2))
        .map(|((a, b), c)| a + b + c)
        .scan(None, |s, next| {
            let res = if let Some(prev) = *s {
                next > prev
            } else {
                false
            };
            *s = Some(next);
            Some(res)
        })
        .filter(|x| *x)
        .count() as u64
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
            1393
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            5
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            1359
        );
    }
}
