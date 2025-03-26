use std::fs;

#[cfg(debug_assertions)]
const PATH: &str = "sample.txt";

#[cfg(not(debug_assertions))]
const PATH: &str = "input.txt";

mod part1;
mod part2;

fn parse_input(path: &str) -> Vec<[(i64, i64); 2]> {
    let pair_map: Vec<Vec<(i64, i64)>> = fs::read_to_string(path).unwrap()
        .lines()
        .map(|x| { x.split_ascii_whitespace()
            .map(|half| { 
                let mut iter = half.split(',');
                let x: i64 = iter.next().unwrap()[2..].parse().unwrap();
                let y: i64 = iter.next().unwrap().parse().unwrap();
                (x, y)
            })
            .collect()
        })
        .collect();
    let mut pairs: Vec<[(i64, i64); 2]> = Vec::new();
    for pair in pair_map {
        pairs.push([pair[0], pair[1]]);
    }
    pairs
}

fn main() {
    let input = parse_input(PATH);
    part1::run(&input);
    part2::run(&input);
}