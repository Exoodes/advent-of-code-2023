use regex::Regex;

pub fn solve(input: &str) -> String {
    format!("{}\n{}\n", part_one(input), part_two(input))
}

pub fn part_one(input: &str) -> u32 {
    solver(input, false)
}

pub fn part_two(input: &str) -> u32 {
    solver(input, true)
}

pub fn solver(input: &str, count_as_word: bool) -> u32 {
    input
        .lines()
        .map(|l| {
            let numbers = if count_as_word {
                get_numbers(l)
            } else {
                l.chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>()
            };

            let first = numbers[0];
            let last = numbers.last().unwrap();

            first * 10 + last
        })
        .sum()
}

fn get_numbers(line: &str) -> Vec<u32> {
    let re = Regex::new(r"^(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let mut digits: Vec<u32> = Vec::new();

    for i in 0..line.len() {
        if let Some(m) = re.find(&line[i..]) {
            let digit = match m.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => m.as_str().parse::<u32>().unwrap(),
            };
            digits.push(digit);
        }
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "\
            1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet";

        assert_eq!(part_one(input).to_string(), "142");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            two1nine\n\
            eightwothree\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen";
        assert_eq!(part_two(input).to_string(), "281");
    }
}
