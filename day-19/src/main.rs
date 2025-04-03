use std::{collections::HashSet, fs};

#[cfg(debug_assertions)]
const PATH: &str = "sample.txt";

#[cfg(not(debug_assertions))]
const PATH: &str = "input.txt";

mod part1;
mod part2;

fn parse_input(path: &str) -> (HashSet<String>, Vec<String>) {
    let input: String = fs::read_to_string(path).unwrap();
    let mut lines = input.lines();

    let towels = lines.next().unwrap().split(", ").map(|x| x.to_string()).collect();
    let designs = lines.skip(1).map(|x| x.to_string()).collect();

    (towels, designs)
}

fn main() {
    let input = parse_input(PATH);
    part1::run(input.clone());
    part2::run(input.clone());
}