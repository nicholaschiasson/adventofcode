use std::{env, fs};

mod days;

use days::day_01::*;

fn main() {
    let input_file = env::current_dir()
        .expect("Failed to get current directory")
        .join("rsrc/input_01.txt");
    let input = fs::read_to_string(input_file).expect("Failed to read file.");
    println!("{}", day_01_01(&input));
    println!("{}", day_01_02(&input));
}
