advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let width = input.lines().next().unwrap().chars().count();
    let mut beam = vec![0_u64; width];
    let mut total: u64 = 0;
    for line in input.trim().lines() {
        let mut next_beam = beam.clone();
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => next_beam[j] = 1,
                '^' => {
                    if beam[j] > 0 {
                        next_beam[j - 1] = 1;
                        next_beam[j + 1] = 1;
                        next_beam[j] = 0;

                        total += 1;
                    }
                }
                _ => (),
            }
        }
        beam = next_beam;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    // This part requires the input to be parsed in a different way.
    let width = input.lines().next().unwrap().chars().count();
    let mut beam = vec![0_u64; width];
    for line in input.trim().lines() {
        let mut next_beam = beam.clone();
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => next_beam[j] = 1,
                '^' => {
                    if beam[j] > 0 {
                        next_beam[j - 1] += beam[j];
                        next_beam[j + 1] += beam[j];
                        next_beam[j] = 0;
                    }
                }
                _ => (),
            }
        }
        beam = next_beam;
    }
    Some(beam.iter().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
