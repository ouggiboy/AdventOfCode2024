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

fn make_map(rules: Vec<&str>) -> HashMap<u8, Vec<u8>> {
    let mut map: HashMap<u8, Vec<u8>> = HashMap::new();
    for rule in rules {
        let mut parts = rule
            .split("|")
            .map(|x| x.parse::<u8>().unwrap());
        let (k, v) = (parts.next().unwrap(), parts.next().unwrap());
        let prev = map.insert(k, vec![v]);
        if let Some(mut vector) = prev {
            vector.push(v);
            map.insert(k, vector);
        }
    }

    map
}

fn is_correct_order(page: &str, map: &HashMap<u8, Vec<u8>>) -> bool {
    let mut prev_nums = Vec::<u8>::new();

    for n in page.split(',').map(|i| i.parse::<u8>().unwrap()) {
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

fn page_to_vec(page: &str) -> Vec<u8> {
    page
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}

fn reorder_page(page: &str, map: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
    // turn it around to work from end to beginning to sort it easier
    let mut new_page = page_to_vec(page);
    new_page.reverse();
    let len = new_page.len();
    // if last number doesnt have pages after it that appear before it keep it in place and so on
    let mut i = 0;
    while i < len - 1 {
        if let Some(v) = map.get(&new_page[i]) {
            'inner: for j in i + 1..len {
                // check if incorrect order
                if v.contains(&new_page[j]) {
                    // swap the two incorrect ones between themselves
                    new_page.swap(i, j);
                    // continue sorting from the same spot after swap because all before it has already been checked
                    // need to use wrapping sub in case new_page[0] gets swapped
                    // alternative is to just reset i to 0 everytime but i think this is more efficient
                    i = i.wrapping_sub(1);
                    break 'inner;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    // reverse back
    new_page.reverse();
    new_page
}

fn get_middle_page(page: &str) -> u64 {
    let nums = page_to_vec(page);
    nums[(nums.len() - 1) / 2] as u64
}


fn main() {
    let data = fs::read_to_string(INPUT).expect("Couldn't read file:(");
    let parts = get_parts(&data);
    let map = make_map(parts.0);

    let correct = parts.1.clone()
        .into_iter()
        .filter(|&u| is_correct_order(u, &map))
        .collect::<Vec<&str>>();

    let correct_sum = correct
        .iter()
        .map(|&u| get_middle_page(u))
        .sum::<u64>();
    
    println!("Sum of correct middle pages is: {}", correct_sum);

    let incorrect = parts.1.clone()
        .into_iter()
        .filter(|&u| !is_correct_order(u, &map))
        .collect::<Vec<&str>>();

    let reordered = incorrect
        .iter()
        .map(|&u| reorder_page(u, &map))
        .collect::<Vec<Vec<u8>>>();

    let reordered_sum = reordered
        .iter()
        .map(|u| u[(u.len() - 1) / 2] as u64)
        .sum::<u64>();

    println!("Sum of reordered middle pages is: {}", reordered_sum);

}   
