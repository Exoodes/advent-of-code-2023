pub fn solve(input: &str) -> String {
    format!("{}\n{}\n", part_one(input), part_two(input))
}

pub fn part_one(input: &str) -> i64 {
    solver(input, true)
}

pub fn part_two(input: &str) -> i64 {
    solver(input, false)
}

fn solver(input: &str, part: bool) -> i64 {
    input
        .lines()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|l| compute_lines(l.clone()))
        .map(|l| {
            if part {
                compute_first_part(l)
            } else {
                compute_second_part(l)
            }
        })
        .sum::<i64>()
}

fn compute_first_part(values: Vec<Vec<i64>>) -> i64 {
    values.iter().map(|l| l.last().unwrap()).sum::<i64>()
}

fn compute_second_part(values: Vec<Vec<i64>>) -> i64 {
    let mut out = 0;

    for l in values.iter().rev() {
        out = l[0] - out;
    }

    out
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
