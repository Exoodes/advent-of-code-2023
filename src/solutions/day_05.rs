pub fn solve(input: &str) -> String {
    format!("{}\n{}\n", part_one(input), part_two(input))
}

#[derive(Debug, Clone)]
struct MapPart {
    dest: i64,
    source: i64,
    len: i64,
}

#[derive(Debug, Clone)]
struct Map(Vec<MapPart>);

fn parse_input(input: &str) -> (Vec<i64>, Vec<Map>) {
    let lines = input.lines().collect::<Vec<_>>();
    let seeds = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut maps: Vec<Map> = Vec::new();
    let mut curr_map = Vec::new();
    for l in &lines[3..] {
        if l.is_empty() {
            continue;
        }

        let first = l.chars().next().unwrap();
        if first.is_digit(10) {
            let x = l
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            curr_map.push(MapPart {
                dest: x[0],
                source: x[1],
                len: x[2],
            });
        } else {
            maps.push(Map(curr_map));
            curr_map = Vec::new();
        }
    }

    maps.push(Map(curr_map));

    (seeds, maps)
}

fn map_number(map: &Map, n: &mut i64) {
    for mapping in &map.0 {
        if mapping.dest <= *n && *n < (mapping.dest + mapping.len) {
            *n = *n + (mapping.source - mapping.dest);
            break;
        }
    }
}

pub fn part_one(input: &str) -> i64 {
    let (seeds, maps) = parse_input(input);

    for i in 0..i64::MAX {
        let mut n = i;
        for map in maps.iter().rev() {
            map_number(map, &mut n);
        }

        if seeds.contains(&n) {
            return i;
        }
    }

    -1
}

pub fn part_two(input: &str) -> i64 {
    let (seeds, maps) = parse_input(input);

    for i in 0..i64::MAX {
        let mut n = i;
        for map in maps.iter().rev() {
            map_number(map, &mut n);
        }

        for range in seeds.chunks(2) {
            if range[0] <= n && n < (range[0] + range[1]) {
                return i;
            }
        }
    }

    -1
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
