fn nums_from_str(s: &str) -> (u64, u64) {
    let pair = s
        .split(',')
        .map(|x| x.chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap())
        .collect::<Vec<u64>>();
    (pair[0], pair[1])
}

fn get_parts(group: &str) -> ((u64, u64), (u64, u64), (u64, u64)) {
    let mut lines = group.lines();
    (
        nums_from_str(lines.next().unwrap()),
        nums_from_str(lines.next().unwrap()),
        nums_from_str(lines.next().unwrap()),
    )
}

pub fn run(input: &Vec<String>) {
    for group in input {
        let ((ax, ay), (bx, by), (px, py)) = get_parts(&group);
        
    }
}