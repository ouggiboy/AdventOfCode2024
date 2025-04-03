use std::collections::HashSet;

fn is_possible(design: &str, towels: &HashSet<String>, possible: &mut HashSet<String>, impossible: &mut HashSet<String>) -> bool {
    if impossible.contains(design) {
        return false;
    }
    if design.is_empty() || possible.contains(design) {
        return true;
    }
    for towel in towels {
        if design.starts_with(towel) {
            if is_possible(&design[towel.len()..], &towels, possible, impossible) {
                possible.insert(design[towel.len()..].to_string());
                return true;
            }
        }
    }
    impossible.insert(design.to_string());
    false
}   

pub fn run(input: (HashSet<String>, Vec<String>)) {
    let towels = input.0;
    let designs = input.1;
    let mut possible = HashSet::new();
    let mut impossible = HashSet::new();

    let mut c = 0;
    for design in designs {
        if is_possible(&design, &towels, &mut possible, &mut impossible) {
            c += 1;
        }
    }

    println!("Part 1: {}", c);
}