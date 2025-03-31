use std::collections::{HashMap, HashSet};

fn can_move(x: usize, y: usize, map: &Vec<Vec<char>>, visited: &mut HashMap<(usize, usize), (usize, usize)>, steps: usize, turns: usize, this_path: &mut HashSet<(usize, usize)>) -> bool {
    if map[y][x] == '#' {
        return false;
    }
    // if we have already visited this tile with fewer score than our current score is, return false as we dont want to continue any further down this path
    // otherwise set the new steps and turns to replace the old ones, that were less efficient
    else if let Some((old_steps, old_turns)) = visited.get(&(x, y)) {
        if turns * 1000 + steps < old_turns * 1000 + old_steps {
            this_path.insert((x, y));
            visited.insert((x, y), (steps, turns));
            return true;
        }
        return false;
    }
    else {
        this_path.insert((x, y));
        visited.insert((x, y), (steps, turns));
        return true;
    }
}

fn find_paths(x: &mut usize, y: &mut usize, map: &mut Vec<Vec<char>>, dir: &mut (i32, i32), visited: &mut HashMap<(usize, usize), (usize, usize)>,
              end_x: usize, end_y: usize, steps: usize, turns: usize, low: &mut usize, this_path: &mut HashSet<(usize, usize)>, tiles: &mut HashSet<(usize, usize)>)
    {
    // if we are above our lowest scoring path stop calculating any further
    if turns * 1000 + steps > *low {
        return;
    }
    // if at the end set out lowest to our score, as we already excluded scores higher than our lowest
    if (*x, *y) == (end_x, end_y) {
        visited.remove(&(*x, *y));
        // if we find another path leading to same score, add that path's tiles to tile set as well
        if turns * 1000 + steps == *low {
            for (x, y) in this_path.iter() {
                tiles.insert((*x, *y));
            }
        }
        // otherwise we empty the set and add current tiles to it
        else {
            tiles.clear();
            for (x, y) in this_path.iter() {
                tiles.insert((*x, *y));
            }
            *low = turns * 1000 + steps;
        }
        return;
    }

    for (v_x, v_y) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        // ignore opposite direction
        if (dir.0 * -1, dir.1 * -1) == (v_x, v_y) {
            continue;
        }
        let mut x = (*x as i32 + v_x) as usize;
        let mut y = (*y as i32 + v_y) as usize;
        let mut temp_dir = dir.clone();
        let mut turning= false;
        if temp_dir != (v_x, v_y) {
            temp_dir = (v_x, v_y);
            turning = true;
            // if we turn add 1 turn to the previous tile, that was at the turning point for other paths to compare
            visited.insert(((x as i32 - v_x) as usize, (y as i32 - v_y) as usize), (steps, turns + 1));
        }
        let mut this_path = this_path.clone();
        let turns = if turning { turns + 1 } else { turns };
        if can_move(x, y, map, visited, steps + 1, turns, &mut this_path) {
            find_paths(&mut x, &mut y, map, &mut temp_dir, visited, end_x, end_y, steps+1, turns , low, &mut this_path, tiles);
        }
    }
}

pub fn run(input: &Vec<Vec<char>>) {
    let mut map = input.clone();
    let (mut x, mut y) = (1usize, map.len() - 2);
    let (end_x, end_y) = (map[0].len() - 2, 1);
    let mut dir = (1, 0);
    let mut visited: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut low = usize::MAX;
    let mut tiles = HashSet::new();
    
    find_paths(&mut x, &mut y, &mut map, &mut dir, &mut visited, end_x, end_y, 0, 0, &mut low, &mut HashSet::<(usize, usize)>::new(), &mut tiles);

    println!("Part 2: {}", tiles.len() + 1);

}