use std::fs;


const TESTING: bool = false;
const INPUT: &str = if TESTING { 
    "sample.txt"
} else {
    "input.txt"
};

fn get_inputs(path: &str) -> String {
    fs::read_to_string(path).expect("lol")
}

fn get_cmds(data: &str) -> Vec<&str> {
    data.split("mul").collect::<Vec<&str>>()
}

fn is_valid_p1(cmd: &str) -> bool {
    if cmd.chars().nth(0) == Some('(') && cmd.contains(')') {
        return cmd.find(')').is_some()
    }
    false
}

static mut ENABLED: bool = true;

fn is_valid_p2(cmd: &str) -> bool {
    let mut enabled_after_cmd = true;
    unsafe {
        if cmd.contains("don't()") {
            enabled_after_cmd = false;
        }
        if cmd.contains("do()") {
            if !ENABLED {
                ENABLED = true;
                return false;
            }
        }
        if cmd.chars().nth(0) == Some('(') && cmd.contains(')') && ENABLED {
            ENABLED = enabled_after_cmd;
            return cmd.find(')').is_some()
        }
        if !enabled_after_cmd {
            ENABLED = false;
        }
    }
    false
}

fn get_mul(cmd: &str) -> u64 {
    let end = cmd.find(')').unwrap();
    let nums = cmd.get(1..end).unwrap();
    let product = nums
        .split(',')
        .into_iter()
        .map(|i| i.parse::<u64>().unwrap_or(0))
        .product();

    product
}


fn main() {
    let data = get_inputs(INPUT);
    let mul_cmds = get_cmds(&data);

    let valid_instructions_p1 = mul_cmds.clone()
        .into_iter()
        .filter(|&cmd| is_valid_p1(cmd))
        .collect::<Vec<&str>>();

    let sum_p1 = valid_instructions_p1
        .iter()
        .map(|&cmd| get_mul(cmd))
        .sum::<u64>();

    let valid_instructions_p2 = mul_cmds
        .into_iter()
        .filter(|&cmd| is_valid_p2(cmd))
        .collect::<Vec<&str>>();

    let sum_p2 = valid_instructions_p2
        .iter()
        .map(|&cmd| get_mul(cmd))
        .sum::<u64>();


    println!("Part 1: {}", sum_p1);
    println!("Part 2: {}", sum_p2);
}