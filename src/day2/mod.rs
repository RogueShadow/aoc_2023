
pub fn day2_tests() {
    println!("Day2 Part1 Test: {:?}",part1(include_str!("test.txt")));
    println!("Day2 Part2 Test: {:?}",part2(include_str!("test.txt")));
}
pub fn day2() {
    println!("Day2 Part1 Full: {:?}",part1(include_str!("input.txt")));
    println!("Day2 Part2 Full: {:?}",part2(include_str!("input.txt")));
}

pub fn part1(data: &str) -> usize {
    let result = data.lines().map(|line| {
        let (id,rolls) = line.split_once(':').unwrap();
        let id = id[5..].parse::<usize>().unwrap();
        let cubes = rolls.split(';').map(|handfull|{
            let mut colors = (0usize,0usize,0usize);
            handfull.split(',').for_each(|dice| {
                let (number, color) = dice.trim().split_once(' ').unwrap();
                let number = number.parse::<usize>().unwrap();
                match (number, color) {
                    (n, "red") => { colors.0 += n; }
                    (n, "green") => { colors.1 += n; }
                    (n, "blue") => { colors.2 += n; }
                    _ => {}
                };
            });
            colors
        }).collect::<Vec<_>>();
        Game {id, cubes}
    }).collect::<Vec<_>>();

    let possible = result.iter().filter(|game| {
        let max = game.max();
        max.0 <= 12 && max.1 <= 13 && max.2 <= 14
    }).collect::<Vec<_>>();
    possible.iter().map(|game| game.id).sum::<usize>()
}

pub fn part2(data: &str) -> usize {
    let result = data.lines().map(|line| {
        let (id,rolls) = line.split_once(':').unwrap();
        let id = id[5..].parse::<usize>().unwrap();
        let cubes = rolls.split(';').map(|handfull|{
            let mut colors = (0usize,0usize,0usize);
            handfull.split(',').for_each(|dice| {
                let (number, color) = dice.trim().split_once(' ').unwrap();
                let number = number.parse::<usize>().unwrap();
                match (number, color) {
                    (n, "red") => { colors.0 += n; }
                    (n, "green") => { colors.1 += n; }
                    (n, "blue") => { colors.2 += n; }
                    _ => {}
                };
            });
            colors
        }).collect::<Vec<_>>();
        Game {id, cubes}
    }).collect::<Vec<_>>();

    result.iter().map(|game| game.power()).sum::<usize>()
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    cubes: Vec<(usize,usize,usize)>,
}
impl Game {
    pub fn max(&self) -> (usize,usize,usize) {
        let mut max = (0,0,0);
        self.cubes.iter().for_each(|set|  {
            max.0 = max.0.max(set.0);
            max.1 = max.1.max(set.1);
            max.2 = max.2.max(set.2);
        });
        max
    }
    pub fn power(&self) -> usize {
        let max = self.max();
        max.0 * max.1 * max.2
    }
}