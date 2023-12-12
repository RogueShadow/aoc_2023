use std::fs::{ read_to_string};
use crate::{bench, run_day};



pub fn day11_tests() {
    let test = &*read_to_string("src/day11/test.txt").unwrap();
    let (r1,r2) = run_day("11 Tests",vec![test],part1,part2,None);
    assert_eq!(r1, 374);

}
pub fn day11() {
    let data = &*read_to_string("src/day11/input.txt").unwrap();
    let (r1,r2) = run_day("11",vec![data],part1,part2,None);
    assert_eq!(r1,9370588);
    assert_eq!(r2,746207878188);
}

pub fn part1(input: &str) -> i32 {
    let (mut gx,mut gy) = (vec![],vec![]);
    input.lines().enumerate().for_each(|(y,line)| {
        line.as_bytes().iter().enumerate().for_each(|(x,b)|{
            if b == &b'#' {
                gx.push(x as i32);
                gy.push(y as i32);
            }
        });
    });


    let mut empties_x = vec![];
    for testx in (0..*gx.iter().max().unwrap()).rev() {
        if !gx.contains(&testx) { empties_x.push(testx); }
    }
    let mut empties_y = vec![];
    for testy in (0..*gy.iter().max().unwrap()).rev() {
        if !gy.contains(&testy) { empties_y.push(testy); }
    }
    for tx in empties_x.iter() {
        for x in gx.iter_mut().rev() {
            if *x > *tx { *x += 1 }
        }
    }
    for ty in empties_y.iter() {
        for y in gy.iter_mut().rev() {
            if *y > *ty { *y += 1 }
        }
    }

    let mut index = 0;
    let mut pairs = vec![];
    for i in 0..gx.len() {
        for iy in index..gy.len() {
            if i != iy {
                pairs.push((i,iy));
            }
        }
        index += 1;
    }

    let mut distance = vec![];
    for (i1,i2) in pairs {
        let dx = (gx[i1] - gx[i2]).abs();
        let dy = (gy[i1] - gy[i2]).abs();
        distance.push(dx + dy)
    }

    distance.iter().sum()
}

pub fn part2(input: &str) -> i64 {
    let (mut gx,mut gy) = (vec![],vec![]);
    input.lines().enumerate().for_each(|(y,line)| {
        line.as_bytes().iter().enumerate().for_each(|(x,b)|{
            if b == &b'#' {
                gx.push(x as i64);
                gy.push(y as i64);
            }
        });
    });


    let mut empties_x = vec![];
    for testx in (0..*gx.iter().max().unwrap()).rev() {
        if !gx.contains(&testx) { empties_x.push(testx); }
    }
    let mut empties_y = vec![];
    for testy in (0..*gy.iter().max().unwrap()).rev() {
        if !gy.contains(&testy) { empties_y.push(testy); }
    }
    for tx in empties_x.iter() {
        for x in gx.iter_mut().rev() {
            if *x > *tx { *x += 999_999 }
        }
    }
    for ty in empties_y.iter() {
        for y in gy.iter_mut().rev() {
            if *y > *ty { *y += 999_999 }
        }
    }

    let mut index = 0;
    let mut pairs = vec![];
    for i in 0..gx.len() {
        for iy in index..gy.len() {
            if i != iy {
                pairs.push((i,iy));
            }
        }
        index += 1;
    }

    let mut distance = vec![];
    for (i1,i2) in pairs {
        let dx = (gx[i1] - gx[i2]).abs();
        let dy = (gy[i1] - gy[i2]).abs();
        distance.push(dx + dy)
    }

    distance.iter().sum()
}
