pub fn solve(input: &str) -> String {
    format!("{}\n{}\n", part_one(input), part_two(input))
}

fn parse_input(input: &str, combine_numbers: bool) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|s| s.split_once(':').unwrap().1)
        .map(|s| {
            if combine_numbers {
                return s.replace(" ", "");
            }

            s.to_string()
        })
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> u64 {
    solve_part(input, false)
}

pub fn part_two(input: &str) -> u64 {
    solve_part(input, true)
}

fn solve_part(input: &str, combine_numbers: bool) -> u64 {
    let numbers = parse_input(input, combine_numbers);
    let mut sum = 0;

    for i in 0..numbers[0].len() {
        let time = numbers[0][i];
        let distance = numbers[1][i];
        let mut ways_to_win = 0;

        for hold_time in 0..time + 1 {
            if (time - hold_time) * hold_time > distance {
                ways_to_win += 1;
            }
        }

        if sum == 0 {
            sum = ways_to_win;
        } else {
            sum *= ways_to_win;
        }
    }

    sum
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
