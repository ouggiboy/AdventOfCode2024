pub fn run() -> Vec<u64> {
    let mut output = Vec::new();
    let mut a = 17323786;
    // this is the program I wrote it out by hand according to the input
    loop {
        // 2,4; 1,1
        let mut b = (a & 7) ^ 1;
        // 7,5
        let c = a >> b;
        // 1,5; 4,1
        b = (b ^ 5) ^ c;
        // 5, 5
        output.push(b & 7);
        // 0,3
        a >>= 3;
        // 3,0
        if a == 0 {
            break;
        }
    }
    println!("Part 1: {}", output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
    output
}