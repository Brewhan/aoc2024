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
    let contents: Vec<String> = contents
        .split("\r\n")
        .map(|s| s.to_string())
        .filter(|s| !s.trim().is_empty())
        .collect();
    traverse(contents.clone(), &mut total);
    println!("Total: {}", total);
}

fn traverse(contents: Vec<String>, total: &mut i32) {
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