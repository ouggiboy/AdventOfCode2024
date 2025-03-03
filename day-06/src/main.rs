use std::{fs, collections::HashMap};

const TESTING: bool = true;
const INPUT: &str = if TESTING {
    "sample.txt"
} else {
    "input.txt"
};

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn get_map(path: &str) -> Vec<Vec<char>> {
    let str = fs::read_to_string(path).unwrap();
    let mut lines = Vec::<Vec<char>>::new();

    for line in str.split("\r\n") {
        lines.push(
            line.chars().collect::<Vec<char>>()
        )
    }

    lines
}

fn collision(map: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if map[y][x] == '#' {
        return true;
    }
    false
}

fn step(direction: &mut Direction, x: &mut usize, y: &mut usize, map: &Vec<Vec<char>>) -> bool {
    match direction {
        Direction::Up => {
            // if going oob
            if *y == 0 { return false; }

            if collision(&map, *x, *y - 1) {
                *direction = Direction::Right;
                *x += 1;
            } 
            else { *y -= 1; }
        }
        Direction::Right => {
            // if going oob
            if *x == map.len() - 1 { return false; }

            if collision(&map, *x + 1, *y) {
                *direction = Direction::Down;
                *y += 1;
            } 
            else { *x += 1; }
        }
        Direction::Down => {
            // if going oob
            if *y == map.len() - 1 { return false; }

            if collision(&map, *x, *y + 1) {
                *direction = Direction::Left;
                *x -= 1;
            } 
            else { *y += 1; }
        }
        Direction::Left => {
            // if going oob
            if *x == 0 { return false; }

            if collision(&map, *x - 1, *y) {
                *direction = Direction::Up;
                *y -= 1;
            } 
            else { *x -= 1; }
        }
    }
    true
}

fn store_direction(x: usize, y: usize, direction: &Direction, pos_map: &mut HashMap<(usize, usize), Vec<Direction>>) {
    let key = (x, y);
    if let Some(vector) = pos_map.get_mut(&key) {
        vector.push(direction.clone());
    }    
    else {
        pos_map.insert(key, vec![direction.clone()]);
    }
}

fn get_start_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '^' {
                return (x, y)
            }
        }
    }
    (0, 0)
}

fn main() {
    let map = get_map(INPUT);
    let mut pos_map: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();
    let mut direction = Direction::Up;
    let (mut x, mut y) = get_start_pos(&map);

    while step(&mut direction, &mut x, &mut y, &map) { 
        store_direction(x, y, &direction, &mut pos_map);
    }
    println!("Distinct positions: {}", pos_map.len());
}
