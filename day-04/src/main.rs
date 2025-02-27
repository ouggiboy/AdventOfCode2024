use std::fs;

const TESTING: bool = false;
const INPUT: &str = if TESTING { 
    "sample.txt"
} else {
    "input.txt"
};

fn get_input_rows(path: &str) -> Vec<Vec<char>> {
    let data = fs::read_to_string(path).unwrap();
    let rows = data.split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|row| row.to_string())
        .collect::<Vec<String>>();

    let mut rows_arr: Vec<Vec<char>> = Vec::new();

    for row in rows {
        let mut chars: Vec<char> = Vec::new();
        for char in row.chars() {
            chars.push(char);
        }
        rows_arr.push(chars);
    }

    rows_arr
}

fn arr_equal_to_xmas_p1(arr: &[char; 4]) -> bool {
    *arr == ['X', 'M', 'A', 'S']
}


fn find_xmas_p1(rows: &Vec<Vec<char>>) -> u64 {
    let mut xmas_count = 0;

    let h = rows.len();
    let w = rows[0].len();
    
    for i in 0..h {
        for j in 0..w {
            if rows[i][j] == 'X' {
                // checking directions if in bounds
                let up = i >= 3;
                let down = i < h - 3;
                let left = j >= 3;
                let right = j < w - 3;
                // diagonal
                let ul = up && left;
                let ur = up && right;
                let dl = down && left;
                let dr = down && right;

                let mut check_arrays: Vec<[char; 4]> = Vec::new();
                
                if up { check_arrays.push([rows[i][j], rows[i-1][j], rows[i-2][j], rows[i-3][j]]); }
                if down { check_arrays.push([rows[i][j], rows[i+1][j], rows[i+2][j], rows[i+3][j]]); }
                if left { check_arrays.push([rows[i][j], rows[i][j-1], rows[i][j-2], rows[i][j-3]]); }
                if right { check_arrays.push([rows[i][j], rows[i][j+1], rows[i][j+2], rows[i][j+3]]); }
                if ul { check_arrays.push([rows[i][j], rows[i-1][j-1], rows[i-2][j-2], rows[i-3][j-3]]); }
                if ur { check_arrays.push([rows[i][j], rows[i-1][j+1], rows[i-2][j+2], rows[i-3][j+3]]); }
                if dl { check_arrays.push([rows[i][j], rows[i+1][j-1], rows[i+2][j-2], rows[i+3][j-3]]); }
                if dr { check_arrays.push([rows[i][j], rows[i+1][j+1], rows[i+2][j+2], rows[i+3][j+3]]); }

                for arr in check_arrays {
                    if arr_equal_to_xmas_p1(&arr) {
                        xmas_count += 1;
                    }
                }
            }
        }
    }
    xmas_count
}

fn arr_equal_to_xmas_p2(arr: &[char; 5]) -> bool {
    *arr == ['M', 'S', 'A', 'M', 'S'] ||
    *arr == ['S', 'S', 'A', 'M', 'M'] ||
    *arr == ['M', 'M', 'A', 'S', 'S'] ||
    *arr == ['S', 'M', 'A', 'S', 'M']
}

fn find_xmas_p2(rows: &Vec<Vec<char>>) -> u64 {
    let mut xmas_count = 0;

    let h = rows.len();
    let w = rows[0].len();
    
    for i in 0..h {
        for j in 0..w {
            if rows[i][j] == 'A' {
                // check bounds ('A' not in edges of arr)
                if i > 0 && j > 0 && i < h - 1 && j < w - 1 {
                    // making array [tl, tr, m, bl, br]
                    let arr = [rows[i-1][j-1], rows[i-1][j+1], rows[i][j], rows[i+1][j-1], rows[i+1][j+1]];
                    if arr_equal_to_xmas_p2(&arr) {
                        xmas_count += 1;
                    }
               }
            }
        }
    }
    xmas_count
}

fn main() {
    let rows = get_input_rows(INPUT);
    println!("Part 1 result: {}", find_xmas_p1(&rows));
    println!("Part 2 result: {}", find_xmas_p2(&rows));
}