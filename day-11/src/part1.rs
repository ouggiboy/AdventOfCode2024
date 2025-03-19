use std::fs;

fn get_input(path: &str) -> Vec<String> {
    fs::read_to_string(path).unwrap()
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect()
} 

fn remove_leading_zeroes(stone: &str) -> String {
    stone.parse::<u128>().unwrap().to_string()
}

fn transform(input: &Vec<String>) -> Vec<String> {
    let mut new: Vec<String> = Vec::new();
    for i in 0..input.len() {
        if input[i] == "0" { 
            new.push("1".to_string());
        }
        else if input[i].len() % 2 == 0 {
            let mid = input[i].len() / 2;
            // remove the stone and split it into 2
            let halves = input[i].split_at(mid);
            // remove leading zeroes in case we split something like "1000" into "10" and "00"
            new.push(halves.0.to_string());
            new.push(remove_leading_zeroes(halves.1));
            // add to end as the vector just grew
        }
        else {
            new.push((input[i].parse::<u64>().unwrap() * 2024).to_string());
        }
    }
    new
}

pub fn run(path: &str) {
    let mut input = get_input(path);
    for _ in 0..25 {
        input = transform(&input);
    }
    println!("Part 1: {}", input.len())
}