pub mod day1;
pub mod day2;
pub mod day3;

pub mod colors {
    use std::fmt::Display;
    use colorful::{Color, Colorful};
    use colorful::core::color_string::CString;

    pub fn aoc() -> CString {
        "[Advent Of Code 2023]".color(Color::LightRed)
    }
    pub fn title(day: u32) {
        println!("{}", format!("  [Running Day {day}]").color(Color::Aquamarine1a));
    }
    pub fn part<T>(part: u32,value: T) where T: Display {
        print!("{}",format!("    Part {part}").color(Color::Blue));
        print!("{}", " -> ".color(Color::Yellow));
        print!("{}\n",value.to_string().color(Color::Blue));
    }
    pub fn time(value: f32) {
        println!("{}", format!("    Time: {}ms",value).color(Color::Orange1));
    }
}
