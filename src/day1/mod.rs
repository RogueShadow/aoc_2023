use std::collections::HashMap;

pub fn day1_tests() {
    println!("Part1 Test: {}",part1(include_str!("test.txt")));
    println!("Part2 Test: {}",part2(include_str!("test2.txt")));
}
pub fn day1() {
    println!("Part1: {}",part1(include_str!("input.txt")));
    println!("Part2: \
    {}",part2(include_str!("input.txt")));
}

pub fn part1(input: &str) -> usize {
    let result = input.lines().map(|l| {
        let mut result1 = None;
        while result1.is_none() {
            for i in 0..l.len() {
                let test = l[i..i+1].parse::<usize>();
                if test.is_ok() {
                    result1 = Some(test.unwrap());
                    break;
                }
            }
        }
        let mut result2 = None;
        while result2.is_none() {
            for i in (0..l.len()).rev() {
                let test = l[i..i+1].parse::<usize>();
                if test.is_ok() {
                    result2 = Some(test.unwrap());
                    break;
                }
            }
        }
        result1.unwrap() * 10 + result2.unwrap()
    }).collect::<Vec<_>>();
    result.iter().sum::<usize>()
}

pub fn part2(input: &str) -> usize {
    let digits = HashMap::from(
        [("one",1usize),("two",2),("three",3),("four",4),("five",5),
            ("six",6),("seven",7),("eight",8),("nine",9)]
    );

    let result = input.lines().map(|l| {
        let mut result1 = None;
        'first: while result1.is_none() {
            for i in 0..l.len() {
                for (k,v) in digits.iter() {
                    if l[i..].starts_with(*k) {
                        result1 = Some(*v);
                        break 'first;
                    }
                }
                let test = l[i..i+1].parse::<usize>();
                if test.is_ok() {
                    result1 = Some(test.unwrap());
                    break;
                }
            }
        }
        let mut result2 = None;
        'second: while result2.is_none() {
            for i in (0..l.len()).rev() {
                for (k,v) in digits.iter() {
                    if l[i..].starts_with(*k) {
                        result2 = Some(*v);
                        break 'second;
                    }
                }
                let test = l[i..i+1].parse::<usize>();
                if test.is_ok() {
                    result2 = Some(test.unwrap());
                    break;
                }
            }
        }
        result1.unwrap() * 10 + result2.unwrap()
    }).collect::<Vec<_>>();
    result.iter().sum::<usize>()
}