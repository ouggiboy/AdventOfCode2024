use std::collections::HashSet;

use crate::MAX_COORDS;

fn find_path(grid: &[[bool; MAX_COORDS + 1]; MAX_COORDS + 1]) -> bool {
    let mut queue = Vec::from([(0i32, 0i32)]);

    let mut seen = HashSet::from([(0, 0)]);

    while !queue.is_empty() {
        let (x, y) = queue.remove(0);
        for (nx, ny) in [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)] {
            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if nx > MAX_COORDS || ny > MAX_COORDS {
                continue;
            }
            if grid[ny][nx] == false {
                continue;
            }
            if seen.contains(&(nx, ny)) {
                continue;
            }
            if (nx, ny) == (MAX_COORDS, MAX_COORDS) {
                return true;
            }
            seen.insert((nx, ny));
            queue.push((nx as i32, ny as i32));
        }
    }
    false
}

pub fn run(input: &Vec<(usize, usize)>) {
    let mut grid = [[true; MAX_COORDS + 1]; MAX_COORDS + 1];
    let mut i = 0;

    while find_path(&grid) {
        grid[input[i].1][input[i].0] = false;
        i += 1;
    }
    
    println!("Part 2: {},{}", input[i-1].0, input[i-1].1);
}