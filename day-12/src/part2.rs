// changing this function to use wrapping sub to avoid having to constantly check for bounds, since the map doesnt go that far anyways
fn calculate_sides(coords: &Vec<(usize, usize)>) -> u64 {
    let mut corners = 0;

    for (x, y) in coords {
        // check all 4 directions around the tile, and if there is a corner, we increase our side count by 1
        // for each side we check the direction, and the one to the right of it to get the corner count in cases like these:
        //OX     X
        //OO and OX
        //where O is a tile in area and X is not

        let up = (*x, y.wrapping_sub(1));
        let right = (x + 1, *y);
        let down = (*x, y + 1);
        let left = (x.wrapping_sub(1), *y);

        // left and up
        if coords.contains(&left) && coords.contains(&up) && !coords.contains(&(left.0, up.1)) {
            corners += 1;
        }   
        if !coords.contains(&left) && !coords.contains(&up) {
            corners += 1;
        }
        // up and right
        if coords.contains(&up) && coords.contains(&right) && !coords.contains(&(right.0, up.1)) {
            corners += 1;
        }   
        if !coords.contains(&up) && !coords.contains(&right) {
            corners += 1;
        }
        // right and down
        if coords.contains(&right) && coords.contains(&down) && !coords.contains(&(right.0, down.1)) {
            corners += 1;
        }   
        if !coords.contains(&right) && !coords.contains(&down) {
            corners += 1;
        }
        // down and left
        if coords.contains(&down) && coords.contains(&left) && !coords.contains(&(left.0, down.1)) {
            corners += 1;
        }   
        if !coords.contains(&down) && !coords.contains(&left) {
            corners += 1;
        }
    }

    corners
}

fn get_price(map: &Vec<Vec<char>>, c: char, (x, y): (usize, usize), seen: &mut Vec<(usize, usize)>, area_map: &mut Vec<(usize, usize)>) -> u64 {
    
    // just return something, we don't really care
    if x > map[0].len() - 1 || y > map.len() - 1 {
        return 0;
    }
    // same here
    if map[y][x] != c || seen.contains(&(x, y)) {
        return 0;
    }
    // otherwise we push coordinates to seen for the main loop to not include them in the future
    seen.push((x, y));

    // all of this is just for building the temporary area map containing of only same kind of plants
    area_map.push((x, y));
    get_price(map, c, (x + 1, y), seen, area_map);
    get_price(map, c, (x, y + 1), seen, area_map);
    if x != 0 {
        get_price(map, c, (x - 1, y), seen, area_map);
    }
    if y != 0 {
        get_price(map, c, (x, y - 1), seen, area_map);
    }

    // now we can start calulating the area and sides
    // area is the same as the number of tiles in our area map
    let area = area_map.len() as u64;
    let sides = calculate_sides(area_map);

    area * sides
}

pub fn run(input: &Vec<Vec<char>>) {
    let mut seen = Vec::new();
    let mut total_price = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if !seen.contains(&(x, y)) {
                // using a temporary vector to get all coordinates of tiles of the same region to calculate area and perim
                // also marking them to seen for iteration
                total_price += get_price(input, input[y][x], (x, y), &mut seen, &mut Vec::new());
            }
        }
    }

    println!("Part 2: {}", total_price);
}