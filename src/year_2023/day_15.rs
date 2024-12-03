use std::collections::{HashMap, VecDeque};

fn holiday_ascii_string_helper<S: AsRef<str>>(s: &S) -> u8 {
    s.as_ref()
        .bytes()
        .fold(0_u16, |h, b| ((h + b as u16) * 17) % 256) as u8
}

fn holiday_ascii_string_helper_manual_arrangement_procedure(s: &str) -> HolidayHashMap<&str, u8> {
    let mut map = HolidayHashMap::new();
    for step in s.split(',') {
        match step.split_once(['=', '-']) {
            Some((label, focal_length)) => {
                if let Ok(fl) = focal_length.parse::<u8>() {
                    map.insert(label, fl);
                } else {
                    map.remove(&label);
                }
            }
            _ => panic!("gross. you call that trash input?"),
        }
    }
    map
}

#[derive(Debug)]
struct HolidayHashMap<K: AsRef<str> + PartialEq, V: Copy> {
    keys: [VecDeque<K>; 256],
    values: [VecDeque<V>; 256],
}

impl<K: AsRef<str> + Clone + PartialEq, V: Copy> HolidayHashMap<K, V> {
    const KEYS_DEQUE: VecDeque<K> = VecDeque::new();
    const VALUES_DEQUE: VecDeque<V> = VecDeque::new();
    fn new() -> Self {
        Self {
            keys: [Self::KEYS_DEQUE; 256],
            values: [Self::VALUES_DEQUE; 256],
        }
    }
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut out = None;
        let index = holiday_ascii_string_helper(&key) as usize;
        if let Some(i) = self.keys[index].iter().position(|k| key.eq(k)) {
            out = Some(self.values[index][i]);
            self.keys[index][i] = key;
            self.values[index][i] = value;
        } else {
            self.keys[index].push_back(key);
            self.values[index].push_back(value);
        }
        out
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        let index = holiday_ascii_string_helper(key) as usize;
        if let Some(i) = self.keys[index].iter().position(|k| k == key) {
            self.keys[index].remove(i);
            self.values[index].remove(i);
            return None;
        }
        None
    }
}

impl<K: AsRef<str> + Clone + PartialEq, V: Copy> IntoIterator for HolidayHashMap<K, V> {
    type Item = (K, V);
    type IntoIter = HolidayHashMapIntoIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        HolidayHashMapIntoIterator {
            holiday_hash_map: self,
            index: 0,
            hash_index: 0,
        }
    }
}

pub struct HolidayHashMapIntoIterator<K: AsRef<str> + Clone + PartialEq, V: Copy> {
    holiday_hash_map: HolidayHashMap<K, V>,
    index: usize,
    hash_index: usize,
}

impl<K: AsRef<str> + Clone + PartialEq, V: Copy> Iterator for HolidayHashMapIntoIterator<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        let mut out = None;
        while self.index < self.holiday_hash_map.keys.len()
            && self.hash_index >= self.holiday_hash_map.keys[self.index].len()
        {
            self.index += 1;
            self.hash_index = 0;
        }
        if self.index < self.holiday_hash_map.keys.len() {
            out = Some((
                self.holiday_hash_map.keys[self.index][self.hash_index].clone(),
                self.holiday_hash_map.values[self.index][self.hash_index],
            ))
        }
        self.hash_index += 1;
        out
    }
}

pub fn part_01(input: &str) -> u64 {
    input
        .split(',')
        .fold(0, |s, step| s + holiday_ascii_string_helper(&step) as u64)
}

pub fn part_02(input: &str) -> u64 {
    let mut label_count: HashMap<u8, u64> = HashMap::new();
    holiday_ascii_string_helper_manual_arrangement_procedure(input)
        .into_iter()
        .fold(0, |sum, (l, lens)| {
            let hash = holiday_ascii_string_helper(&l);
            *label_count.entry(hash).or_default() += 1;
            sum + ((1 + hash as u64) * label_count[&hash] * lens as u64)
        })
}

#[cfg(test)]
mod tests {
    use crate::utils::{read_resource, relative_input_path};

    const INPUT_PATH: &str = module_path!();

    #[test]
    fn part_01() {
        assert_eq!(super::part_01("HASH"), 52);
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            1320
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            504449
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::practice_01"
            )))),
            145
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{INPUT_PATH}::final"
            )))),
            262044
        );
    }
}
