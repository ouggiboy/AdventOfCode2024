fn nums_from_str(s: &str) -> (f64, f64) {
    let pair = s
        .split(',')
        .map(|x| x.chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap())
        .collect::<Vec<f64>>();
    (pair[0], pair[1])
}

fn get_parts(group: &str) -> ((f64, f64), (f64, f64), (f64, f64)) {
    let mut lines = group.lines();
    (
        nums_from_str(lines.next().unwrap()),
        nums_from_str(lines.next().unwrap()),
        nums_from_str(lines.next().unwrap()),
    )
}

pub fn run(input: &Vec<String>) {
    let mut total = 0;
    for group in input {
        let ((ax, ay), (bx, by), (mut px, mut py)) = get_parts(&group);
        px += 10000000000000.0;
        py += 10000000000000.0;
        /* we get a pair of equations
        ax*s + bx*t = px
        ay*s + by*t = py
        where s is the amount of times we press A button and t is the amount of presses for B button */
        let s = (px * by - py * bx) / (ax * by - ay * bx);
        let t = (px - ax * s) / bx;
        // our answers can only be whole numbers
        if s.trunc() == s && t.trunc() == t {
            total += 3 * s as u64 + t as u64;
        }
    }
    println!("Part 2: {}", total);
}