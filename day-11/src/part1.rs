fn remove_leading_zeroes(stone: &str) -> String {
    stone.parse::<u128>().unwrap().to_string()
}

fn transform(input: &Vec<u64>) -> Vec<u64> {
    let mut new: Vec<u64> = Vec::new();
    for i in 0..input.len() {
        if input[i] == 0 { 
            new.push(1);
        }
        else if input[i].to_string().len() % 2 == 0 {
            let s = input[i].to_string();
            let mid = s.len() / 2;
            // remove the stone and split it into 2
            let halves = s.split_at(mid);
            // remove leading zeroes in case we split something like "1000" into "10" and "00"
            new.push(halves.0.parse().unwrap());
            new.push(remove_leading_zeroes(halves.1).parse().unwrap());
            // add to end as the vector just grew
        }
        else {
            new.push(input[i] * 2024);
        }
    }
    new
}

pub fn run(input: &Vec<u64>) {
    let mut input = input.clone();
    for _ in 0..25 {
        input = transform(&input);
    }
    println!("Part 1: {}", input.len())
}