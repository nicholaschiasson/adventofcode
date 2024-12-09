pub fn part_01(input: &str) -> u64 {
    let mut files = input
        .chars()
        .flat_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        .step_by(2)
        .collect::<Vec<u64>>();

    let mut free = input
        .chars()
        .flat_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        .skip(1)
        .step_by(2)
        .collect::<Vec<u64>>();

    let mut blocks = Vec::new();
    let mut i = 0;
    let mut j = files.len() - 1;

    while i < files.len() {
        while files[i] > 0 {
            blocks.push(i as u64);
            files[i] -= 1;
        }
        if let Some(k) = free.get_mut(i) {
            while *k > 0 && j > 0 {
                if files[j] > 0 {
                    blocks.push(j as u64);
                    files[j] -= 1;
                    *k -= 1;
                } else {
                    j -= 1;
                }
            }
        }
        i += 1;
    }

    blocks
        .into_iter()
        .enumerate()
        .fold(0, |a, (i, n)| a + (i as u64 * n))
}

#[derive(Clone, Copy, Debug)]
enum Segment {
    File(usize, u64),
    Free(usize),
}

pub fn part_02(input: &str) -> u64 {
    let mut segments = input
        .chars()
        .flat_map(|c| c.to_digit(10))
        .map(|d| d as usize)
        .enumerate()
        .map(|(i, n)| {
            if i % 2 == 0 {
                Segment::File(n, i as u64 / 2)
            } else {
                Segment::Free(n)
            }
        })
        .collect::<Vec<_>>();

    let mut i = 0;
    while i < segments.len() {
        if let Segment::Free(free) = segments[i] {
            let mut j = segments.len() - 1;
            while j > i {
                if let Segment::File(size, id) = segments[j] {
                    if let Some(rem) = free.checked_sub(size) {
                        segments[j] = Segment::Free(size);
                        let file = Segment::File(size, id);
                        if rem > 0 {
                            segments[i] = Segment::Free(rem);
                            segments.insert(i, file);
                        } else {
                            segments[i] = file;
                        }
                        break;
                    }
                }
                j -= 1;
            }
        }
        i += 1;
    }

    segments
        .into_iter()
        .flat_map(|segment| match segment {
            Segment::File(size, id) => vec![id; size],
            Segment::Free(size) => vec![0; size],
        })
        .enumerate()
        .fold(0, |a, (i, b)| a + (i as u64 * b))
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
            1928
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            6395800119709
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            2858
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            6418529470362
        );
    }
}
