use std::{collections::HashSet, fmt, str::FromStr};

#[derive(Clone, Debug)]
enum Instruction {
    Accumulate(i32),
    Jump(i32),
    NoOperation(i32),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = s[4..].parse::<i32>() {
            return match &s[0..3] {
                "acc" => Ok(Instruction::Accumulate(val)),
                "jmp" => Ok(Instruction::Jump(val)),
                "nop" => Ok(Instruction::NoOperation(val)),
                _ => Err(ParseInstructionError),
            };
        }
        Err(ParseInstructionError)
    }
}

#[derive(Debug, Clone)]
struct ParseInstructionError;

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse instruction")
    }
}

struct Boot {
    accumulator: i32,
    current_instruction: i32,
    history: HashSet<i32>, // Not really history, just a set of previously run instructions
    instructions: Vec<Instruction>,
}

impl Boot {
    fn new(instructions: Vec<Instruction>) -> Self {
        Boot {
            accumulator: 0,
            current_instruction: 0,
            history: HashSet::new(),
            instructions,
        }
    }

    fn try_step(&mut self) -> Result<Option<i32>, i32> {
        if self.current_instruction as usize >= self.instructions.len() {
            Ok(Some(self.accumulator))
        } else if !self.history.insert(self.current_instruction) {
            Err(self.accumulator)
        } else {
            match self.instructions[self.current_instruction as usize] {
                Instruction::Accumulate(v) => {
                    self.accumulator += v;
                    self.current_instruction += 1;
                }
                Instruction::Jump(v) => {
                    self.current_instruction += v;
                }
                Instruction::NoOperation(_) => {
                    self.current_instruction += 1;
                }
            }
            Ok(None)
        }
    }

    fn run(&mut self) -> Result<Option<i32>, i32> {
        loop {
            match self.try_step() {
                Ok(Some(v)) => {
                    return Ok(Some(v));
                }
                Err(e) => {
                    return Err(e);
                }
                _ => {}
            }
        }
    }
}

fn parse_instructions(instructions: &String) -> Vec<Instruction> {
    instructions
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect()
}

pub fn part_01(input: &String) -> u64 {
    let mut boot = Boot {
        accumulator: 0,
        current_instruction: 0,
        history: HashSet::new(),
        instructions: parse_instructions(input),
    };
    loop {
        if let Err(a) = boot.try_step() {
            return a as u64;
        }
    }
}

pub fn part_02(input: &String) -> i64 {
    let mut a = None;
    let instructions = parse_instructions(input);
    instructions
        .iter()
        .enumerate()
        .filter(|(_, i)| match i {
            Instruction::Jump(_) | Instruction::NoOperation(_) => true,
            _ => false,
        })
        .find(|(n, _)| {
            let mut inst = instructions.clone();
            if let Instruction::Jump(v) = inst[*n] {
                inst[*n] = Instruction::NoOperation(v);
            } else if let Instruction::NoOperation(v) = inst[*n] {
                inst[*n] = Instruction::Jump(v);
            }
            match Boot::new(inst).run() {
                Ok(Some(v)) => {
                    a = Some(v);
                    true
                }
                _ => false,
            }
        });
    a.unwrap() as i64
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
            5
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            1949
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            8
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            2092
        );
    }
}
