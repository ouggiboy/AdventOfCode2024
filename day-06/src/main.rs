use std::fs;

const TESTING: bool = false;
const INPUT: &str = if TESTING {
    "sample.txt"
} else {
    "input.txt"
};

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

fn mark(x: usize, y: usize, map: &mut Vec<Vec<char>>) -> bool {
    if map[y][x] == '.' || map[y][x] == '^' {
        map[y][x] = 'x';
        return true
    }
    false
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
    let mut map = get_map(INPUT);
    let mut direction = Direction::Up;
    let (mut x, mut y) = get_start_pos(&map);
    let mut pos_count = 1;
    mark(x, y, &mut map);


    while step(&mut direction, &mut x, &mut y, &map) { 
        if mark(x, y, &mut map) {
            pos_count += 1;
        }
    }
    println!("Distinct positions: {}", pos_count);
}
