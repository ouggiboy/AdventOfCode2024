use std::collections::HashSet;

use crate::MAX_COORDS;

fn find_path(grid: &[[bool; MAX_COORDS + 1]; MAX_COORDS + 1]) -> Option<u64> {
    let mut queue = Vec::from([(0i32, 0i32, 0)]);

    let mut seen = HashSet::from([(0, 0)]);

    while !queue.is_empty() {
        let (x, y, steps) = queue.remove(0);
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
                return Some(steps + 1);
            }
            seen.insert((nx, ny));
            queue.push((nx as i32, ny as i32, steps + 1));
        }
    }
    None
}

pub fn run(input: &Vec<(usize, usize)>) {
    let start = std::time::Instant::now();

    let mut grid = [[true; MAX_COORDS + 1]; MAX_COORDS + 1];
    for i in 0..1024 {
        grid[input[i].1][input[i].0] = false;
    }
    
    if let Some(steps) = find_path(&grid) {
        let end = std::time::Instant::now();
        println!("Part 1: {}", steps);
        println!("Total time: {:?}", end - start);
    }

}