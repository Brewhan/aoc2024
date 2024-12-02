use std::fs;
use std::time::Instant;
use std::collections::HashMap;

pub fn task1(){
    let before = Instant::now();
    let contents = fs::read_to_string("inputs/input1.txt").expect("Expected file contents!");
    let mut diff: Vec<i32> = Vec::new();

    let lines: Vec<Vec<i32>> = contents.lines()
        .map(|line| line.split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect())
        .collect();

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = lines.into_iter()
    .map(|item|(item[0], item[1])
    
    ).fold((Vec::new(), Vec::new()), |(mut l, mut r), (first, second)| {
        l.push(first);
        r.push(second);
        (l,r)
    });
    left.sort();
    right.sort();

    
    //This could be part of the previous let but i really cant be bothered
   for (l,r) in left.iter().zip(right.iter()){
        diff.push((r - l).abs());
   }
   let total: i32 = diff.iter().sum();


   println!("{}", total);
   println!("Day 1 Task 1: Elapsed time: {:.2?}", before.elapsed());

}

pub fn task2(){
    let before = Instant::now();
    let contents = fs::read_to_string("inputs/input1.txt").expect("Expected file contents!");


    let lines: Vec<Vec<i32>> = contents.lines()
        .map(|line| line.split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect())
        .collect();

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = lines.into_iter()
    .map(|item|(item[0], item[1])
    
    ).fold((Vec::new(), Vec::new()), |(mut l, mut r), (first, second)| {
        l.push(first);
        r.push(second);
        (l,r)
    });
    left.sort();
    right.sort();

    let mut right_freq_map= HashMap::new();
    let mut sim_score: Vec<i32> = Vec::new();

    //This could be part of the previous let but i really cant be bothered
   for r in right.iter(){
    *right_freq_map.entry(r).or_insert(0) += 1;

   }

   for l in left.iter(){
    sim_score.push(l * right_freq_map.get(l).unwrap_or(&0));
   }

   let total: i32 = sim_score.iter().sum();
   
   println!("{}", total);
   println!("Day 1 Task 2: Elapsed time: {:.2?}", before.elapsed());

}

