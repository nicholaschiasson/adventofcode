use std::{ops::Range, str::FromStr};

struct SectionAssignmentPair(Range<u8>, Range<u8>);

impl FromStr for SectionAssignmentPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s.split_once(',').expect("section assignments pair");
        let ((a1, a2), (b1, b2)) = (
            s1.split_once('-').expect("section assignment"),
            s2.split_once('-').expect("section assignment"),
        );
        Ok(SectionAssignmentPair(
            a1.parse::<u8>().expect("section bound a1 as number")
                ..a2.parse::<u8>().expect("section bound a2 as number"),
            b1.parse::<u8>().expect("section bound b1 as number")
                ..b2.parse::<u8>().expect("section bound b2 as number"),
        ))
    }
}

pub fn part_01(input: &String) -> u64 {
    input
        .lines()
        .map(|l| {
            l.parse::<SectionAssignmentPair>()
                .expect("parse section assignment pair")
        })
        .filter(|SectionAssignmentPair(s1, s2)| {
            (s1.start >= s2.start && s1.end <= s2.end) || (s1.start <= s2.start && s1.end >= s2.end)
        })
        .count() as u64
}

pub fn part_02(input: &String) -> u64 {
    input
        .lines()
        .map(|l| {
            l.parse::<SectionAssignmentPair>()
                .expect("parse section assignment pair")
        })
        .filter(|SectionAssignmentPair(s1, s2)| s1.start <= s2.end && s1.end >= s2.start)
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
            2
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            644
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            4
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            926
        );
    }
}
