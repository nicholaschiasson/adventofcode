fn calculate_fuel(mass: i64) -> u64 {
    let fuel = mass / 3 - 2;
    if fuel < 0 {
        0
    } else {
        fuel as u64
    }
}

fn try_calculate_fuel(mass: i64) -> Option<u64> {
    let fuel = calculate_fuel(mass);
    if fuel > 0 {
        Some(fuel)
    } else {
        None
    }
}

pub fn part_01(input: &String) -> u64 {
    input.lines().fold(0, |fuel, mass| {
        fuel + calculate_fuel(mass.parse::<i64>().expect("Failed to parse mass"))
    })
}

pub fn part_02(input: &String) -> u64 {
    input.lines().fold(0, |fuel, mass| {
        let mut fuel_mass = calculate_fuel(mass.parse::<i64>().expect("Failed to parse mass"));
        let mut sum_fuel = fuel + fuel_mass;
        while let Some(added_fuel) = try_calculate_fuel(fuel_mass as i64) {
            sum_fuel += added_fuel;
            fuel_mass = added_fuel;
        }
        sum_fuel
    })
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
            34241
        );
        assert_eq!(
            super::part_01(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            3393938
        );
    }

    #[test]
    fn part_02() {
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::practice_01",
                INPUT_PATH
            )))),
            51316
        );
        assert_eq!(
            super::part_02(&read_resource(relative_input_path(&format!(
                "{}::final",
                INPUT_PATH
            )))),
            5088037
        );
    }
}
