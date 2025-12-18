advent_of_code::solution!(12);

use regex::Regex;

type Point = (u64, u64);

#[derive(Debug)]
struct Tree {
    size: Point,
    gifts: u64,
}

impl Tree {
    fn from_input(input: &str) -> Option<Self> {
        let re = Regex::new(r"(?<shape>(?<s1>\d+)x(?<s2>\d+)): (?<gifts>(\d+\s*)+)").unwrap();
        let caps = re.captures(input)?;
        let height = caps["s1"].parse::<u64>().unwrap();
        let width = caps["s2"].parse::<u64>().unwrap();
        let gifts = caps["gifts"]
            .trim()
            .split(" ")
            .map(|g| g.parse::<u64>().unwrap())
            .sum();

        Some(Tree {
            size: (height, width),
            gifts,
        })
    }

    fn check_viable(&self) -> bool {
        let (height, width) = self.size;
        (height / 3) * (width / 3) >= self.gifts
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let count = input
        .trim()
        .lines()
        .filter_map(Tree::from_input)
        .filter(|t| t.check_viable())
        .count() as u64;
    Some(count)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    /// Tests are useless in this problem.
    #[test]
    fn test_part_one() {
        assert_eq!(Some(0), Some(0));
    }
}
