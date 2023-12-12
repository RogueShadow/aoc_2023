use std::fs::{ read_to_string};
use crate::{bench, run_day};



pub fn day9_tests() {
    let test = &*read_to_string("src/day9/test.txt").unwrap();
    let (r1,r2) = run_day("9 Tests",vec![test],part1,part2,Some(1.0));
    assert_eq!(r1,114);
    assert_eq!(r2,2);

}
pub fn day9() {
    let data = &*read_to_string("src/day9/input.txt").unwrap();
    let (r1,r2) = run_day("9",vec![data],part1,part2,Some(1.0));
    assert_eq!(r1,1647269739);
    assert_eq!(r2,864);
}

pub fn part1(input: &str) -> i32 {
    input.lines().map(|line|{
        line.split_whitespace().map(|value| value.parse::<i32>().unwrap()).collect::<Vec<_>>()
    }).map(|history| {
        let mut sequences = vec![history];
        loop {
            sequences.push(sequences.last().unwrap().windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>());
            let sum = sequences.last().unwrap().iter().sum::<i32>();
            if sum == 0 {
                break;
            }
        }
        sequences.iter().map(|s| s.last().unwrap()).fold(0, |acc, e| *e + acc)
    }).sum::<i32>()
}

pub fn part2(input: &str) -> i32 {
    input.lines().map(|line|{
        line.split_whitespace().map(|value| value.parse::<i32>().unwrap()).collect::<Vec<_>>()
    }).map(|history| {
        let mut sequences = vec![history];
        loop {
            sequences.push(sequences.last().unwrap().windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>());
            let sum = sequences.last().unwrap().iter().sum::<i32>();
            if sum == 0 {
                break;
            }
        }
        sequences.iter().map(|s| s.first().unwrap()).rev().fold(0, |acc, e| *e - acc)
    }).sum::<i32>()
}
