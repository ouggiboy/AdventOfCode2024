use std::fs;

fn get_map(input: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(input).unwrap()
        .split('\n')
        .map(|line| 
            line.trim()
                .chars()
                // parse char to u8
                .map(|c| c as u8 - 48)
                .collect()
        )
        .collect()
}

fn get_score(map: &Vec<Vec<u8>>, x: i32, y: i32, n: u8, peaks: &mut Vec<(i32, i32)>) -> u16 {

    // check if we are out of bounds
    if x < 0 || y < 0 || x as usize >= map[0].len() || y as usize >= map.len() {
        return 0;
    }

    // check if current tile is what it should be
    if map[y as usize][x as usize] == n {
        // check if we are at the end of trail and we haven't been to the end by some other route
        if map[y as usize][x as usize] == 9 && !peaks.contains(&(x, y)) {
            peaks.push((x, y));
            return 1;
        }
        // otherwise go to each direction from that point
        else {
            return get_score(map, x - 1, y, n + 1, peaks)
            + get_score(map, x + 1, y, n + 1, peaks)
            + get_score(map, x, y - 1, n + 1, peaks)
            + get_score(map, x, y + 1, n + 1, peaks)
        }
    }

    // otherwise we return 0 as score if it is the wrong way
    return 0;
}

fn get_score_sum(map: &Vec<Vec<u8>>) -> u16 {
    let mut score_sum = 0;
    let w = map[0].len();
    let h = map.len();

    for y in 0..h {
        for x in 0..w {
            if map[y][x] == 0 {
                // store info about the '9' tiles we have already visited in a vector of tuples
                let mut peaks= vec![(x as i32, y as i32)];
                // add each starting positions' score to result
                score_sum += get_score(map, x as i32, y as i32, 0, &mut peaks);
            }
        }
    }
    score_sum
}

pub fn run(input: &str) {
    let map = get_map(input);
    println!("{}", get_score_sum(&map));    
}