advent_of_code::solution!(1);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"([RL])(\d+)").unwrap();
    let mut dial: u64 = 50;
    let mut counter: u64 = 0;
    for (_, [dir, dist]) in re.captures_iter(input).map(|c| c.extract()) {
        let dist = dist.parse::<u64>().unwrap();
        match dir {
            "L" => {
                dial += 100;
                dial -= dist % 100;
                dial %= 100;
            }
            "R" => {
                dial += dist;
                dial %= 100;
            }
            _ => (),
        }
        if dial == 0 {
            counter += 1;
        }
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    // In this part, we have to check all the times there's a 0 crossing (on top of counting every
    // time that the dial ends up at 0).
    // NOTE: distances can be larger than 100.
    // NOTE: There can be multiple crossings in one rotation!
    let re = Regex::new(r"([RL])(\d+)").unwrap();
    let mut dial: u64 = 50;
    let mut counter: u64 = 0;
    for (_, [dir, dist]) in re.captures_iter(input).map(|c| c.extract()) {
        let dist = dist.parse::<u64>().unwrap();
        match dir {
            "L" => {
                dial += 100;
                dial -= dist % 100;
                dial %= 100;
                counter += (dial + dist) / 100;
                if (dial + dist).is_multiple_of(100) {
                    counter -= 1;
                }
            }
            "R" => {
                dial += dist;
                counter += dial / 100;
                dial %= 100;
                if dist > 0 && dial == 0 {
                    // If counter ends up at 0, we've already counted the crossing, so subtract one
                    // to aoid double-counting later.
                    counter -= 1;
                }
            }
            _ => (),
        }
        if dial == 0 {
            // Count times dial ends up at 0.
            counter += 1
        }
    }
    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
