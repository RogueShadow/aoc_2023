use std::fmt::Display;
use crate::colors::{part, title, time};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub mod colors {
    use std::fmt::Display;
    use colorful::{Color, Colorful};
    use colorful::core::color_string::CString;

    pub fn aoc() -> CString {
        "[Advent Of Code 2023]".color(Color::LightRed)
    }
    pub fn title(day: &str) {
        println!("{}", format!("  [Running Day {day}]").color(Color::Aquamarine1a));
    }
    pub fn part<T>(part: u32,value: &T) where T: Display {
        print!("{}",format!("    Part {part}").color(Color::Blue));
        print!("{}", " -> ".color(Color::Yellow));
        print!("{}\n",value.to_string().color(Color::Blue));
    }
    pub fn time(value: std::time::Duration) {
        println!("{}", format!("      Time: {:?}",value).color(Color::Orange1));
    }
}

pub fn run_day<T,G>(day: &str, test: Vec<&str>,p1: fn(&str) -> T, p2: fn(&str) -> G) -> (T,G) where T: Display, G: Display {
    let (p1data,p2data) = if test.len() == 1 {
        (test[0],test[0])
    } else if test.len() == 2 {
        (test[0],test[1])
    } else {
        panic!("length should be 1 or 2 only.")
    };
    let timer = std::time::Instant::now();
    let result1 = p1(p1data);
    let t = timer.elapsed();
    title(day);
    part(1,&result1);
    time(t);

    let timer = std::time::Instant::now();
    let result2 = p2(p2data);
    let t = timer.elapsed();
    part(2,&result2);
    time(t);

    (result1,result2)
}