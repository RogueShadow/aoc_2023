use crate::run_day;

pub fn day9_tests() {
    let (r1,r2) = run_day("9 Tests",vec![include_str!("test.txt")],part1,part2);
    assert_eq!(r1,114);
    assert_eq!(r2,2);
}
pub fn day9() {
    let (r1,r2) = run_day("9",vec![include_str!("input.txt")],part1,part2);
    assert_eq!(r1,1647269739);
    assert_eq!(r2,864);
}

pub fn part1(input: &str) -> i32 {
    let histories = input.lines().map(|line|{
        line.split_whitespace().map(|value| value.parse::<i32>().unwrap()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    histories.iter().map(|history| {
        let mut sequences = vec![history.clone()];
        loop {
            sequences.push(sequences.last().unwrap().windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>());
            let sum = sequences.last().unwrap().iter().sum::<i32>();
            if sum == 0 {
                break;
            }
        }
        let ends = sequences.iter().map(|s| s.last().unwrap()).collect::<Vec<_>>();
        ends.iter().fold(0, |acc, e| **e + acc)
    }).sum::<i32>()
}

pub fn part2(input: &str) -> i32 {
    let histories = input.lines().map(|line|{
        line.split_whitespace().map(|value| value.parse::<i32>().unwrap()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    histories.iter().map(|history| {
        let mut sequences = vec![history.clone()];
        loop {
            sequences.push(sequences.last().unwrap().windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>());
            let sum = sequences.last().unwrap().iter().sum::<i32>();
            if sum == 0 {
                break;
            }
        }
        let ends = sequences.iter().map(|s| s.first().unwrap()).collect::<Vec<_>>();
        let result = ends.iter().rev().fold(0, |acc, e| {
            **e - acc
        });
        result
    }).sum::<i32>()
}