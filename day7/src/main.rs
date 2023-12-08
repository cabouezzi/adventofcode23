use compare::{natural, Compare};
use core::panic;
use std::{cmp::Ordering, collections::HashMap, fs};

enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandKind {
    fn rank(&self) -> usize {
        match *self {
            HandKind::HighCard => 1,
            HandKind::OnePair => 2,
            HandKind::TwoPair => 3,
            HandKind::ThreeKind => 4,
            HandKind::FullHouse => 5,
            HandKind::FourKind => 6,
            HandKind::FiveKind => 7,
        }
    }

    fn compare(&self, other: &Self) -> Ordering {
        return natural().compare(&self.rank(), &other.rank());
    }
}

struct HandData {
    raw_value: String,
    kind: HandKind,
    ordered_cards: Vec<usize>,
    bid: usize,
}

fn read_hand(hand: String) -> (HandKind, Vec<usize>) {
    fn to_numeric(c: char) -> usize {
        let map: HashMap<char, usize> = [('T', 10), ('J', 1), ('Q', 12), ('K', 13), ('A', 14)]
            .iter()
            .cloned()
            .collect();
        if c.is_numeric() {
            return c.to_string().parse::<usize>().unwrap();
        } else {
            return map.get(&c).unwrap().to_owned();
        }
    }

    let mut card_count = [0; 14];
    let mut order: Vec<usize> = Vec::new();
    for c in hand.chars() {
        card_count[to_numeric(c) - 1] += 1;
        order.push(to_numeric(c));
    }

    let jokers = card_count[to_numeric('J') - 1];
    for i in 0..card_count.len() {
        if i != to_numeric('J') - 1 {
            card_count[i] += jokers;
        }
    }
    card_count.sort();
    let best = match (card_count[13], card_count[12] - jokers) {
        (5, _) => HandKind::FiveKind,
        (4, _) => HandKind::FourKind,
        (3, 2) => HandKind::FullHouse,
        (3, x) if x < 2 => HandKind::ThreeKind,
        (2, 2) => HandKind::TwoPair,
        (2, x) if x < 2 => HandKind::OnePair,
        (1, x) if x <= 1 => HandKind::HighCard,
        (_, _) => panic!("Strange number of cards in hand"),
    };

    return (best, order);
}

fn read_input() -> Vec<HandData> {
    let content = fs::read_to_string("input.txt").expect("should have read the file");
    let binding = content.replace(".", " ");
    let lines = binding.split("\n").collect::<Vec<_>>();

    let mut hands: Vec<HandData> = Vec::new();
    for line in lines {
        let (order, bid) = line.split_once(" ").unwrap();
        let hand = read_hand(order.to_string());
        hands.push(HandData { raw_value: line.to_string(), kind: hand.0, ordered_cards: hand.1, bid: bid.to_string().parse::<usize>().unwrap() }
        );
    }

    hands.sort_by(|this, that| {
        if this.kind.compare(&that.kind) == Ordering::Equal {
            for i in 0..that.ordered_cards.len() {
                if this.ordered_cards[i].cmp(&that.ordered_cards[i]) == Ordering::Equal {
                    continue;
                }
                return this.ordered_cards[i].cmp(&that.ordered_cards[i]);
            }
            println!(
                "Read following vectors as equal: {:?} \t {:?} \t \t {} \t {} ",
                this.ordered_cards, that.ordered_cards, this.raw_value, that.raw_value
            );
            return Ordering::Equal;
        } else {
            return this.kind.compare(&that.kind);
        }
    });

    return hands;
}

fn main() {
    let hands = read_input();
    let mut total = 0;
    for i in 0..hands.len() {
        total += (i + 1) * hands.get(i).unwrap().bid;
    }
    println!("{}", total);
}
