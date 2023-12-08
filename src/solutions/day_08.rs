use std::collections::HashMap;

pub fn solve(input: &str) -> String {
    format!("{}\n{}\n", part_one(input), part_two(input))
}

pub fn part_one(input: &str) -> usize {
    solver(input, false)
}

pub fn part_two(input: &str) -> usize {
    solver(input, true)
}

fn solver(input: &str, ghost: bool) -> usize {
    let navigation_instruction = input.lines().next().unwrap();
    let map = parse_input(input);
    let starting_positions = get_starting_position(&map, ghost);
    starting_positions
        .iter()
        .map(|s| follow_path(s, navigation_instruction, &map, ghost) as i64)
        .fold(1, |a, b| num::integer::lcm(a, b)) as usize
}

fn parse_neighbour(line: &str) -> (&str, &str) {
    let mut line = line.trim().chars();

    line.next();
    line.next_back();

    let (l, r) = line.as_str().split_once(',').unwrap();
    (l.trim(), r.trim())
}

fn parse_input(input: &str) -> HashMap<&str, (&str, &str)> {
    input
        .lines()
        .skip(2)
        .map(|s| s.split_once('=').unwrap())
        .map(|(k, n)| (k.trim(), parse_neighbour(n)))
        .collect::<HashMap<_, _>>()
}

fn follow_path(
    node: &str,
    navigation: &str,
    map: &HashMap<&str, (&str, &str)>,
    ghost: bool,
) -> usize {
    let mut curr_node = node;
    let pattern = if ghost { "Z" } else { "ZZZ" };
    for (step, way) in navigation.chars().cycle().enumerate() {
        if curr_node.ends_with(pattern) {
            return step;
        }

        let direction = map.get(curr_node).unwrap();

        if way == 'L' {
            curr_node = direction.0;
        } else {
            curr_node = direction.1;
        }
    }

    0
}

fn get_starting_position<'a>(
    map: &'a HashMap<&'a str, (&'a str, &'a str)>,
    ghost: bool,
) -> Vec<&'a str> {
    if !ghost {
        return vec!["AAA"];
    }

    map.keys()
        .filter(|s| s.ends_with("A"))
        .map(|s| *s)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example_1() {
        let input = "\
                RL\n\
                \n\
                AAA = (BBB, CCC)\n\
                BBB = (DDD, EEE)\n\
                CCC = (ZZZ, GGG)\n\
                DDD = (DDD, DDD)\n\
                EEE = (EEE, EEE)\n\
                GGG = (GGG, GGG)\n\
                ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part_one(input).to_string(), "2");
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "\
                LLR\n\
                \n\
                AAA = (BBB, BBB)\n\
                BBB = (AAA, ZZZ)\n\
                ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part_one(input).to_string(), "6");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
                LR\n\
                \n\
                11A = (11B, XXX)\n\
                11B = (XXX, 11Z)\n\
                11Z = (11B, XXX)\n\
                22A = (22B, XXX)\n\
                22B = (22C, 22C)\n\
                22C = (22Z, 22Z)\n\
                22Z = (22B, 22B)\n\
                XXX = (XXX, XXX)";
        assert_eq!(part_two(input).to_string(), "6");
    }
}
