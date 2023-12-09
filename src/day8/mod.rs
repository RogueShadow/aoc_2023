use std::collections::HashMap;
use crate::run_day;

pub fn day8_tests() {
    let (r1,r2) = run_day("8 Tests", vec![include_str!("test.txt"),include_str!("test2.txt")],part1,part2);
    assert_eq!(r1,2);
    assert_eq!(r2,6);
}
pub fn day8() {
    let (r1,r2) = run_day("8", vec![include_str!("input.txt")],part1,part2);
    assert_eq!(r1,14681);
    assert_eq!(r2,14321394058031)
}

pub fn part1(data: &str) -> u32 {
    let mut lines = data.lines();
    let directions = lines.next().unwrap().chars().map(Dir::from).collect::<Vec<_>>();
    let nodes = lines.filter_map(|line| {
        if !line.is_empty() {
            let b = line.as_bytes();
            let id: [u8; 3] = [b[0], b[1], b[2]];
            let left = [b[7], b[8], b[9]];
            let right = [b[12], b[13], b[14]];
            Some((id,(left, right)))
        } else {None}
    }).collect::<HashMap<[u8; 3],([u8;3],[u8;3])>>();
    let mut position = [65u8;3];
    let mut steps = 0u32;
    let destination = [90u8;3];
    while position != destination {
        directions.iter().for_each(|dir| {
            match dir {
                Dir::Left => {
                    position = nodes.get(&position).unwrap().0;
                }
                Dir::Right => {
                    position = nodes.get(&position).unwrap().1;
                }
            }
            steps += 1;
        });
    }
    steps
}

pub fn part2(data: &str) -> u128 {
    let mut lines = data.lines();
    let directions = lines.next().unwrap().chars().map(Dir::from).collect::<Vec<_>>();
    let nodes = lines.filter_map(|line| {
        if !line.is_empty() {
            let b = line.as_bytes();
            let id: [u8; 3] = [b[0], b[1], b[2]];
            let left = [b[7], b[8], b[9]];
            let right = [b[12], b[13], b[14]];
            Some((id,(left, right)))
        } else {None}
    }).collect::<HashMap<[u8; 3],([u8;3],[u8;3])>>();

    let mut positions = nodes.iter().filter(|(k,v)| k[2] == 65u8 ).map(|(k,v)| *k).collect::<Vec<_>>();
    let mut steps = 0u64;
    let mut pos_steps = positions.iter().map(|_| None).collect::<Vec<_>>();

    'exit: loop {
        for dir in directions.iter() {
            for (idx, position) in positions.iter_mut().enumerate() {
                match dir {
                    Dir::Left => {
                        let _ = std::mem::replace(position, nodes.get(position).unwrap().0);
                    }
                    Dir::Right => {
                        let _ = std::mem::replace(position, nodes.get(position).unwrap().1);
                    }
                }
                if position[2] == 90u8 {
                    if pos_steps[idx].is_none() {pos_steps[idx] = Some(steps+1)};
                }
            }
            steps += 1;
            if pos_steps.iter().filter(|s| s.is_none()).count() == 0 {
                break 'exit;
            }
        }
    }
    let pos_steps = pos_steps.iter().map(|s| s.unwrap() as u128).collect::<Vec<_>>();

    let mut ans = 1u128;
    pos_steps.iter().for_each(|x| {
        ans = (x * ans) / (gcd::euclid_u128(*x,ans));
    });

    ans
}

#[derive(Debug)]
pub enum Dir {
    Right,
    Left,
}
impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'R' => Dir::Right,
            'L' => Dir::Left,
            _ => panic!("Invalid Dir")
        }
    }
}