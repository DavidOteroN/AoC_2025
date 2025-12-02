advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    // The goal is to check the numbers that are made of two repeating sequences of digits (i.e.:
    // 123123, 55, 9898, etc)
    let mut total: u64 = 0;
    for range in input.trim().split(",") {
        if let [first, last] = range.split("-").collect::<Vec<&str>>()[..] {
            let first = first.parse::<u64>().unwrap();
            let last = last.parse::<u64>().unwrap();

            for num in first..=last {
                let num_str = num.to_string();
                if num_str.len() % 2 != 0 {
                    continue;
                }
                if num_str[0..num_str.len() / 2] == num_str[num_str.len() / 2..num_str.len()] {
                    total += num;
                }
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Now, we have to detect any repeating sequence of digits (at least twice)
    // Examples: 121212, 999, 123123.
    let mut total: u64 = 0;
    for range in input.trim().split(",") {
        if let [first, last] = range.split("-").collect::<Vec<&str>>()[..] {
            let first = first.parse::<u64>().unwrap();
            let last = last.parse::<u64>().unwrap();

            for num in first..=last {
                let num_str = num.to_string();

                // Do something here to detect repeating patterns.
                let length = num_str.len();
                for end in 1..=length / 2 {
                    let chunk = &num_str[0..end];
                    if length % end != 0 {
                        // Can't construct a number of the same length using this chink.
                        continue;
                    }
                    let repeating = chunk.repeat(length / (end));
                    if num_str == repeating {
                        total += num;
                        break;
                    }
                }
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
