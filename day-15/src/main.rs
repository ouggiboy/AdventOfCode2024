use std::fs;

#[cfg(debug_assertions)]
const PATH: &str = "sample.txt";

#[cfg(not(debug_assertions))]
const PATH: &str = "input.txt";

mod part1;
mod part2;

fn parse_input(path: &str) -> (Vec<Vec<char>>, String) {
    let input: Vec<String> = fs::read_to_string(path).unwrap().split("\r\n\r\n").map(|x| x.to_string()).collect();
    let map = input[0].lines().into_iter().map(|x| x.chars().collect()).collect();
    let cmds = input[1].lines().collect();

    (map, cmds)
}

fn main() {
    let input = parse_input(PATH);
    part1::run(&input);
    part2::run(&input);
}