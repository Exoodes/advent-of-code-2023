use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

pub fn solve(input: &str) -> String {
    format!("{}\n{}\n", part_one(input), part_two(input))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand<'a> {
    hand_type: HandType,
    cards: &'a str,
    value: usize,
}

fn get_card_order(a: char, b: char) -> std::cmp::Ordering {
    let mut map = HashMap::new();
    map.insert('A', 1);
    map.insert('K', 2);
    map.insert('Q', 3);
    map.insert('J', 40);
    map.insert('T', 5);
    map.insert('9', 6);
    map.insert('8', 7);
    map.insert('7', 8);
    map.insert('6', 9);
    map.insert('5', 10);
    map.insert('4', 11);
    map.insert('3', 12);
    map.insert('2', 13);

    map.get(&a).cmp(&map.get(&b))
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Eq for Hand<'_> {}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut out = self.hand_type.cmp(&other.hand_type);

        if out == Ordering::Equal {
            for i in 0..5 as usize {
                if self.cards.as_bytes()[i] != other.cards.as_bytes()[i] {
                    out = get_card_order(
                        self.cards.as_bytes()[i].into(),
                        other.cards.as_bytes()[i].into(),
                    );
                    break;
                }
            }
        }

        out
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn hand_type_from_letter_count(map: &HashMap<char, i32>) -> HandType {
    match map.len() {
        1 => HandType::FiveOfKind,
        2 => {
            if map.values().any(|c| *c == 4) {
                return HandType::FourOfKind;
            } else {
                return HandType::FullHouse;
            }
        }
        3 => {
            if map.values().any(|c| *c == 3) {
                return HandType::ThreeOfKind;
            } else {
                return HandType::TwoPair;
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!(),
    }
}

fn get_hand_type(hand: &str) -> HandType {
    let letter_counts: HashMap<char, i32> = hand.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    hand_type_from_letter_count(&letter_counts)
}

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<_>>())
        .map(|s| Hand {
            hand_type: get_hand_type(s[0]),
            cards: s[0],
            value: s[1].parse::<usize>().unwrap(),
        })
        .sorted()
        .rev()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.value)
        .sum()
}

pub fn part_two(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "";
        assert_eq!(part_one(input).to_string(), "");
    }

    #[test]
    fn test_part_2_example() {
        let input = "";
        assert_eq!(part_two(input).to_string(), "");
    }
}
