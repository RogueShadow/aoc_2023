use std::ops::Range;
use rayon::prelude::*;
use crate::run_day;

pub fn day5_tests() {
    let (r1,r2) = run_day("5 Tests", vec![include_str!("test.txt")],part1,part2);
    assert_eq!(r1,35);
    assert_eq!(r2,46);
}
pub fn day5() {
    let (r1,r2) = run_day("5", vec![include_str!("input.txt")],part1,part2);
    assert_eq!(r1,484023871);
    assert_eq!(r2,46294175);
}
pub fn part1(data: &str) -> u128 {
    let mut lines = data.lines();
    let seeds = lines.next().unwrap().split_once(':').unwrap().1.split_whitespace().map(|seed| seed.parse::<u128>().unwrap()).collect::<Vec<_>>();
    let mut seed_soil = vec![];
    let mut soil_fert = vec![];
    let mut fert_water = vec![];
    let mut water_light = vec![];
    let mut light_temp = vec![];
    let mut temp_humidity = vec![];
    let mut hum_loc = vec![];
    let mut state = 0;
    lines.by_ref().for_each(|line| {
       if line.starts_with("seed-to-soil") {
           state = 0;
       } else if line.starts_with("soil-to-fertilizer") {
           state = 1;
       }else if line.starts_with("fertilizer-to-water") {
           state = 2;
       }else if line.starts_with("water-to-light") {
           state = 3;
       }else if line.starts_with("light-to-temperature") {
           state = 4;
       }else if line.starts_with("temperature-to-humidity") {
           state = 5;
       }else if line.starts_with("humidity-to-location") {
           state = 6;
       }else if !line.is_empty() {
           let range = line.split_whitespace().map(|num| num.parse::<u128>().unwrap()).collect::<Vec<_>>();
           match state {
               0 => seed_soil.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
               1 => soil_fert.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
               2 => fert_water.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
               3 => water_light.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
               4 => light_temp.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
               5 => temp_humidity.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
               6 => hum_loc.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
               _ => println!("Invalid state!"),
           }
       }
    });
    seeds.iter().by_ref().map(|seed| {
        let mut number = *seed;
        number = check(&seed_soil,number);
        number = check(&soil_fert,number);
        number = check(&fert_water,number);
        number = check(&water_light,number);
        number = check(&light_temp,number);
        number = check(&temp_humidity,number);
        number = check(&hum_loc,number);
        number
    }).min().unwrap()
}

pub fn check(ranges: &Vec<(Range<u128>, Range<u128>)>, number: u128) -> u128 {
    for ss in ranges {
        if ss.1.contains(&number) {
            let pos = number - ss.1.start;
            return ss.0.start+pos;
        }
    }
    return number;
}
pub fn part2(data: &str) -> u128 {
    let mut lines = data.lines();
    let seeds = lines.next().unwrap().split_once(':').unwrap().1.split_whitespace().map(|seed| seed.parse::<u128>().unwrap()).collect::<Vec<_>>();
    let seeds = seeds.chunks(2).map(|c| c[0]..c[0]+c[1]).collect::<Vec<_>>();
    let mut seed_soil = vec![];
    let mut soil_fert = vec![];
    let mut fert_water = vec![];
    let mut water_light = vec![];
    let mut light_temp = vec![];
    let mut temp_humidity = vec![];
    let mut hum_loc = vec![];
    let mut state = 0;
    lines.by_ref().for_each(|line| {
        if line.starts_with("seed-to-soil") {
            state = 0;
        } else if line.starts_with("soil-to-fertilizer") {
            state = 1;
        }else if line.starts_with("fertilizer-to-water") {
            state = 2;
        }else if line.starts_with("water-to-light") {
            state = 3;
        }else if line.starts_with("light-to-temperature") {
            state = 4;
        }else if line.starts_with("temperature-to-humidity") {
            state = 5;
        }else if line.starts_with("humidity-to-location") {
            state = 6;
        }else if !line.is_empty() {
            let range = line.split_whitespace().map(|num| num.parse::<u128>().unwrap()).collect::<Vec<_>>();
            match state {
                0 => seed_soil.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
                1 => soil_fert.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
                2 => fert_water.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
                3 => water_light.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
                4 => light_temp.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
                5 => temp_humidity.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
                6 => hum_loc.push((range[0]..range[0]+range[2],range[1]..range[1]+range[2])),
                _ => println!("Invalid state!"),
            }
        }
    });
    let len = seeds.len();
    seeds.par_iter().enumerate().map(|(i,range)| {
        let mut result = u128::MAX;
        range.clone().into_iter().for_each(|seed| {
            let mut number = seed;
            number = check2(&seed_soil, number);
            number = check2(&soil_fert, number);
            number = check2(&fert_water, number);
            number = check2(&water_light, number);
            number = check2(&light_temp, number);
            number = check2(&temp_humidity, number);
            number = check2(&hum_loc, number);
            result = result.min(number);
        });
        result
    }).min().unwrap()
}
#[inline(always)]
pub fn check2(ranges: &Vec<(Range<u128>, Range<u128>)>, number: u128) -> u128 {
    for ss in ranges {
        if ss.1.contains(&number) {
            let pos = number - ss.1.start;
            return ss.0.start + pos;
        }
    }
    return number;
}

pub fn part2_smarter(data: &str) -> u128 {
    let mut lines = data.lines();
    let seeds = lines.next().unwrap().split_once(':').unwrap().1.split_whitespace().map(|seed| seed.parse::<u128>().unwrap()).collect::<Vec<_>>();
    let seeds = seeds.chunks(2).map(|c| c[0]..c[0]+c[1]).collect::<Vec<_>>();
    let mut seed_soil = vec![];
    let mut soil_fert = vec![];
    let mut fert_water = vec![];
    let mut water_light = vec![];
    let mut light_temp = vec![];
    let mut temp_humidity = vec![];
    let mut hum_loc = vec![];
    let mut state = 0;
    lines.for_each(|line| {
        if line.starts_with("seed-to-soil") {
            state = 0;
        } else if line.starts_with("soil-to-fertilizer") {
            state = 1;
        }else if line.starts_with("fertilizer-to-water") {
            state = 2;
        }else if line.starts_with("water-to-light") {
            state = 3;
        }else if line.starts_with("light-to-temperature") {
            state = 4;
        }else if line.starts_with("temperature-to-humidity") {
            state = 5;
        }else if line.starts_with("humidity-to-location") {
            state = 6;
        }else if !line.is_empty() {
            let mut range = line.split_whitespace().map(|num| num.parse::<u128>().unwrap());
            let start1 = range.next().unwrap();
            let start2 = range.next().unwrap();
            let len = range.next().unwrap();
            let ranges = (start1..start1+len,start2..start2+len);
            match state {
                0 => seed_soil.push(ranges),
                1 => soil_fert.push(ranges),
                2 => fert_water.push(ranges),
                3 => water_light.push(ranges),
                4 => light_temp.push(ranges),
                5 => temp_humidity.push(ranges),
                6 => hum_loc.push(ranges),
                _ => println!("Invalid state!"),
            }
        }
    });

    hum_loc.sort_by(|a,b|a.0.start.cmp(&b.0.start));
    let mut result = 0u128;
    'outer: for r in hum_loc.iter() {
        for loc in r.0.clone().into_iter() {
            let mut number = loc;
            let number = check_reverse2(&hum_loc,&number);
            let number = check_reverse2(&temp_humidity,&number);
            let number = check_reverse2(&light_temp,&number);
            let number = check_reverse2(&water_light,&number);
            let number = check_reverse2(&fert_water,&number);
            let number = check_reverse2(&soil_fert,&number);
            let number = check_reverse2(&seed_soil,&number);
            for sr in seeds.iter() {
                if number >= sr.start && number < sr.end {
                    result = loc;
                    break 'outer;
                }
            }
        }
    }

    result
}

pub fn check_reverse2(ranges: &Vec<(Range<u128>,Range<u128>)>, number: &u128) -> u128 {
    for ss in ranges {
        if number >= &ss.0.start && number < &ss.0.end {
            let pos = number - ss.0.start;
            return ss.1.start + pos;
        }
    }
    return *number;
}