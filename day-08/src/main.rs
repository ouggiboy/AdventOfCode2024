use std::fs;

#[cfg(debug_assertions)]
const INPUT: &str = "sample.txt";

#[cfg(not(debug_assertions))]
const INPUT: &str = "input.txt";

fn get_map(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn find_matches(c: char, start_pos: (usize, usize), map: &Vec<Vec<char>>, width: usize, height: usize) -> Vec<(usize, usize)> {

    let mut matches = Vec::new();
    // skip start pos
    let mut x = start_pos.0 + 1;
    if x == height { x = 0; }

    for y in start_pos.1..height {
        while x < width {
            if map[y][x] == c {
                matches.push((x, y));
            }    
            x += 1;
        }
        x = 0;
    }
    matches
}

fn get_distance(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    (a.0.abs_diff(b.0), a.1.abs_diff(b.1))
}

fn check_if_node_valid(pos: (usize, usize), nodemap: &mut Vec<(usize, usize)>) -> bool {
    if nodemap.contains(&pos) {
        return false;
    }
    nodemap.push(pos);
    true
}

// return value indicates if we added no new antinodes, or 1 or 2 new ones
// 'a' is always the first antenna
fn mark_antinodes(a: (usize, usize), b: (usize, usize), width: usize, height: usize, nodemap: &mut Vec<(usize, usize)>) -> (bool, bool) {

    let (mut n1, mut n2) = (false, false);

    let d = get_distance(a, b);

    if d == (0, 0) {
        return (n1, n2);
    }
    
    // if on same vertical line
    if d.0 == 0 {
        if a.1 >= d.1 {
            n1 = check_if_node_valid((a.0, a.1 - d.1), nodemap);
        }
        if b.1 < height - d.1 {
            n2 = check_if_node_valid((b.0, b.1 + d.1), nodemap);
        }
    }
    // if on same horizontal line
    else if d.1 == 0 {
        if a.0 >= d.0 {
            n1 = check_if_node_valid((a.0 - d.0, a.1), nodemap);
        }
        if b.0 < width - d.0 {
            n2 = check_if_node_valid((b.0 + d.0, b.1), nodemap);
        }
    }
    // if a more left than b
    else if a.0 < b.0 {
        if a.1 >= d.1 && a.0 >= d.0 {
            n1 = check_if_node_valid((a.0 - d.0, a.1 - d.1), nodemap);
        }
        if b.1 < height - d.1 && b.0 < width - d.0 {
            n2 = check_if_node_valid((b.0 + d.0, b.1 + d.1), nodemap);
        }
    }
    // if a more right than b
    else if a.0 > b.0 {
        if a.1 >= d.1 && a.0 < width - d.0 {
            n1 = check_if_node_valid((a.0 + d.0, a.1 - d.1), nodemap);
        }
        if b.1 < height - d.1 && b.0 >= d.0 {
            n2 = check_if_node_valid((b.0 - d.0, b.1 + d.1), nodemap);
        }
    }

    (n1, n2)
}

fn main() {
    let map = get_map(INPUT);
    let width = map[0].len();
    let height = map.len();
    let mut nodemap: Vec<(usize, usize)> = Vec::new();

    let mut unique_nodes = 0;

    for y in 0..height {
        for x in 0..width {
            if map[y][x] != '.' && map[y][x] != '#' {
                let v = find_matches(map[y][x], (x, y), &map, width, height); {
                    if v.is_empty() { 
                        continue; 
                    }
                    else {
                        for pair in v {
                            let ans = mark_antinodes((x, y), pair, width, height, &mut nodemap);
                            if ans.0 { unique_nodes += 1; }
                            if ans.1 { unique_nodes += 1; }
                        }
                    }
                }
            }
        }
    }
    println!("Unique nodes: {}", unique_nodes);
}