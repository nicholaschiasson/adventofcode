use std::collections::HashMap;

use regex::Regex;

fn apply_bitmask(num: u64, bitmask: &[u8]) -> u64 {
    bitmask.iter().enumerate().fold(num, |x, (i, b)| match b {
        b'0' => clear_bit(x, i as u64),
        b'1' => set_bit(x, i as u64),
        _ => x,
    })
}

fn apply_floating_bitmask(num: u64, bitmask: &[u8]) -> Vec<u64> {
    bitmask
        .iter()
        .enumerate()
        .fold(vec![num], |values, (i, b)| match b {
            b'0' => values,
            b'1' => values
                .iter()
                .map(|v| set_bit(*v, i as u64))
                .collect::<Vec<_>>(),
            _ => {
                let mut r = values
                    .iter()
                    .map(|v| clear_bit(*v, i as u64))
                    .collect::<Vec<_>>();
                r.extend(values.iter().map(|v| set_bit(*v, i as u64)));
                r
            }
        })
}

fn clear_bit(num: u64, bit: u64) -> u64 {
    num & !(1 << bit)
}

fn set_bit(num: u64, bit: u64) -> u64 {
    num | (1 << bit)
}

pub fn part_01(input: &String) -> u64 {
    let mut mask = String::new();
    let mut mem = HashMap::new();
    input.lines().for_each(|l| {
        if l.starts_with("mask") {
            mask = Regex::new(r"^mask = ([X0-1]+)$")
                .unwrap()
                .captures(l)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .rev()
                .collect::<String>();
        } else {
            let captures = Regex::new(r"^mem\[(\d+)\] = (\d+)$")
                .unwrap()
                .captures(l)
                .unwrap();
            mem.insert(
                captures.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                apply_bitmask(
                    captures.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                    mask.as_bytes(),
                ),
            );
        }
    });
    mem.iter().fold(0, |sum, (_, v)| sum + v)
}

pub fn part_02(input: &String) -> u64 {
    let mut mask = String::new();
    let mut mem = HashMap::new();
    input.lines().for_each(|l| {
        if l.starts_with("mask") {
            mask = Regex::new(r"^mask = ([X0-1]+)$")
                .unwrap()
                .captures(l)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .rev()
                .collect::<String>();
        } else {
            let captures = Regex::new(r"^mem\[(\d+)\] = (\d+)$")
                .unwrap()
                .captures(l)
                .unwrap();
            let addrs = apply_floating_bitmask(
                captures.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                mask.as_bytes(),
            );
            for addr in addrs {
                mem.insert(
                    addr,
                    captures.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                );
            }
        }
    });
    mem.iter().fold(0, |sum, (_, v)| sum + v)
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
            165
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::practice_02",
                INPUT_PATH
            )))),
            51
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            8332632930672
        );
    }

    #[test]
    fn part_02() {
        // // Keep commented, this one is not meant to work.
        // assert_eq!(super::part_02(&read_resource(relative_input_path(&format!("{}::practice_01", INPUT_PATH)))), );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_02",
                INPUT_PATH
            )))),
            208
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            4753238784664
        );
    }
}
