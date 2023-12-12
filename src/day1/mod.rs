use crate::run_day;

pub fn day1_tests() {
    let (r1,r2) = run_day("1 Tests",vec![include_str!("test.txt"),include_str!("test2.txt")],part1,part2,Some(1.0));
    assert_eq!(r1,142);
    assert_eq!(r2,281);
}
pub fn day1() {
    let (r1,r2) = run_day("1",vec![include_str!("input.txt")],part1,part2,Some(1.0));
    assert_eq!(r1,54338);
    assert_eq!(r2,53389);
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