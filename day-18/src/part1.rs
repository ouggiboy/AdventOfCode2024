use std::collections::HashMap;

use crate::MAX_COORDS;

fn find_path(grid: &[[bool; MAX_COORDS + 1]; MAX_COORDS + 1], start: (usize, usize), end: (usize, usize), seen: &mut HashMap<(usize, usize), u64>, steps: u64, min_steps: &mut u64) {
    if start == end {
        if steps < *min_steps {
            *min_steps = steps;
        }
        return;
    }
    let directions = [(1, 0), (0, 1), (0, -1), (-1, 0)];
    for (vx, vy) in directions {
        let x = start.0 as i64 + vx;
        let y = start.1 as i64 + vy;
        if x < 0 || y < 0 || x as usize > MAX_COORDS || y as usize > MAX_COORDS {
            continue;
        }
        let x = x as usize;
        let y = y as usize;
        if grid[y][x] == false {
            continue;
        }
        if let Some(&old_steps) = seen.get(&(x, y)) {
            if old_steps > steps {
                seen.insert((x, y), steps);
            }
            else {
                continue;
            }
        }
        else {
            seen.insert((x, y), steps);
        }
        find_path(grid, (x, y), end, seen, steps + 1, min_steps);
    }
}

pub fn run(input: &Vec<(usize, usize)>) {
    let mut grid = [[true; MAX_COORDS + 1]; MAX_COORDS + 1];
    for i in 0..1024 {
        grid[input[i].1][input[i].0] = false;
    }
    
    let mut seen = HashMap::new();
    let mut min_steps = u64::MAX;

    find_path(&grid, (0, 0), (MAX_COORDS, MAX_COORDS), &mut seen, 0, &mut min_steps);

    println!("Part 1: {}", min_steps);
}