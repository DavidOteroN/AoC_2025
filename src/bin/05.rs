advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut split = input.split("\n\n");
    let ranges = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut s = line.split("-");
            let first = s.next().unwrap().parse::<u64>().unwrap();
            let last = s.next().unwrap().parse::<u64>().unwrap();
            (first, last)
        })
        .collect();
    let ids = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    (ranges, ids)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse_input(input);
    let mut counter: u64 = 0;
    for &id in &ids {
        for &(start, end) in &ranges {
            if id >= start && id <= end {
                counter += 1;
                break;
            }
        }
    }
    Some(counter)
}

#[allow(clippy::needless_range_loop)]
pub fn part_two(input: &str) -> Option<u64> {
    let (mut ranges, _) = parse_input(input);

    // NOTE: Sorting is essential for the next part of the algorythm!
    ranges.sort_by(|(start_1, _), (start_2, _)| start_1.cmp(start_2));

    // Iterate through the ranges to check for overlaps.
    let mut counter: u64 = 0;
    let mut used: Vec<usize> = Vec::new();
    for i in 0..ranges.len() {
        if used.contains(&i) {
            continue;
        }
        let &(mut start, mut end) = &ranges[i];
        for j in i + 1..ranges.len() {
            let &(start_2, end_2) = &ranges[j];
            if start <= end_2 && start_2 <= end {
                // Overlap detected. Expand range.
                start = start.min(start_2);
                end = end.max(end_2);
                used.push(j);
            }
        }
        counter += end - start + 1;
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
        assert_eq!(result, Some(14));
    }
}
