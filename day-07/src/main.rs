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

fn is_valid(line: &(u64, Vec<u64>)) -> bool {
    let target = line.0;
    let nums = &line.1;

    // map to keep track of what we've tried
    let mut map = vec!['+'; nums.len() - 1];

    loop {       
        let mut result = nums[0];

        // now calculate whether we add or multiply the first number for each number in vector
        for i in 0..map.len() {
            if map[i] == '+' {
                result += nums[i + 1];
            }
            else {
                result *= nums[i + 1];
            }
        }
        // check if this combination was correct
        if result == target {
            return true
        }
        // or if we are at the end of combinations
        else if map == vec!['*'; nums.len() - 1] {
            return false
        }
        // else start changing map to multiplication kind of like a binary numbers counts up
        // check where is first '+' char and change it to '*' char, then change all before it back to '+'
        else {
            let index = map.iter().position(|&c| c == '+').unwrap();
            map[index] = '*';
            let slice = &mut map[..index];
            slice.iter_mut().for_each(|c| *c = '+');
        }
    }
}

fn main() {
    let lines: Vec<(u64, Vec<u64>)> = get_lines(INPUT)
        .iter()
        .map(|line| transform_to_tuple(line))
        .collect();

    let total_result: u64 = lines.iter()
        .filter(|&line| is_valid(line))
        .map(|valid_line| valid_line.0)
        .sum();

    println!("{}", total_result);
}
