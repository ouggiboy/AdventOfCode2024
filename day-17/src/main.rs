// use std::fs;

// #[cfg(debug_assertions)]
// const PATH: &str = "sample.txt";

// #[cfg(not(debug_assertions))]
// const PATH: &str = "input.txt";

mod part1;
mod part2;

fn main() {
    part1::run();
    part2::run();
}