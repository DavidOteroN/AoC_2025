advent_of_code::solution!(3);

fn process_line(line: &str, n_digits: u8) -> u64 {
    // Iteratively find the largest digit in the range determined by the previously found largest
    // digit and the remaining number of digits.
    let mut num: u64 = 0;
    let mut idx: usize = 0;

    for remaining in (0..n_digits).rev() {
        // Find the largest digit in the range idx..len - remaining.
        let mut best: u8 = 0;
        for (i, c) in line[idx..line.len() - remaining as usize]
            .chars()
            .enumerate()
        {
            let n = c.to_digit(10).unwrap() as u8;
            if n > best {
                best = n;
                idx = i;
            }
        }
        num += 10_u64.pow(remaining as u32);
    }

    num
}

pub fn part_one(input: &str) -> Option<u64> {
    let r = input
        .trim()
        .lines()
        .map(|line| process_line(line, 2))
        .sum::<u64>();
    Some(r)
}

pub fn part_two(input: &str) -> Option<u64> {
    let r = input
        .trim()
        .lines()
        .map(|line| process_line(line, 12))
        .sum::<u64>();
    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
