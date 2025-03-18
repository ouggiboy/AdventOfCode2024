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
    // find the series of numbers' length from the back and check the first series of dots' length that is the same or bigger than numbers'
    let mut end = disk.len() - 1;
    'sort: while end > 0 {
        if disk[end] != "." 
        {
            let curr = disk[end].clone();
            let last_num = end;
            let mut n_len = 0;
            // find num-series
            while disk[end] == *curr {
                n_len += 1;
                if end == 0 {
                    break 'sort;
                }
                end -= 1;
            }

            // now find corresponding dot-series
            let mut start = 0;
            while start < last_num {
                let mut d_len = 0;
                
                if disk[start] == "." {
                    let first_dot = start;
                    while disk[start] == "." {
                        d_len += 1;
                        start += 1;
                    }
                    // swap dots and numbers if a spot is found
                    if d_len >= n_len {
                        for x in 0..n_len {
                            disk.swap(first_dot + x, last_num - x);
                        }
                        break;
                    }
                }

                start += 1;
            }

            // need to give 1 back in case another series continues right after
            end += 1;
        }
        end -= 1;
    }
} 

fn check_sum(disk: &Vec<String>) -> u64 {
    let mut sum = 0;
    for i in 0..disk.len() {
        if disk[i] != "." {
            sum += i as u64 * disk[i].parse::<u64>().unwrap();
        }
    }
    sum
}

pub fn run(file: &str) {
    let mut disk = get_disk(file);
    sort_disk(&mut disk);
    println!("Part 2: {}", check_sum(&disk));
}