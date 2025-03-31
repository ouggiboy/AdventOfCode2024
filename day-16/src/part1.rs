use std::collections::HashMap;

fn can_move(x: usize, y: usize, map: &Vec<Vec<char>>, visited: &mut HashMap<(usize, usize), (usize, usize)>, steps: usize, turns: usize) -> bool {
    if map[y][x] == '#' {
        return false;
    }
    // if we have already visited this tile with fewer score than our current score is, return false as we dont want to continue any further down this path
    // otherwise set the new steps and turns to replace the old ones, that were less efficient
    else if let Some((old_steps, old_turns)) = visited.get(&(x, y)) {
        if old_turns * 1000 + old_steps > turns * 1000 + steps {
            visited.insert((x, y), (steps, turns));
            return true;
        }
        return false;
    }
    else {
        visited.insert((x, y), (steps, turns));
        return true;
    }
}

fn find_paths(x: &mut usize, y: &mut usize, map: &Vec<Vec<char>>, dir: &mut (i32, i32), visited: &mut HashMap<(usize, usize), (usize, usize)>,
              end_x: usize, end_y: usize, steps: usize, turns: usize, low: &mut usize) 
    {
    // if we are above our lowest scoring path stop calculating any further
    if turns * 1000 + steps > *low {
        return;
    }
    // if at the end set out lowest to our score, as we already excluded scores higher than our lowest
    if (*x, *y) == (end_x, end_y) {
        visited.remove(&(*x, *y));
        *low = turns * 1000 + steps;
        return;
    }

    for (v_x, v_y) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        if (dir.0 * -1, dir.1 * -1) == (v_x, v_y) {
            continue;
        }
        let mut x = (*x as i32 + v_x) as usize;
        let mut y = (*y as i32 + v_y) as usize;
        let mut temp_dir = dir.clone();
        let mut turned= false;
        if temp_dir != (v_x, v_y) {
            temp_dir = (v_x, v_y);
            turned = true;
        }
        if can_move(x, y, map, visited, steps, turns) {
            find_paths(&mut x, &mut y, map, &mut temp_dir, visited, end_x, end_y, steps+1, if turned { turns+1 } else { turns }, low);
        }
    }
}

pub fn run(input: &Vec<Vec<char>>) {
    let map = input;
    let (mut x, mut y) = (1usize, map.len() - 2);
    let (end_x, end_y) = (map[0].len() - 2, 1);
    let mut dir = (1, 0);
    let mut visited: HashMap<(usize, usize), (usize, usize)> = HashMap::with_capacity((map.len() - 1) * (map[0].len() - 1));

    let mut low = usize::MAX;
    find_paths(&mut x, &mut y, map, &mut dir, &mut visited, end_x, end_y, 0, 0, &mut low);

    println!("{}", low);
}