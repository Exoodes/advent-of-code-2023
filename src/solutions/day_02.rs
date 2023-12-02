pub fn solve(input: &str) -> String {
    format!("{}\n{}\n", part_one(input), part_two(input))
}

struct Cubes {
    red: u32,
    blue: u32,
    green: u32,
}

struct Game {
    id: u32,
    result: Vec<Cubes>,
}

impl Game {
    fn is_ok(&self) -> bool {
        for r in &self.result {
            if r.red > 12 || r.green > 13 || r.blue > 14 {
                return false;
            }
        }

        true
    }

    fn power_sets(&self) -> u32 {
        let mut minimum = Cubes {
            red: 0,
            blue: 0,
            green: 0,
        };

        for r in &self.result {
            if r.red > minimum.red {
                minimum.red = r.red;
            }
            if r.blue > minimum.blue {
                minimum.blue = r.blue;
            }
            if r.green > minimum.green {
                minimum.green = r.green;
            }
        }

        minimum.red * minimum.blue * minimum.green
    }
}

fn parse_id(line: &str) -> u32 {
    line.split(' ').last().unwrap().parse().unwrap()
}

fn parse_cubes(line: &str) -> Cubes {
    let mut out = Cubes {
        red: 0,
        blue: 0,
        green: 0,
    };

    line.trim().split(',').for_each(|s| {
        let x = s.trim().split(' ').collect::<Vec<&str>>();
        let value: u32 = x[0].parse().unwrap();

        match x[1] {
            "red" => out.red = value,
            "green" => out.green = value,
            "blue" => out.blue = value,
            _ => {}
        }
    });

    out
}

fn parse_game(line: &str) -> Game {
    let parts = line.split(':').collect::<Vec<&str>>();

    Game {
        id: parse_id(parts[0]),
        result: parts[1]
            .trim()
            .split(';')
            .map(|s| parse_cubes(s))
            .collect::<Vec<Cubes>>(),
    }
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| parse_game(l))
        .filter(|g| g.is_ok())
        .map(|g| g.id)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|l| parse_game(l))
        .map(|g| g.power_sets())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "\
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_one(input).to_string(), "8");
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "\
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_two(input).to_string(), "2286");
    }
}
