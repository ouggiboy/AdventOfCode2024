fn is_wall(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    map[y][x] == '#'
}
fn is_box(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    map[y][x] == 'O'
}

fn push_boxes(cmd: char, map: &mut Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    match cmd {
        '^' => {
            for i in (0..y).rev() {
                if is_wall(map, (x, i)) {
                    return false;
                }
                if map[i][x] == '.' {
                    map[i][x] = 'O';
                    map[y][x] = '@';
                    return true;
                }
            }
            false
        },
        '>' => {
            for i in x + 1..map[0].len() {
                if is_wall(map, (i, y)) {
                    return false;
                }
                if map[y][i] == '.' {
                    map[y][i] = 'O';
                    map[y][x] = '@';
                    return true;
                }
            }
            false
        },
        '<' => {
            for i in (0..x).rev() {
                if is_wall(map, (i, y)) {
                    return false;
                }
                if map[y][i] == '.' {
                    map[y][i] = 'O';
                    map[y][x] = '@';
                    return true;
                }
            }
            false
        },
        'v' => {
            for i in y + 1..map.len() {
                if is_wall(map, (x, i)) {
                    return false;
                }
                if map[i][x] == '.' {
                    map[i][x] = 'O';
                    map[y][x] = '@';
                    return true;
                }
            }
            false
        },
        _ => {
            panic!("Something weird is going on, current command is {cmd}");
        },
    }
}

fn move_robot(cmd: char, map: &mut Vec<Vec<char>>, (x, y): &mut (usize, usize)) {
    map[*y][*x] = '.';
    match cmd {
        '^' => {
            if !is_wall(&map, (*x, *y - 1)) {
                if is_box(&map, (*x, *y - 1)) {
                    if push_boxes(cmd, map, (*x, *y - 1)) {
                        *y -= 1;
                    }
                }
                else { *y -= 1; }
            } 
        },
        '>' => {
            if !is_wall(&map, (*x + 1, *y)) {
                if is_box(&map, (*x + 1, *y)) {
                    if push_boxes(cmd, map, (*x + 1, *y)) {
                        *x += 1;
                    }
                }
                else { *x += 1; }
            } 
        },
        '<' => {
            if !is_wall(&map, (*x - 1, *y)) {
                if is_box(&map, (*x - 1, *y)) {
                    if push_boxes(cmd, map, (*x - 1, *y)) {
                        *x -= 1;
                    }
                }
                else { *x -= 1; }
            } 
        },
        'v' => {
            if !is_wall(&map, (*x, *y + 1)) {
                if is_box(&map, (*x, *y + 1)) {
                    if push_boxes(cmd, map, (*x, *y + 1)) {
                        *y += 1;
                    }
                }
                else { *y += 1; }
            } 
        },
        _ => panic!("Something weird is going on, current command is {cmd}"),
    }
    map[*y][*x] = '@';
}

pub fn run(input: &(Vec<Vec<char>>, String)) {
    let mut map = input.0.clone();
    let cmds = &input.1;
    let mut pos = (0, 0);
    'start_finder: for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                pos = (x, y);
                break 'start_finder;
            }
        }
    };

    for cmd in cmds.chars() {
        move_robot(cmd, &mut map, &mut pos);
    }

    let mut sum = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                sum += 100 * y + x;
            }
        }
    }
    println!("Part 1: {}", sum);
}