use std::fs;

#[cfg(debug_assertions)]
const MAX_COORDS: usize = 6;
#[cfg(debug_assertions)]
const PATH: &str = "sample.txt";

#[cfg(not(debug_assertions))]
const MAX_COORDS: usize = 70;
#[cfg(not(debug_assertions))]
const PATH: &str = "input.txt";

mod part1;
mod part2;

fn parse_input(path: &str) -> Vec<(usize, usize)> {
    let input: String = fs::read_to_string(path).unwrap();
    input.lines()
    .map(|x| {
        let mut halves = x.split(',');
        (halves.next().unwrap().parse().unwrap(), halves.next().unwrap().parse().unwrap())
    })
    .collect()
}

fn main() {
    let input = parse_input(PATH);
    part1::run(&input);
    part2::run(&input);
}