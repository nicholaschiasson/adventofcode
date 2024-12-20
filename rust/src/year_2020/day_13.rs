use lexical::parse_partial;

use crate::utils::modulo;

pub fn part_01(input: &str) -> u64 {
    let (earliest_departure, i) = parse_partial::<u64, _>(input.as_bytes()).unwrap();
    let (time, id) = input[i + 1..]
        .split(',')
        .map(|s| s.trim())
        .filter(|s| s != &"x")
        .map(|s| s.parse::<u64>().unwrap())
        .fold((2 * earliest_departure, 0), |(t, i), id| {
            let time = earliest_departure + (id - (earliest_departure % id));
            if time < t {
                (time, id)
            } else {
                (t, i)
            }
        });
    (time - earliest_departure) * id
}

pub fn part_02(input: &str) -> u64 {
    let (_, i) = parse_partial::<u64, _>(input.as_bytes()).unwrap();
    let ids = input[i + 1..]
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    let mut congruences = ids
        .iter()
        .enumerate()
        .filter(|(_, id)| *id != &"x")
        .map(|(i, id)| (i as u64, id.parse::<u64>().unwrap()))
        .map(|(i, id)| (modulo(-(i as i64), id as i64) as u64, id))
        .collect::<Vec<_>>();

    congruences.sort_by(|a, b| b.1.cmp(&a.1));

    congruences
        .iter()
        .skip(1)
        .fold(congruences[0], |(a1, n), (a2, x)| {
            (
                a1 + (0..).find(|i| (a1 + i * n) % x == *a2).unwrap() * n,
                n * x,
            )
        })
        .0
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
            295
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            4808
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            1068781
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            741745043105674
        );
    }
}
