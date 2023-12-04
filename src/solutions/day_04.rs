use std::{cmp::min, collections::HashSet};

pub fn solve(input: &str) -> String {
    format!("{}\n{}\n", part_one(input), part_two(input))
}

pub fn get_winning_count(values: &str) -> u32 {
    let (winning, my_numbers) = values.split_once('|').unwrap();

    let winning_numbers = winning
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();

    my_numbers
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .filter(|n| winning_numbers.contains(n))
        .count() as u32
}

pub fn part_one(input: &str) -> u32 {
    let base: i32 = 2;

    input
        .lines()
        .map(|s| s.split_once(':').unwrap().1)
        .map(|s| get_winning_count(s))
        .filter(|value| *value != 0)
        .map(|value| base.pow(value - 1) as u32)
        .sum::<u32>()
}

pub fn part_two(input: &str) -> u32 {
    let len = input.lines().count();
    let mut card_counts: Vec<u32> = vec![1; len];

    for (card, winning_count) in input
        .lines()
        .map(|s| s.split_once(':').unwrap().1)
        .map(|s| get_winning_count(s) as usize)
        .enumerate()
    {
        for i in card + 1..min(card + winning_count + 1, len) {
            card_counts[i] += card_counts[card];
        }
    }

    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "\
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part_one(input).to_string(), "13");
    }

    #[test]
    fn test_part_2_example() {
        let input = "\
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part_two(input).to_string(), "30");
    }
}
