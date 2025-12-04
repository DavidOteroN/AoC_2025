advent_of_code::solution!(4);

use std::collections::HashSet;

fn parse_input(input: &str) -> HashSet<(usize, usize)> {
    // The grid can be represented as a HashSet storing the coordinates.
    let mut rolls = HashSet::new();
    for (i, line) in input.trim().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '@' {
                rolls.insert((i, j));
            }
        }
    }
    rolls
}

pub fn part_one(input: &str) -> Option<u64> {
    let rolls = parse_input(input);

    let mut total: u64 = 0;
    for (x, y) in &rolls {
        let c = (-1..=1)
            .flat_map(|i| (-1..=1).map(move |j| (i, j)))
            .filter_map(|(i, j)| {
                let new_x = x.wrapping_add_signed(i);
                let new_y = y.wrapping_add_signed(j);
                rolls.get(&(new_x, new_y))
            })
            .count();
        // NOTE: The count will include the current coordinates and the 8 adjacent places.
        if c <= 4 {
            total += 1;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rolls = parse_input(input);

    let mut total: u64 = 0;
    let mut removed: bool = true;
    while removed {
        removed = false;
        for (x, y) in rolls.iter().copied().collect::<Vec<_>>() {
            let c = (-1..=1)
                .flat_map(|i| (-1..=1).map(move |j| (i, j)))
                .filter_map(|(i, j)| {
                    let new_x = x.wrapping_add_signed(i);
                    let new_y = y.wrapping_add_signed(j);
                    rolls.get(&(new_x, new_y))
                })
                .count();
            // NOTE: The count will include the current coordinates and the 8 adjacent places.
            if c <= 4 {
                removed = true;
                rolls.remove(&(x, y));
                total += 1;
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13_u64));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43_u64));
    }
}
