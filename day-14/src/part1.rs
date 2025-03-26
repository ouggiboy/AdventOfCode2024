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

enum Quad {
    TL,
    TR,
    BL,
    BR,
}

fn get_quad_score(quad: Quad, score_map: &[[i32; WIDTH]; HEIGHT]) -> i32 {
    let wm = WIDTH / 2;
    let hm = HEIGHT / 2;
    let mut sum = 0;
    match quad {
        Quad::TL => {
            for y in 0..hm {
                for x in 0..wm {
                    sum += score_map[y][x];
                }
            }
        },
        Quad::TR => {
            for y in 0..hm {
                for x in wm + 1..WIDTH {
                    sum += score_map[y][x];
                }
            }
        },
        Quad::BL => {
            for y in hm + 1..HEIGHT {
                for x in 0..wm {
                    sum += score_map[y][x];
                }
            }
        },
        Quad::BR => {
            for y in hm + 1..HEIGHT {
                for x in wm + 1..WIDTH {
                    sum += score_map[y][x];
                }
            }
        },
    }
    sum
}

pub fn run(input: &Vec<[(i64, i64); 2]>) {
    let mut map = input.clone();
    for _ in 0..100 {
        for robot in &mut map {
            let v = robot[1];
            move_robot(&mut robot[0], v);
        }
    }
    // now mark the robot positions after 100 seconds into our score_map map
    let mut score_map = [[0; WIDTH]; HEIGHT];
    for robot in map {
        score_map[robot[0].1 as usize][robot[0].0 as usize] += 1;
    }
    // now calculate score for each quadrant and multiply them together
    let safety_factor = get_quad_score(Quad::TL, &score_map) 
                           * get_quad_score(Quad::TR, &score_map)
                           * get_quad_score(Quad::BL, &score_map)
                           * get_quad_score(Quad::BR, &score_map);
    println!("{}", safety_factor);
}