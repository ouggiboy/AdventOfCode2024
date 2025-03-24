use std::fs;

#[cfg(debug_assertions)]
const PATH: &str = "sample.txt";

#[cfg(not(debug_assertions))]
const PATH: &str = "input.txt";

mod part1;
mod part2;

fn parse_input(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path).unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect()
} 

fn main() {
    let input = parse_input(PATH);
    part1::run(&input);
    part2::run(&input);
}