use std::fs;
use std::collections::HashMap;

const TESTING: bool = true;
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


fn main() {
    let data = fs::read_to_string(INPUT).expect("Couldn't read file:(");
    let parts = get_parts(&data);
    let order_map = make_map(parts.0);
    println!("{:?}", order_map);
}   
