use crate::run_day;

pub fn day6_tests() {
    let (r1,r2) = run_day("6 Tests", vec![include_str!("test.txt")],part1,part2,Some(1.0));
    assert_eq!(r1,288);
    assert_eq!(r2,71503);
}
pub fn day6() {
    let (r1,r2) = run_day("6", vec![include_str!("input.txt")],part1,part2,Some(1.0));
    assert_eq!(r1, 1710720);
    assert_eq!(r2, 35349468);
}

pub fn part1(data: &str) -> usize {
    let mut lines = data.lines();
    let time = lines.next().unwrap()
        .strip_prefix("Time:").unwrap()
        .split_whitespace().map(|time| parse_u128(time));

    let dist = lines.next().unwrap()
        .strip_prefix("Distance:").unwrap()
        .split_whitespace().map(|dist| parse_u128(dist));

    let races = time.zip(dist).map(|(t,d)|{
        Race::new(t,d)
    }).collect::<Vec<_>>();


    let product = races.iter().map(|race| {
        (0..=race.length).map(|held|{
           boat_distance(held,race)
        }).filter(|rr| rr.won).count()
    }).product::<usize>();

    product
}
#[inline(always)]
pub fn boat_distance(held: u128, race: &Race) -> RaceResult {
    let distance = held * (race.length - held);
    RaceResult {
        held,
        distance,
        won: distance > race.record,
    }
}

pub fn part2(data: &str) -> u64 {
    let mut lines = data.lines();
    let mut time = 0;
    lines.next().unwrap().strip_prefix("Time:").unwrap().bytes().for_each(|b|{
        match b {
            b'0'..=b'9' => time = time * 10 + ((b - b'0') as u64),
            _ => {}
        }
    });

    let mut dist = 0;
    lines.next().unwrap().strip_prefix("Distance:").unwrap().bytes().for_each(|b| {
        match b {
            b'0'..=b'9' => dist = dist * 10 + ((b - b'0') as u64),
            _ => {}
        }
    });

    let mut lowest = 0;
    for held in 0..=time {
        if (held * (time - held)) >= dist {
            lowest = held;
            break;
        }
    }
    let mut greatest = u64::MAX;
    for held in (0..=time).rev() {
        if (held * (time - held)) >= dist {
            greatest = held;
            break;
        }
    }

    (greatest - lowest) + 1
}

#[derive(Debug)]
pub struct Race {
    length: u128,
    record: u128,
}
impl Race {
    pub fn new(length: u128, record: u128) -> Self {
        Self {
            length,
            record,
        }
    }
}
#[derive(Debug)]
pub struct RaceResult {
    held: u128,
    distance: u128,
    won: bool,
}
pub fn parse_u128(str: &str) -> u128 {
    let mut result = 0;
    for b in str.as_bytes() {
        match b {
            b'0'..=b'9' => { result = result * 10 + (*b - b'0') as u128}
            _ => {}
        }
    }
    result
}