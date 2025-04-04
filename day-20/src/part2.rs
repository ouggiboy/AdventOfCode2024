use std::collections::{HashMap, HashSet};

fn find_coord(c: char, map: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == c {
                return Some((x as i32, y as i32));
            }
        }
    }
    None
}

fn store_path_times(times: &mut HashMap<(i32, i32), i32>, map: &Vec<Vec<char>>, start: (i32, i32), end: (i32, i32), time: i32) {
    if start == end {
        return;
    }
    for (vx, vy) in [(1,0), (0,1), (-1,0), (0,-1)] {
        let x = start.0 + vx;
        let y = start.1 + vy;
        if x < 0 || y < 0 {
            continue;
        }
        if map[y as usize][x as usize] != '#' && !times.contains_key(&(x, y)) {
            times.insert((x, y), time);
            return store_path_times(times, map, (x, y), end, time + 1);
        }
    }
}

fn find_cheats(times: &HashMap<(i32, i32), i32>) -> Vec<i32> {
    let mut time_saves = Vec::new();
    let mut compared: HashSet<(i32, i32)> = HashSet::new();
    for (pos, time) in times {
        compared.insert(*pos);
        for (npos, ntime) in times {
            // if the pair has already been compared
            if compared.contains(&npos) {
                continue;
            }
            // distance between points can be at most 20
            let dx = (pos.0-npos.0).abs();
            let dy = (pos.1-npos.1).abs();
            let dist = dx + dy;
            if dist > 20 {
                continue;
            }
            let time_save = (ntime - time).abs() - dist;
            if time_save >= 100 {
                time_saves.push(time_save);
            }
        }
    }
    time_saves
}

pub fn run(map: &Vec<Vec<char>>) {
    let start = find_coord('S', map).unwrap();
    let end = find_coord('E', map).unwrap();

    let mut times = HashMap::new();
    times.insert(start, 0);
    store_path_times(&mut times, map, start, end, 1);

    let cheat_times = find_cheats(&times);
    println!("Part 2: {}", cheat_times.len());
}