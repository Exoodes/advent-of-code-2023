use std::collections::HashSet;

pub fn solve(input: &str) -> String {
    format!("{}\n{}\n", part_one(input), part_two(input))
}

pub fn part_one(input: &str) -> usize {
    solver(input, 2)
}

pub fn part_two(input: &str) -> usize {
    solver(input, 1000000)
}

pub fn solver(input: &str, empty_multiplayer: usize) -> usize {
    let (max_x, max_y): (usize, usize) =
        (input.lines().count(), input.lines().next().unwrap().len());

    let mut galaxy_coordinates = Vec::new();

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                galaxy_coordinates.push((x, y));
            }
        }
    }

    let expanded_x = generate_expand_coordinates(&galaxy_coordinates, max_x, true);
    let expanded_y = generate_expand_coordinates(&galaxy_coordinates, max_y, false);

    let mut out = 0;
    for coord_1 in &galaxy_coordinates {
        for coord_2 in &galaxy_coordinates {
            out += compute_distance(coord_1.0, coord_2.0, &expanded_x, empty_multiplayer);
            out += compute_distance(coord_1.1, coord_2.1, &expanded_y, empty_multiplayer);
        }
    }

    out = out / 2;
    out
}

fn compute_distance(
    from: usize,
    to: usize,
    expanded_indices: &Vec<usize>,
    empty_multiplayer: usize,
) -> usize {
    let iter = if from > to { to..from } else { from..to };

    let tmp = iter
        .map(|value| {
            if expanded_indices.contains(&value) {
                empty_multiplayer
            } else {
                1
            }
        })
        .sum::<usize>();
    tmp
}

fn generate_expand_coordinates(
    galaxy_coordinates: &Vec<(usize, usize)>,
    max: usize,
    main_axis: bool,
) -> Vec<usize> {
    let galaxy = galaxy_coordinates
        .iter()
        .map(|value| {
            if main_axis {
                value.clone().0
            } else {
                value.clone().1
            }
        })
        .collect::<HashSet<_>>();

    let all_numbers = HashSet::from_iter(0..max);
    all_numbers.difference(&galaxy).cloned().collect::<Vec<_>>()
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
