fn is_wall(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    map[y][x] == '#'
}
fn is_box(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    map[y][x] == '[' || map[y][x] == ']'
}

fn push_boxes(cmd: char, map: &mut Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    let left_side = map[y][x] == '[';
    match cmd {
        '^' => {
            for i in (0..y).rev() {
                if left_side {
                    if is_wall(map, (x, i)) && is_wall(map, (x + 1, i)) {
                        return false;
                    }
                    if map[i][x] == '.' && map[i][x + 1] == '.' {
                        map[i][x] = '[';
                        map[i][x + 1] = ']';
                        map[y][x] = '@';
                        map[y][x + 1] = '.';
                        return true;
                    }
                    if map[i][x] == ']' {
                        push_boxes(cmd, map, (x, i));
                    }

                }
                else {
                    if is_wall(map, (x, i)) && is_wall(map, (x - 1, i)) {
                        return false;
                    }
                    if map[i][x] == '.' && map[i][x - 1] == '.' {
                        map[i][x] = ']';
                        map[i][x - 1] = '[';
                        map[y][x] = '@';
                        map[y][x - 1] = '.';
                        return true;
                    }
                    if map[i][x] == '[' {
                        if push_boxes(cmd, map, (x, i)) {
                            map[i][x] = ']';
                            map[i][x - 1] = '[';
                        }
                    }
                    if map[i][x - 1] == ']' {
                        if push_boxes(cmd, map, (x - 1, i)) {
                            map[i][x - 1] = ']';
                            map[i][x - 2] = '[';
                        }
                    }
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
                    for j in x..i + 1 {
                        if map[y][j] == ']' {
                            map[y][j] = '[';
                        }
                        else { map[y][j] = ']'; }
                    }
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
                    for j in i..x {
                        if map[y][j] == '[' {
                            map[y][j] = ']';
                        }
                        else { map[y][j] = '['; }
                    }
                    map[y][x] = '@';
                    return true;
                }
            }
            false
        },
        'v' => {
            for i in y + 1..map.len() {
                if left_side {
                    if is_wall(map, (x, i)) && is_wall(map, (x + 1, i)) {
                        return false;
                    }
                    if map[i][x] == '.' && map[i][x + 1] == '.' {
                        map[i][x] = '[';
                        map[i][x + 1] = ']';
                        map[y][x] = '@';
                        map[y][x + 1] = '.';
                        return true;
                    }
                    if map[i][x] == ']' {
                        push_boxes(cmd, map, (x, i));
                    }
                }
                else {
                    if is_wall(map, (x, i)) && is_wall(map, (x - 1, i)) {
                        return false;
                    }
                    if map[i][x] == '.' && map[i][x - 1] == '.' {
                        map[i][x] = ']';
                        map[i][x - 1] = '[';
                        map[y][x] = '@';
                        map[y][x - 1] = '.';
                        return true;
                    }
                    if map[i][x] == '[' {
                        push_boxes(cmd, map, (x, i));
                    }
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

fn expanded_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new = Vec::new();
    for y in 0..map.len() {
        let mut temp = Vec::new();
        for x in 0..map[0].len() {
            match map[y][x] {
                '#' => { temp.push('#'); temp.push('#'); },
                'O' => { temp.push('['); temp.push(']'); },
                '.' => { temp.push('.'); temp.push('.'); },
                '@' => { temp.push('@'); temp.push('.'); },
                _ => panic!("Failed to expand map"),
            }
        }
        new.push(temp);
    }
    new
}

pub fn run(input: &(Vec<Vec<char>>, String)) {
    let mut map = expanded_map(&input.0);
    println!("{:?}", map);
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
        println!("Move: {cmd}");
        move_robot(cmd, &mut map, &mut pos);
        println!("{:?}\n", map);
    }

    let mut sum = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                sum += 100 * y + x;
            }
        }
    }
    println!("Part 2: {}", sum);
}