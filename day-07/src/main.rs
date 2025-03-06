use std::fs;

#[cfg(debug_assertions)]
const INPUT: &str = "sample.txt";

#[cfg(not(debug_assertions))]
const INPUT: &str = "input.txt";

fn get_lines(path: &str) -> Vec<String> {
    fs::read_to_string(path).unwrap().split('\n').map(|x| x.trim().to_string()).collect::<Vec<String>>()
}

fn transform_to_tuple(line: &str) -> (u64, Vec<u64>) {
    let mut line_iterator = line.split(':');

    let value: u64 = line_iterator.next().unwrap()
        .parse().expect("Something is wrong with input");

    let numbers: Vec<u64> = line_iterator.next().unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse().expect("Something is wrong with input"))
        .collect();

    (value, numbers)
}

fn validate(target: u64, current: u64, nums: &Vec<u64>, i: usize) -> bool {
    if i == nums.len() {
        return current == target;
    }
    if current > target {
        return false;
    }
    return validate(target, current * nums[i], nums, i + 1)
        || validate(target, current + nums[i], nums, i + 1)
        // this is only for part 2
        || validate(target, concat(current, nums[i]), nums, i + 1)

}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.to_string().len() as u32) + b
}

fn main() {
    let lines: Vec<(u64, Vec<u64>)> = get_lines(INPUT)
        .iter()
        .map(|line| transform_to_tuple(line))
        .collect();

    let total_result: u64 = lines.iter()
        .filter(|&line| validate(line.0, 0, &line.1, 0))
        .map(|valid_line| valid_line.0)
        .sum();

    println!("{}", total_result);
}
