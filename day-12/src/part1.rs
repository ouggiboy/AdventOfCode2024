// basically we want to follow one diection as long as there is tiles there, after which we go down etc.
fn calculate_perimeter(coords: &Vec<(usize, usize)>, map: &Vec<Vec<char>>) -> u64 {
    let mut perimeter = 0;

    for (x, y) in coords {
        // check all 4 directions around the tile, adding 1 to perimeter each time the neighbour is not in this region
        // first check if we are at the border, which instantly gives +1 to perimeter
        // left
        if *x != 0 {
            if !coords.contains(&(x - 1, *y)) {
                perimeter += 1;
            }
        }
        else {
            perimeter += 1;
        }
        // right
        if *x != map[0].len() - 1 {
            if !coords.contains(&(x + 1, *y)) {
                perimeter += 1;
            }
        }
        else {
            perimeter += 1;
        }
        // up
        if *y != 0 {
            if !coords.contains(&(*x, y - 1)) {
                perimeter += 1;
            }
        }
        else {
            perimeter += 1;
        }
        // down
        if *y != map.len() - 1 {
            if !coords.contains(&(*x, y + 1)) {
                perimeter += 1;
            }
        }
        else {
            perimeter += 1;
        }
    }

    perimeter
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

    // now we can start calulating the area and perimeter
    // area is the same as the number of tiles in our area map
    let area = area_map.len() as u64;
    let perimeter = calculate_perimeter(area_map, map);

    area * perimeter
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

    println!("{}", total_price);
}