#[cfg(debug_assertions)]
const INPUT: &str = "sample.txt";

#[cfg(not(debug_assertions))]
const INPUT: &str = "input.txt";

mod part1;
mod part2;

fn main() {
    part1::run(INPUT);
    part2::run(INPUT);
}