use std::fs;

#[cfg(debug_assertions)]
const PATH: &str = "sample.txt";

#[cfg(not(debug_assertions))]
const PATH: &str = "input.txt";

mod part1;
mod part2;

fn get_input(path: &str) -> Vec<u64> {
    fs::read_to_string(path).unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
} 

fn main() {
    let input = get_input(PATH);
    part1::run(&input);
    part2::run(&input);
}