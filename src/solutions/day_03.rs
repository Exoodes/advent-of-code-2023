use std::collections::HashMap;

pub fn solve(input: &str) -> String {
    format!("{:?}\n{:?}\n", part_one(input), part_two(input))
}

fn check_symbol(lines: &Vec<&[u8]>, x: usize, y: usize) -> bool {
    let x = x as i32;
    let y = y as i32;

    let positions = vec![
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ];

    for (xi, yi) in positions {
        if xi >= 0 && yi >= 0 && xi < lines.len() as i32 && yi < lines[0].len() as i32 {
            let c = lines[xi as usize][yi as usize] as char;
            if !c.is_digit(10) && c != '.' {
                return true;
            }
        }
    }

    false
}

pub fn part_one(input: &str) -> u32 {
    let lines = input.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    let mut out = 0;

    let mut is_part_number = false;
    let mut curr_number = 0;

    for x in 0..lines.len() {
        for y in 0..lines[0].len() {
            let c = lines[x][y] as char;

            if c.is_digit(10) {
                curr_number = curr_number * 10 + c.to_digit(10).unwrap();
                is_part_number = is_part_number || check_symbol(&lines, x, y);
            } else {
                if is_part_number {
                    println!("{curr_number}");
                    out += curr_number;
                }
                curr_number = 0;
                is_part_number = false;
            }
        }
    }

    out
}

pub fn part_two(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "\
                467..114..\n\
                ...*......\n\
                ..35..633.\n\
                ......#...\n\
                617*......\n\
                .....+.58.\n\
                ..592.....\n\
                ......755.\n\
                ...$.*....\n\
                .664.598..";
        assert_eq!(part_one(input).to_string(), "4361");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "";
        assert_eq!(part_two(input).to_string(), "");
    }
}
