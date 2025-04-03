use std::collections::{HashSet, HashMap};

fn is_possible(design: &str, towels: &HashSet<String>, mem: &mut HashMap<String, u64>) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(&c) = mem.get(design) {
        return c;
    }
    let mut result = 0;
    for towel in towels {
        if design.starts_with(towel) {
            result += is_possible(&design[towel.len()..], &towels, mem);
        }
    }
    mem.insert(design.to_string(), result);
    result
}   

pub fn run(input: (HashSet<String>, Vec<String>)) {
    let towels = input.0;
    let designs = input.1;
    let mut mem = HashMap::new();

    let mut c = 0;
    for design in designs {
        c += is_possible(&design, &towels, &mut mem);
    }

    println!("Part 1: {}", c);
}