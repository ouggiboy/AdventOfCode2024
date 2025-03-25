use std::fs;

#[cfg(debug_assertions)]
const PATH: &str = "sample.txt";

#[cfg(not(debug_assertions))]
const PATH: &str = "input.txt";

mod part1;
mod part2;

fn parse_input(path: &str) -> Vec<String> {
    fs::read_to_string(path).unwrap()
        .split("\r\n\r\n")
        .map(|x| x.to_string())
        .collect()
}

fn main() {
    let input = parse_input(PATH);
    part1::run(&input);
    part2::run(&input);
}