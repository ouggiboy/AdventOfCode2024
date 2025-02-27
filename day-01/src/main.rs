use std::fs;

const TESTING: bool = false;
const INPUT: &str = if TESTING {
    "sample.txt"
} else {
    "input.txt"
};

fn get_sorted_lists_from_input(path: &str) -> (Vec<u64>, Vec<u64>) {
    let file = fs::read_to_string(path).expect("Couldn't read file");
    let input_arr: Vec<u64> = file.split_ascii_whitespace().map(|x| x.parse::<u64>().expect("Couldn't parse number.")).collect();
    let mut left: Vec<u64> = Vec::new();
    let mut right: Vec<u64> = Vec::new();

    for i in 0..input_arr.len() {
        if i % 2 == 0 {
            left.push(input_arr[i]);
        } else {
            right.push(input_arr[i]);
        }
    }
    left.sort();
    right.sort();

    (left, right)
}

fn difference(left: u64, right: u64) -> u64 {
    left.abs_diff(right)
}

fn calc_similarity_score(n: u64, right: &Vec<u64>) -> u64 {
    right.iter().filter(|&x| *x == n).count() as u64 * n
}

fn main() {
    let (left, right) = get_sorted_lists_from_input(INPUT);
    let mut total_dist = 0;
    let mut similarity_score = 0;
    for i in 0..left.len() {
        total_dist += difference(left[i], right[i]);
        similarity_score += calc_similarity_score(left[i], &right)
    }
    println!("Total distance between left and right list is: {}", total_dist);
    println!("Similarity score is: {}", similarity_score);
}

