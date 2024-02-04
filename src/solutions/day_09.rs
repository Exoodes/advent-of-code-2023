pub fn solve(input: &str) -> String {
    format!("{}\n{}\n", part_one(input), part_two(input))
}

pub fn part_one(input: &str) -> i64 {
    input
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|l| compute_lines(l.clone()))
        .map(|l| l.iter().map(|l| l.last().unwrap()).sum::<i64>())
        .sum::<i64>()
}


pub fn part_two(_input: &str) -> i64 {
    0
}

fn compute_lines(initial_values: Vec<i64>) -> Vec<Vec<i64>> {
    let mut lines = Vec::new();
    lines.push(initial_values);

    loop {
        let last_row = lines.last().unwrap();
        let mut new_row = Vec::new();

        for i in 1..last_row.len() {
            let value = last_row[i] - last_row[i - 1];
            new_row.push(value);
        }

        if new_row.iter().all(|n| *n == 0) {
            break;
        }

        lines.push(new_row);
    }

    lines
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
    fn test_part_2_example_1() {
        let input = "";
        assert_eq!(part_two(input).to_string(), "");
    }
}
