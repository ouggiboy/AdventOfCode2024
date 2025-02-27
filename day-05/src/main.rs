use std::fs;

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


fn main() {
    let data = fs::read_to_string(INPUT).expect("Couldn't read file:(");
    let x = get_parts(&data);
    println!("{:?}", x);
}   
