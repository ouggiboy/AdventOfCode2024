fn get_output(a: u64) -> (u64, u64) {
    // 2,4; 1,1
    let mut b = (a & 7) ^ 1;
    // 7,5
    let c = a >> b;
    // 1,5; 4,1
    b = (b ^ 5) ^ c;
    // 5, 5
    let out = b & 7;
    // 0,3
    let test_a = a >> 3;
    // 3, 0
    // return this output and new a
    (test_a, out)
}

fn solve(program: &Vec<u64>) -> u64 {

    let mut queue = Vec::<(usize, u64)>::new();
    queue.push((program.len(), 0));

    let mut options = Vec::new();

    while !queue.is_empty() {
        let (index, old_a) = queue.remove(0);

        if index == 0 {
            options.push(old_a);
            continue;
        }

        let next_index = index - 1;
        let target = program[next_index];

        // literally spent 3 hours trying to fix my code to find out i had the loop only from 0 to 6 instead of 7
        // I do not know why it took me so long to print i along all the other stuff I declared
        for i in 0..8 {    
            let next_a = old_a << 3 | i;
            let (test_a, out) = get_output(next_a);
            if test_a == old_a && out == target {
                queue.push((next_index, next_a));
            }
        }
    }
    *options.iter().min().unwrap()
}

pub fn run() {
    let program = vec![2,4,1,1,7,5,1,5,4,1,5,5,0,3,3,0];
    println!("Part 2: {}", solve(&program));
}