use std::cmp::Ordering;
use std::collections::HashMap;
use crate::run_day;

pub fn day7_tests() {
    let (r1,r2) = run_day("7 Tests", vec![include_str!("test.txt")],part1,part2,Some(1.0));
    assert_eq!(r1,6440);
    assert_eq!(r2,5905);
}
pub fn day7() {
    let (r1,r2) = run_day("7", vec![include_str!("input.txt")],part1,part2,Some(1.0));
    assert_eq!(r1, 250453939);
    assert_eq!(r2,248652697);
}

pub fn part1(data: &str) -> u32 {
    let mut hands = data.lines().map(|hand| {
        let mut split = hand.split_whitespace();
        let cards = split.next().unwrap();
        let mut cards = (0..cards.len()).map(|i|{
           Card::from(&cards[i..i + 1])
        }).collect::<Vec<_>>();
        let mut c = cards.drain(0..);
        let bid = split.next().unwrap().parse::<u32>().unwrap();
        Hand {
            cards: [c.next().unwrap(),c.next().unwrap(),c.next().unwrap(),c.next().unwrap(),c.next().unwrap()],
            bid,
        }
    }).collect::<Vec<_>>();
    let mut winnings = 0u32;

    hands.sort_by(|a,b| compare_hands(a,b).reverse());
    for h in hands.iter().enumerate() {
        let rank = h.0 + 1;
        winnings += rank as u32 * h.1.bid;
    }
    winnings
}

pub fn part2(data: &str) -> u32 {
    let mut hands = data.lines().map(|hand| {
        let mut split = hand.split_whitespace();
        let cards = split.next().unwrap();
        let mut cards = (0..cards.len()).map(|i|{
            Card2::from(&cards[i..i + 1])
        }).collect::<Vec<_>>();
        let mut c = cards.drain(0..);
        let bid = split.next().unwrap().parse::<u32>().unwrap();
        Hand2 {
            cards: [c.next().unwrap(),c.next().unwrap(),c.next().unwrap(),c.next().unwrap(),c.next().unwrap()],
            bid,
        }
    }).collect::<Vec<_>>();
    let mut winnings = 0u32;
    hands.sort_by(|a,b| compare_hands2(a,b).reverse());
    for h in hands.iter().enumerate() {
        let rank = h.0 + 1;
        winnings += rank as u32 * h.1.bid;
        //println!("Hand: {:?}\nType: {:?} Rank: {:?}",h.1.cards,h.1.hand_type(),rank);
    }
    winnings
}

#[derive(Debug)]
pub struct Hand {
    cards: [Card; 5],
    bid: u32,
}
impl Hand {
    pub fn hand_type(&self) -> HandType {
        let mut count = count_cards(&self.cards);
        count.sort_by(|a,b|b.1.cmp(&a.1));
        return if count[0].1 == 5 {
            HandType::Five
        } else if count[0].1 == 4 {
            HandType::Four
        } else if count[0].1 == 3 && count[1].1 == 2 {
            HandType::Full
        } else if count[0].1 == 3 {
            HandType::Three
        } else if count[0].1 == 2 && count[1].1 == 2 {
            HandType::Two
        } else if count[0].1 == 2 {
            HandType::One
        } else { HandType::High }
    }
}
#[derive(Debug)]
pub struct Hand2 {
    cards: [Card2; 5],
    bid: u32,
}
impl Hand2 {
    pub fn hand_type(&self) -> HandType {
        let mut count = count_cards2(&self.cards);
        let jokers = count.iter().filter(|(c,_)| c == &Card2::Joker).collect::<Vec<_>>();
        let mut others = count.iter().filter(|(c,_)| c != &Card2::Joker).collect::<Vec<_>>();
        let jokers = if !jokers.is_empty() {jokers[0].1} else {0};

        others.sort_by(|a,b|b.1.cmp(&a.1));

        if others.len() <= 1 {
            return HandType::Five;
        }
        return if others[0].1 + jokers == 5 {
            HandType::Five
        } else if others[0].1 + jokers == 4 {
            HandType::Four
        } else if (others[0].1 + jokers == 3 && others[1].1 == 2) {
            HandType::Full
        } else if others[0].1 + jokers == 3 {
            HandType::Three
        } else if (others[0].1 + jokers == 2 && others[1].1 == 2) || (others[0].1 == 2 && others[1].1 + jokers == 2) {
            HandType::Two
        } else if others[0].1 + jokers == 2 {
            HandType::One
        } else { HandType::High }
    }
}
pub fn count_cards(cards: &[Card; 5]) -> Vec<(Card,u32)> {
    let mut result = HashMap::new();
    cards.iter().for_each(|c| {
        let count = *result.get(c).unwrap_or(&0u32);
        result.insert(*c,count + 1);
    });
    result.iter().map(|(k,v)| (*k,*v) ).collect::<Vec<_>>()
}
pub fn count_cards2(cards: &[Card2; 5]) -> Vec<(Card2,u32)> {
    let mut result = HashMap::new();
    cards.iter().for_each(|c| {
        let count = *result.get(c).unwrap_or(&0u32);
        result.insert(*c,count + 1);
    });
    result.iter().map(|(k,v)| (*k,*v) ).collect::<Vec<_>>()
}
pub fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    let aht = a.hand_type();
    let bht = b.hand_type();
    return if aht > bht {
        Ordering::Greater
    } else if aht < bht {
        Ordering::Less
    } else {
        for i in 0..5 {
            let ord = a.cards[i].cmp(&b.cards[i]);
            if ord != Ordering::Equal {
                return ord
            }
        }
        panic!("Couldn't determine greater hand");
    }
}
pub fn compare_hands2(a: &Hand2, b: &Hand2) -> Ordering {
    let aht = a.hand_type();
    let bht = b.hand_type();
    return if aht > bht {
        Ordering::Greater
    } else if aht < bht {
        Ordering::Less
    } else {
        for i in 0..5 {
            let ord = a.cards[i].cmp(&b.cards[i]);
            if ord != Ordering::Equal {
                return ord
            }
        }
        panic!("Couldn't determine greater hand");
    }
}
#[derive(Debug,Ord, PartialOrd, Eq, PartialEq)]
pub enum HandType {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}
#[derive(Hash,Ord, PartialOrd, Eq, PartialEq,Debug,Copy, Clone)]
pub enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}
#[derive(Hash,Ord, PartialOrd, Eq, PartialEq,Debug,Copy, Clone)]
pub enum Card2 {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}


impl From<&str> for Card {
    fn from(value: &str) -> Self {
        match value {
            "A" => Card::Ace,
            "K" => Card::King,
            "Q" => Card::Queen,
            "J" => Card::Jack,
            "T" => Card::Ten,
            "9" => Card::Nine,
            "8" => Card::Eight,
            "7" => Card::Seven,
            "6" => Card::Six,
            "5" => Card::Five,
            "4" => Card::Four,
            "3" => Card::Three,
            "2" => Card::Two,
            _ => panic!("Invalid Card"),
        }
    }
}
impl From<&str> for Card2 {
    fn from(value: &str) -> Self {
        match value {
            "A" => Card2::Ace,
            "K" => Card2::King,
            "Q" => Card2::Queen,
            "T" => Card2::Ten,
            "9" => Card2::Nine,
            "8" => Card2::Eight,
            "7" => Card2::Seven,
            "6" => Card2::Six,
            "5" => Card2::Five,
            "4" => Card2::Four,
            "3" => Card2::Three,
            "2" => Card2::Two,
            "J" => Card2::Joker,
            _ => panic!("Invalid Card"),
        }
    }
}