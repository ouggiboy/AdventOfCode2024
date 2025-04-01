use std::fs;

#[cfg(debug_assertions)]
const PATH: &str = "sample.txt";

#[cfg(not(debug_assertions))]
const PATH: &str = "input.txt";

mod part1;
mod part2;

fn parse_input(path: &str) -> (u64, u64, u64, Vec<u8>) {
    let input = fs::read_to_string(path).unwrap();
    let mut lines = input.lines();
    let a = lines.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap();
    let b = lines.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap();
    let c = lines.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap();
    let program = lines.skip(1).next().unwrap().split(": ").nth(1).unwrap().split(',').map(|x| x.parse().unwrap()).collect();

    (a, b, c, program)
}

fn main() {
    let (a, b, c, program) = parse_input(PATH);
    let target = part1::run(a, b, c, &program);
    part2::run(a, b, c, &program, &target);
}