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
    map.insert('J', 4);
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

fn hand_type_from_letter_count(map: &HashMap<char, usize>) -> HandType {
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

fn get_letter_count(hand: &str) -> HashMap<char, usize> {
    hand.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    })
}

fn generate_strings(input: String) -> Vec<String> {
    let possible_letters = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    let mut result = Vec::new();
    let mut current_string = String::new();

    generate_strings_recursive(
        &input,
        &possible_letters,
        0,
        &mut current_string,
        &mut result,
    );

    result
}

fn generate_strings_recursive(
    input: &str,
    possible_letters: &[char],
    index: usize,
    current_string: &mut String,
    result: &mut Vec<String>,
) {
    if index == input.len() {
        result.push(current_string.clone());
        return;
    }

    if input.chars().nth(index) == Some('J') {
        for &letter in possible_letters.iter() {
            current_string.push(letter);
            generate_strings_recursive(input, possible_letters, index + 1, current_string, result);
            current_string.pop();
        }
    } else {
        current_string.push(input.chars().nth(index).unwrap());
        generate_strings_recursive(input, possible_letters, index + 1, current_string, result);
        current_string.pop();
    }
}

fn get_hand_type(hand: &str, joker: bool) -> HandType {
    let mut all_hands = vec![hand.to_string()];
    if joker {
        all_hands = generate_strings(hand.to_string());
    }

    all_hands
        .iter()
        .map(|hand| get_letter_count(hand))
        .map(|m| hand_type_from_letter_count(&m))
        .sorted()
        .next()
        .unwrap()
}

pub fn part_one(input: &str) -> usize {
    solver(input, false)
}

pub fn part_two(input: &str) -> usize {
    solver(input, true)
}

fn solver(input: &str, joker: bool) -> usize {
    input
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<_>>())
        .map(|s| Hand {
            hand_type: get_hand_type(s[0], joker),
            cards: s[0],
            value: s[1].parse::<usize>().unwrap(),
        })
        .sorted()
        .rev()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.value)
        .sum()
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
