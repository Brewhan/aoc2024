use std::clone;
use std::time::Instant;

use crate::utils::utils::get_file_contents;

pub fn task1() {
    let before = Instant::now();
    let file_path = "inputs/input2.txt";
    let contents = get_file_contents(file_path);

    let lines: Vec<Vec<i32>> = contents.lines()
    .map(|line| line.split(' ')
    .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect())
    .collect();

    let lines_len: i32 = lines.len() as i32;
    let mut unsafe_reports: i32 = 0;

    for line in lines {
        if check_safety(&line) > 0{
            unsafe_reports+=1;
        }
    }

    println!("{}", lines_len - unsafe_reports);
    println!("Day 2 Task 1: Elapsed time: {:.2?}", before.elapsed());
}

fn check_safety(line: &Vec<i32>) -> i32{
    let asc= line[0] < line[1];
    let mut unsafe_line = 0;
    for i in line.windows(2){
        let (cur, next) = (i[0], i[1]);

        if (cur.abs_diff(next)  ==  0 )|| (cur.abs_diff(next) > 3){
            // println!("{} {} {} {:?}", cur, next, cur.abs_diff(next), line);
            unsafe_line+=1;
        } else if (cur < next) && !asc {
            // println!("{} {} {} {:?}", cur, next, "!asc", line);

            unsafe_line+=1;

        } else if (cur > next) && asc {
            // println!("{} {} {} {:?}", cur, next, "asc", line);
            unsafe_line+=1;
        }
            
    }
     return unsafe_line;
}


pub fn task2() {
    let before = Instant::now();
    let file_path = "inputs/input2.txt";
    let contents = get_file_contents(file_path);

    let lines: Vec<Vec<i32>> = contents.lines()
    .map(|line| line.split(' ')
    .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect())
    .collect();

    let lines_len: i32 = lines.len() as i32;
    let mut unsafe_reports: i32 = 0;

    for line in lines {
        fn loop_over_line(line: Vec<i32>) -> i32{
            let mut unsafe_line_count = 0;
            let safety_score = check_safety(&line);
            if safety_score > 0{;
                unsafe_line_count+=1;
           }
            for l in 0..line.len() {

                let mut cloned_line = line.clone();
                cloned_line.remove(l);
                let qa = check_safety(&cloned_line);
                if qa == 0 {
                    if unsafe_line_count > 0 {
                        unsafe_line_count -=1;
                    }
                    break;
                }
            }
            return unsafe_line_count;
        }
        unsafe_reports += loop_over_line(line);
    }
    println!("{}", lines_len - unsafe_reports);
    println!("Day 2 Task 2: Elapsed time: {:.2?}", before.elapsed());
}

