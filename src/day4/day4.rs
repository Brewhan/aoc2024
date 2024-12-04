use crate::utils::utils::get_file_contents;
use regex::Regex;

// use slice windows to capture 4 items

// look ahead horizontally
// look behind horizontally

// look down vertically if  rows - rows counted > 3
// look up vertically if rows counted > 3

// look up + left
// look up + right
// look down + left
// look down + right

pub fn task1(file_path: &str) {
    let contents = get_file_contents(file_path);
    let mut total: i32 = 0;
    let re = Regex::new(r"XMAS").unwrap();
    let contents: Vec<String> = contents
        .split("\r\n")
        .map(|s| s.to_string())
        .filter(|s| !s.trim().is_empty())
        .collect();

    //THIS LOOKS FORWARDS AND BACKWARDS UP AND DOWN
    //rotate_scan(contents.clone(), re.clone(), &mut total);
    traverse_diagonally(contents.clone(), &mut total);
    println!("{}", total);
}

fn traverse_diagonally(contents: Vec<String>, total: &mut i32) {
    //Convert the Vec<String> to a matrix
    let mut matrix: Vec<Vec<char>> = contents.iter().map(|row| row.chars().collect()).collect();
    matrix =add_padding(&matrix);
    let rows = matrix.len();

    // println!("{:?}", rows);
    // println!("{:?}", matrix[0].len());

    //LOOK FOR X
    
    for (index, value) in matrix.iter().enumerate() {
        for (k, v) in value.iter().enumerate() {

            if *v == 'X' {
                // println!("{:?},{:?}", index, k);

                //Check diagonally down right -- THIS >3 IS A GUESS!!!
                if (value.len() - k >= 3) && (rows - index > 3) {
                    if matrix[index + 1][k + 1] == 'M'
                        && matrix[index + 2][k + 2] == 'A'
                        && matrix[index + 3][k + 3] == 'S'
                    {
                        // println!("{:?}{:?},{:?} dr", matrix[index][k], index, k);
                        *total += 1;
                    }
                }

                //Check diagonally down left
                if (k >= 3) && (rows - index > 3) {
                    if matrix[index + 1][k - 1] == 'M'
                        && matrix[index + 2][k - 2] == 'A'
                        && matrix[index + 3][k - 3] == 'S'
                    {
                        // println!("{:?}{:?},{:?} dl", matrix[index][k], index, k);
                        *total += 1;
                    }
                }

                // Check up and right
                if (value.len() - k >= 3) && (index > 3) {
                    if matrix[index - 1][k + 1] == 'M'
                        && matrix[index - 2][k + 2] == 'A'
                        && matrix[index - 3][k + 3] == 'S'
                    {
                        // println!("{:?}{:?},{:?} ur", matrix[index][k], index, k);
                        *total += 1;
                    }
                }

                // //Check up and left
                if (k >= 3) && (index > 3) {
                    if matrix[index - 1][k - 1] == 'M'
                        && matrix[index - 2][k - 2] == 'A'
                        && matrix[index - 3][k - 3] == 'S'
                    {
                        // println!("{:?}{:?},{:?} ul", matrix[index][k], index, k);
                        *total += 1;
                    }
                }
                // RIGHT
                if value.len() - k >= 3 {
                    if matrix[index][k + 1] == 'M'
                        && matrix[index][k + 2] == 'A'
                        && matrix[index][k + 3] == 'S'
                    {
                        println!("{:?}{:?},{:?} r", matrix[index][k], index, k);
                        *total += 1;
                    }
                }

                //LEFT
                if k >= 3 {
                    if matrix[index][k - 1] == 'M'
                        && matrix[index][k - 2] == 'A'
                        && matrix[index][k - 3] == 'S'
                    {
                        // println!("{:?}{:?},{:?} l", matrix[index][k], index, k);
                        *total += 1;
                    }
                }

                //UP
                if index >= 3 {
                    if matrix[index - 1][k] == 'M'
                        && matrix[index - 2][k] == 'A'
                        && matrix[index - 3][k] == 'S'
                    {
                        // println!("{:?}{:?},{:?} u", matrix[index][k], index, k);
                        *total += 1;
                    }
                }

                //DOWN
                if rows - index >= 3 {
                    println!("ABC{:?}{:?},{:?} r", matrix[index][k], index, k);
                    if matrix[index + 1][k] == 'M'
                        && matrix[index + 2][k] == 'A'
                        && matrix[index + 3][k] == 'S'
                    {
                        // println!("{:?}{:?},{:?} d", matrix[index][k], index, k);

                        *total += 1;
                    }
                }
            }
        }
    }
    
}

fn rotate_clockwise(contents: &mut Vec<String>) {
    let cols = contents.len();
    let rows = contents[0].len();

    println!("{} {} ", cols, rows);

    let mut rotated = vec![String::new(); cols];

    for col in 0..rows {
        let mut new_row = String::new();
        for row in (0..rows).rev() {
            new_row.push(contents[row].chars().nth(col).unwrap());
        }

        rotated.push(new_row);
    }
    if !rotated.is_empty() {
        *contents = rotated
            .into_iter()
            .filter(|s| !s.trim().is_empty())
            .collect();
    }
}

fn rotate_scan(in_contents: Vec<String>, re: Regex, total: &mut i32) {
    let mut local_total: i32 = 0;
    let mut contents = in_contents;

    for _ in 0..1 {
        for _ in 0..2 {
            rotate_clockwise(&mut contents);
            local_total += horizontally(contents.clone(), re.clone());
        }

        for _ in 0..2 {
            rotate_clockwise(&mut contents);
            local_total += horizontally(contents.clone(), re.clone());
        }
        reverse_all_rows(&mut contents);
    }

    *total += local_total;
}

fn horizontally(contents: Vec<String>, re: Regex) -> i32 {
    let mut total = 0;
    for c in contents {
        let matches: usize = re.find_iter(&c).count();
        total += matches as i32;
    }
    return total;
}

fn reverse_all_rows(contents: &mut Vec<String>) {
    *contents = contents
        .iter()
        .map(|row| row.chars().rev().collect::<String>())
        .collect();
}


fn add_padding(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let cols = matrix[0].len();
    let mut padded_matrix = Vec::new();
    padded_matrix.push(vec!['#'; cols + 2]);

    for row in matrix {
        let mut new_row = vec!['#']; 
        new_row.extend(row);         
        new_row.push('#');           
        padded_matrix.push(new_row);
    }
    padded_matrix.push(vec!['#'; cols + 2]);

    padded_matrix
}