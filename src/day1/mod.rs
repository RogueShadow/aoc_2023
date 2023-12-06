use crate::colors::*;

pub fn day1_tests() {
    tests(1);
    let timer = std::time::Instant::now();
    let part1 = part1(include_str!("test.txt"));
    let time = timer.elapsed().as_secs_f32() * 1000.0;
    assert_eq!(part1,142);
    part(1,part1);
    let part2 = part2(include_str!("test2.txt"));
    assert_eq!(part2,281);
    part(2,part2);
}
pub fn day1() {
    title(1);
    let part1 = part1(include_str!("input.txt"));
    assert_eq!(part1,54338);
    part(1,part1);

    let part2 = part2(include_str!("input.txt"));
    assert_eq!(part2,53389);
    part(2,part2);
}

pub fn part1(input: &str) -> usize {
    input.lines().map(|l| {
        let mut result1 = None;
        'one: while result1.is_none() {
            for b in l.as_bytes() {
                match b {
                    b'0'..=b'9' => {
                        result1 = Some((*b - b'0') as usize);
                        break 'one;
                    }
                    _ => {}
                }
            }
        }
        let mut result2 = None;
        'two: while result2.is_none() {
            for b in l.as_bytes().iter().rev() {
                match b {
                    b'0'..=b'9' => {
                        result2 = Some((*b - b'0') as usize);
                        break 'two;
                    }
                    _ => {}
                }
            }
        }
        result1.unwrap() * 10 + result2.unwrap()
    }).sum::<usize>()
}

pub fn part2(input: &str) -> usize {
    const DIGIT: [&'static str;9] = ["one","two","three","four","five","six","seven","eight","nine"];
    const DIGITB: [u8; 9] = [b'1',b'2',b'3',b'4',b'5',b'6',b'7',b'8',b'9'];

    input.lines().map(|l| {
        let result1;
        'first: loop {
            for i in 0..l.len() {
                for (r,s) in DIGIT.iter().enumerate() {
                    if l[i..].starts_with(*s) {
                        result1 = r+1;
                        break 'first;
                    }
                }
                let byte = l[i..i+1].as_bytes()[0];
                if DIGITB.contains(&byte) {
                    result1 = (byte - b'0') as usize;
                    break 'first;
                }
            }
        }
        let result2;
        'second: loop {
            for i in (0..l.len()).rev() {
                for (r,s) in DIGIT.iter().enumerate() {
                    if l[i..].starts_with(*s) {
                        result2 = r+1;
                        break 'second;
                    }
                }
                let byte = l[i..i+1].as_bytes()[0];
                if DIGITB.contains(&byte) {
                    result2 = (byte - b'0') as usize;
                    break 'second;
                }
            }
        }
        result1 * 10 + result2
    }).sum::<usize>()
}