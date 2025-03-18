use std::fs;

fn get_disk(file: &str) -> Vec<String> {
    let input = fs::read_to_string(file).unwrap();

    // transforming to a number vector for easier handling
    let v = input.chars().map(|x| x as u8 - 48).collect::<Vec<u8>>();

    let mut block_arr: Vec<String> = Vec::new();

    for i in 0..v.len() {
        // if we are at length of file
        if i % 2 == 0 {
            // i / 2 is the current id number
            for _ in 0..v[i] {
                block_arr.push((i as u32 / 2).to_string());
            }
        }
        else {
            for _ in 0..v[i] {
                block_arr.push(".".to_string());
            }
        }
    }

    block_arr
}

fn sort_disk(disk: &mut Vec<String>) {
    // iterate from the back, finding the first non "." string, and swapping it with first "." from the front, then continue from that point onwards
    let mut end = disk.len() - 1;
    while end > 0 {
        if disk[end] != "." {
            if let Some(dot) = disk.iter().position(|x| x == ".") {
                // check if we can not move anymore and break if so
                if dot >= end {
                    break;
                }
                // else swap the dot and number
                disk.swap(dot, end);
            }
        }
        end -= 1;
    }
} 

fn check_sum(disk: &Vec<String>) -> u64 {
    let mut i = 0;
    let mut sum = 0;
    while disk[i] != "." {
        sum += i as u64 * disk[i].parse::<u64>().unwrap();
        i += 1;
    }
    sum
}

pub fn run(file: &str) {
    let mut disk = get_disk(file);
    sort_disk(&mut disk);
    println!("Part 1: {}", check_sum(&disk));
}