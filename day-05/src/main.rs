use std::fs;
use std::collections::HashMap;

const TESTING: bool = false;
const INPUT: &str = if TESTING {
    "sample.txt"
} else {
    "input.txt"
};

fn get_parts(data: &str) -> (Vec<&str>, Vec<&str>) {
    let lines = data.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
    let split = lines.iter().position(|&x| x == "").unwrap();

    (lines[..split].to_vec(), lines[split + 1..].to_vec())
}

fn make_map(upper_part: Vec<&str>) -> HashMap<u8, Vec<u8>> {
    let mut map: HashMap<u8, Vec<u8>> = HashMap::new();
    for rule in upper_part {
        let mut parts = rule.split("|").map(|x| x.parse::<u8>().unwrap());
        let (k, v) = (parts.next().unwrap(), parts.next().unwrap());
        let prev = map.insert(k, vec![v]);
        if let Some(mut vector) = prev {
            vector.push(v);
            map.insert(k, vector);
        }
    }

    map
}

fn correct_order_update(update: &str, map: &HashMap<u8, Vec<u8>>) -> bool {
    let mut prev_nums = Vec::<u8>::new();

    for n in update.split(',').map(|i| i.parse::<u8>().unwrap()) {
        if let Some(v) = map.get(&n) {
            for rule in v {
                if prev_nums.contains(rule) {
                    return false;
                }
            }
        }
        prev_nums.push(n);
    }
    true
} 

fn get_middle_page(update: &str) -> u64 {
    let nums = update.split(',').map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    nums[(nums.len() - 1) / 2]
}


fn main() {
    let data = fs::read_to_string(INPUT).expect("Couldn't read file:(");
    let parts = get_parts(&data);
    let map = make_map(parts.0);
    let correct_updates = parts.1.into_iter().filter(|&u| correct_order_update(u, &map)).collect::<Vec<&str>>();
    let page_sum = correct_updates.iter().map(|&u| get_middle_page(u)).sum::<u64>();

    println!("Sum of middle pages is: {}", page_sum);
}   
