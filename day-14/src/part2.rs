use std::io::Write;

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

fn move_robot(p: &mut (i64, i64), v: (i64, i64)) {
    p.0 += v.0;
    p.1 += v.1;
    // if we are too much left
    if p.0 < 0 {
        p.0 += WIDTH as i64;
    }
    // if too much right
    if p.0 >= WIDTH as i64 {
        p.0 -= WIDTH as i64;
    }
    // up
    if p.1 < 0 {
        p.1 += HEIGHT as i64;
    }
    // down
    if p.1 >= HEIGHT as i64 {
        p.1 -= HEIGHT as i64;
    }
}

fn print_map(visual_map: &[[i64; WIDTH]; HEIGHT], i: usize) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", if visual_map[y][x] == 0 { ' ' } else { 'X' });
        }
        println!();
    }
    println!("This was iteration {i}\n");
}

pub fn run(input: &Vec<[(i64, i64); 2]>) {
    let mut map = input.clone();
    let mut visual_map = [[0; WIDTH]; HEIGHT];
    for robot in map.iter() {
        visual_map[robot[0].1 as usize][robot[0].0 as usize] += 1;
    }
    for i in 1..10000 {
        for robot in &mut map {
            // remove 1 robot from the tile we leave
            visual_map[robot[0].1 as usize][robot[0].0 as usize] -= 1;
            let v = robot[1];
            move_robot(&mut robot[0], v);
            // add 1 to the new tile we move to
            visual_map[robot[0].1 as usize][robot[0].0 as usize] += 1;
        }
        // check if there exists a row where there are more than 31, in which case it finds only the map where there is a christmas tree
        // I found 31 just by trial and error
        if visual_map.iter().find(|&x| x.iter().filter(|&y| *y != 0).count() > 31).is_some() {
            print_map(&visual_map, i);
        }
    }
}