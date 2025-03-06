use std::{collections::HashMap, fs};

#[cfg(debug_assertions)]
const INPUT: &str = "sample.txt";

#[cfg(not(debug_assertions))]
const INPUT: &str = "input.txt";

#[derive(Clone, PartialEq, Debug)]
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
    if map[y][x] == '#' || map[y][x] == 'O' {
        return true;
    }
    false
}

// changing directions
fn turn_right(direction: &mut Direction) {
    match direction {
        Direction::Up => {
            *direction = Direction::Right;
        },
        Direction::Right => {
            *direction = Direction::Down;
        },
        Direction::Down => {
            *direction = Direction::Left;
        },
        Direction::Left => {
            *direction = Direction::Up;
        },
    }
}

fn step_or_turn(direction: &mut Direction, x: &mut usize, y: &mut usize, map: &Vec<Vec<char>>) -> bool {
    match direction {
        Direction::Up => {
            // if going oob
            if *y == 0 { return false; }

            if collision(&map, *x, *y - 1) {
                turn_right(direction);
                return true;
            } 
            else { *y -= 1; }
        },
        Direction::Right => {
            // if going oob
            if *x == map.len() - 1 { return false; }

            if collision(&map, *x + 1, *y) {
                turn_right(direction);
                return true;
            } 
            else { *x += 1; }
        },
        Direction::Down => {
            // if going oob
            if *y == map.len() - 1 { return false; }

            if collision(&map, *x, *y + 1) {
                turn_right(direction);
                return true;
            } 
            else { *y += 1; }
        },
        Direction::Left => {
            // if going oob
            if *x == 0 { return false; }

            if collision(&map, *x - 1, *y) {
                turn_right(direction);
                return true;
            } 
            else { *x -= 1; }
        },
    }
    true
}

fn store_direction(x: usize, y: usize, direction: &Direction, pos_map: &mut HashMap<(usize, usize), Vec<Direction>>) {
    let key = (x, y);
    if let Some(vector) = pos_map.get_mut(&key) {
        if !vector.contains(direction) {
            vector.push(direction.clone());
        }
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

fn mark(x: usize, y: usize, map: &mut Vec<Vec<char>>) -> bool {
    if map[y][x] == '.' || map[y][x] == '^' {
        map[y][x] = 'X';
        return true
    }
    false
}

// checking if placing obstruction to next spot causes time loop and
// storing all the values into position map to reduce computing the same positions over and over again
fn obstacle_causes_loop(direction: &Direction, x: usize, y: usize, map: &mut Vec<Vec<char>>, pos_map: &mut HashMap<(usize, usize), Vec<Direction>>) -> bool {

    // getting next tile coords and checking possible out of bounds for the last move to not crash (same as in step_or_turn() function)
    let (obs_x, obs_y) = match direction {
        Direction::Up => {
            if y == 0 { return false; } 
            (x, y - 1) 
        },
        Direction::Right => {
            if x == map.len() - 1 { return false; } 
            (x + 1, y) 
        },
        Direction::Down => {
            if y == map.len() - 1 { return false; }
            (x, y + 1)
        },
        Direction::Left => {
            if x == 0 { return false; }
            (x - 1, y)
        },
    };
    // first check if next tile is already been to by guard, or it is an obstacle
    if map[obs_y][obs_x] == 'X' || map[obs_y][obs_x] == '#' {
        return false;
    }
    
    // else continue  
    map[obs_y][obs_x] = 'O';

    
    // copying the variables to only be used by this function to check the looping
    let mut temp_x = x;
    let mut temp_y = y;
    let mut temp_direction = direction.clone();
    let mut temp_pos_map = pos_map.clone();
    
    // we do this as long as we encounter a already seen position this loop, or exit out of the map
    while step_or_turn(&mut temp_direction, &mut temp_x, &mut temp_y, map) {
        // first checking if position already exists in map, in which case we know it is in a loop 
        // since it has already been to the exact position with the same direction, so we return true
        if let Some(v) = temp_pos_map.get_mut(&(temp_x, temp_y)) {
            if v.contains(&temp_direction) {
                // now we remove obstacle to not interfere with future runs
                map[obs_y][obs_x] = '.';
                return true;
            }
        }
        store_direction(temp_x, temp_y, &temp_direction, &mut temp_pos_map);
    }
    // changin map back to normal
    map[obs_y][obs_x] = '.';
    false
}

fn main() {
    let mut map = get_map(INPUT);
    let mut pos_map: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();
    let mut direction = Direction::Up;
    let (mut x, mut y) = get_start_pos(&map);
    let mut unique = 1;
    let mut obstruction_count = 0;
    
    // storing and marking the start point
    store_direction(x, y, &direction, &mut pos_map);
    mark(x, y, &mut map);   
    
    while step_or_turn(&mut direction, &mut x, &mut y, &map) {
        
        // part 1
        // marking the tiles guard has been to
        if mark(x, y, &mut map) {
            unique += 1;
        }
        
        // part 2
        // checking if placing obstacle to next tile would cause a loop
        if obstacle_causes_loop(&direction, x, y, &mut map, &mut pos_map) {
            obstruction_count += 1;
        }      

        // storing the guard position and direction
        store_direction(x, y, &direction, &mut pos_map);
    }

    println!("Distinct positions: {}", unique);
    println!("Different obstructions positions causing a loop: {}", obstruction_count);
}
