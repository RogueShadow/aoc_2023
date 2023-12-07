use crate::run_day;


pub fn day2_tests() {
    let (r1,r2) = run_day("2 Tests", vec![include_str!("test.txt")],part1,part2);
    assert_eq!(r1,8);
    assert_eq!(r2,2286);
}
pub fn day2() {
    let (r1,r2) = run_day("2",vec![include_str!("input.txt")],part1,part2);
    assert_eq!(r1,2447);
    assert_eq!(r2,56322);
}

pub fn part1(data: &str) -> usize {
    data.lines().map(|line| {
        let (id,rolls) = line.split_once(':').unwrap();
        let id = parse_usize(&id[5..]);
        let mut colors_max = (0usize,0usize,0usize);
        rolls.split(';').for_each(|handfull|{
            let mut colors = (0usize,0usize,0usize);
            handfull.split(',').for_each(|dice| {
                let (number, color) = dice.trim().split_once(' ').unwrap();
                let number = parse_usize(number);
                match (number, color) {
                    (n, "red") => { colors.0 += n; }
                    (n, "green") => { colors.1 += n; }
                    (n, "blue") => { colors.2 += n; }
                    _ => {}
                };
            });
            colors_max.0 = colors_max.0.max(colors.0);
            colors_max.1 = colors_max.1.max(colors.1);
            colors_max.2 = colors_max.2.max(colors.2);
        });
        (id,colors_max)
    }).filter(|(_,colors)| {
        colors.0 <= 12 && colors.1 <= 13 && colors.2 <= 14
    }).map(|(id,_)| id ).sum::<usize>()
}

pub fn part2(data: &str) -> usize {
    data.lines().map(|line| {
        let rolls= line.split_once(':').unwrap().1;
        let mut colors_max = (0usize,0usize,0usize);
        rolls.split(';').for_each(|handfull|{
            let mut colors = (0usize,0usize,0usize);
            handfull.split(',').for_each(|dice| {
                let (number, color) = dice.trim().split_once(' ').unwrap();
                let number = parse_usize(number);
                match (number, color) {
                    (n, "red") => { colors.0 += n; }
                    (n, "green") => { colors.1 += n; }
                    (n, "blue") => { colors.2 += n; }
                    _ => {}
                };
            });
            colors_max.0 = colors_max.0.max(colors.0);
            colors_max.1 = colors_max.1.max(colors.1);
            colors_max.2 = colors_max.2.max(colors.2);
        });
        colors_max
    }).map(|colors| colors.0 * colors.1 * colors.2 ).sum::<usize>()
}

pub fn parse_usize(str: &str) -> usize {
    let mut result = 0usize;
    for b in str.as_bytes() {
        match b {
            b'0'..=b'9' => { result = result * 10 + (*b - b'0') as usize}
            _ => {}
        }
    }
    result
}