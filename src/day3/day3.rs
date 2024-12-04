use crate::utils::utils::get_file_contents;
use fancy_regex::Regex as FancyRegex;
use regex::Regex;
use std::time::Instant;


pub fn task1() {
    let before = Instant::now();
    let file_path = "inputs/input3.txt";
    let contents = get_file_contents(file_path);

    let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();

    let capture_map: Vec<i32> = re
        .captures_iter(&contents)
        .filter_map(|mul_val| {
            let first = mul_val[1].parse::<i32>().ok()?;
            let second = mul_val[2].parse::<i32>().ok()?;

            Some(first * second)
        })
        .collect();

    let total: i32 = capture_map.iter().sum();
    println!("{:?}", total);
    println!("Day 3 Task 1: Elapsed time: {:.2?}", before.elapsed());
}

pub fn task2() {
    let before = Instant::now();
    let file_path = "inputs/input3.txt";
    let contents = get_file_contents(file_path);
    let mut do_val = true;
    let mut total = 0;
    let re = FancyRegex::new(r"(?:mul\((\d+),(\d+)\))|(?<dont>don't\(\))|(?<do>do\(\))").unwrap();
    for mat in re.captures_iter(&contents) {
        match mat {
            Ok(m) => {
                let mut first: i32 = 0;
                if m.name("do").is_some() {
                    do_val = true;
                    continue;
                }
                if m.name("dont").is_some() {
                    do_val = false;
                    continue;
                }
                if do_val {
                    if let Some(first_s) = m.get(1) {
                        first = first_s.as_str().parse().unwrap();
                    }
                    if let Some(second_s) = m.get(2) {
                        total += first * second_s.as_str().parse::<i32>().unwrap();
                    }
                }
            }
            Err(e) => println!("Error matching: {}", e),
        }
    }
    println!("{}", total);
    println!("Day 3 Task 2: Elapsed time: {:.2?}", before.elapsed());
}

pub fn task2_alternate_map_method() {
    let before = Instant::now();
    let file_path = "inputs/input3.txt";
    let contents = get_file_contents(file_path);
    let mut do_val = true;
    // let mut total = 0;
    let re = FancyRegex::new(r"(?:mul\((\d+),(\d+)\))|(?<dont>don't\(\))|(?<do>do\(\))").unwrap();
    let total = re
        .captures_iter(&contents)
        .filter_map(|captures| {
            if let Ok(m) = captures {
                if m.name("do").is_some() {
                    do_val = true;
                    return None;
                }
                if m.name("dont").is_some() {
                    do_val = false;
                    return None;
                }
                if do_val {
                    let first_s = m.get(1).map(|s| s.as_str().parse::<i32>().unwrap_or(0));
                    let second_s = m.get(2).map(|s| s.as_str().parse::<i32>().unwrap_or(0));
                    if let (Some(first_val), Some(second_val)) = (first_s, second_s) {
                        return Some(first_val * second_val);
                    }
                }
            }
            None
        })
        .fold(0, |first, second| first + second);
    println!("{:?}", total);
    println!("Day 3 Task 2: Elapsed time: {:.2?}", before.elapsed());
}
