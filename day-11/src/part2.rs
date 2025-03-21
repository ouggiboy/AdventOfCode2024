use std::collections::HashMap;

fn get_stone_count(stone: u64, steps: usize, mem: &mut HashMap<(u64, usize), u64>) -> u64 {
    // if in memory, return the value
    if mem.contains_key(&(stone, steps)) {
        return mem[&(stone, steps)];
    }
    // if at first step, just return 1 as there is only one stone in the beginning and the value doesn't matter
    if steps == 0 {
        return 1;
    }

    let num_str = stone.to_string();
    let result;

    if stone == 0 { 
        result = get_stone_count(1, steps - 1, mem);
    }
    else if num_str.len() % 2 == 0 {
        let l_stone = num_str[..num_str.len() / 2].parse::<u64>().unwrap();
        let r_stone = num_str[num_str.len() / 2..].parse::<u64>().unwrap();

        result = get_stone_count(l_stone, steps - 1, mem) + get_stone_count(r_stone, steps - 1, mem);
    }
    else {
        result = get_stone_count(stone * 2024, steps - 1, mem)
    }

    mem.insert((stone, steps), result);
    result
}

pub fn run(input: &Vec<u64>) {
    // keep track of already seen positions, that includes the amount of stones that a stone at a exact step produces in the end
    //                   stone step   endsum
    let mut mem: HashMap<(u64, usize), u64> = HashMap::new();
    let mut sum = 0;

    for stone in input {
        sum += get_stone_count(*stone, 75, &mut mem)
    }
    println!("Part 2: {}", sum)
}