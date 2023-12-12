use std::collections::HashMap;
use crate::run_day;

pub fn day4_tests() {
    let (r1,r2) = run_day("4 Tests", vec![include_str!("test.txt")], part1, part2,Some(1.0));
    assert_eq!(r1,13);
    assert_eq!(r2,30);
}
pub fn day4() {
    let (r1,r2) = run_day("4", vec![include_str!("input.txt")], part1, part2,Some(1.0));
    assert_eq!(r1,18619);
    assert_eq!(r2,8063216);
}
pub fn part1(data: &str) -> u32 {
    let mut cards: HashMap<u32,(Vec<u32>,Vec<u32>)> = HashMap::default();
    data.lines().for_each(|line|{
        let (card_winning,numbers) = line.split_once('|').unwrap();
        let (card,winning) = card_winning.split_once(':').unwrap();
        let card = card[5..].trim().parse::<u32>().unwrap();
        let winning = winning.split_whitespace().map(|n|{
            n.parse::<u32>().unwrap()
        }).collect::<Vec<_>>();
        let numbers = numbers.split_whitespace().map(|n|{
            n.parse::<u32>().unwrap()
        }).collect::<Vec<_>>();
        cards.insert(card,(winning,numbers));
    });
    let mut sum = 0;
    cards.iter().for_each(|(_,(win,have))|{
        let mut result = 0;
        have.iter().for_each(|h| {
            if win.contains(h) {
                match result {
                    0 => result = 1,
                    _ => result *= 2,
                }
            }
        });
        sum += result;
    });
    sum
}
pub fn part2(data: &str) -> u32 {
    let mut cards: Vec<(u32,(Vec<u32>,Vec<u32>))> = vec![];
    data.lines().for_each(|line|{
        let (card_winning,numbers) = line.split_once('|').unwrap();
        let (card,winning) = card_winning.split_once(':').unwrap();
        let card = card[5..].trim().parse::<u32>().unwrap();
        let winning = winning.split_whitespace().map(|n|{
            n.parse::<u32>().unwrap()
        }).collect::<Vec<_>>();
        let numbers = numbers.split_whitespace().map(|n|{
            n.parse::<u32>().unwrap()
        }).collect::<Vec<_>>();
        cards.push((card,(winning,numbers)));
    });

    let mut copies: HashMap<u32,u32> = HashMap::default();
    cards.iter().for_each(|(card,(win,have))|{
        let mut wins = 0;
        have.iter().for_each(|h| {
            if win.contains(h) {
                wins += 1;
            }
        });
        let instances = if let Some(copies) = copies.get(card) {
            1 + copies
        } else {
            1
        };
        (*card+1..=card+wins).for_each(|c|{
            if copies.contains_key(&c) {
                let cards = copies.get(&c).unwrap();
                copies.insert(c,cards + instances);
            }else{
                copies.insert(c, instances);
            }
        });
        //println!("Card {card}:Instances {}: matching numbers {wins}\nAdding Copies Of {:?}",instances,*card+1..=card+wins);
    });
    (cards.len() as u32 + copies.iter().map(|c| c.1).sum::<u32>()) as u32
}