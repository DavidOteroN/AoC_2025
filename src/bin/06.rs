advent_of_code::solution!(6);

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    // The data is arranged in columns.
    // The last column contains tha operations (`+`|`*`). The rest contain numbers.
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let mut nums: Vec<Vec<u64>> = Vec::new();
    let ops = lines[lines.len() - 1]
        .chars()
        .filter(|&c| c != ' ')
        .collect::<Vec<char>>();
    for line in lines[0..lines.len() - 1].iter() {
        nums.push(
            line.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );
    }

    (nums, ops)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (nums, ops) = parse_input(input);

    let mut total: u64 = 0;
    for i in 0..ops.len() {
        total += match ops[i] {
            '+' => nums.iter().map(|v| v[i]).sum::<u64>(),
            '*' => nums.iter().map(|v| v[i]).product::<u64>(),
            _ => 0,
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    // In this part, the numbers are arranged vertically with the most significant bit on the first
    // row.
    // For example:
    //
    // 14
    // 25
    // 3
    // +
    //
    // would mean 123 + 45.
    let lines: Vec<_> = input.trim().lines().collect();

    // Transpose the input:
    let columns: Vec<_> = (0..lines[0].len())
        .map(|i| {
            lines[0..lines.len() - 1]
                .iter()
                .map(|&line| line.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect();

    if cfg!(test) {
        for c in &columns {
            println!("{}", c);
        }
    }

    let ops: Vec<_> = lines[lines.len() - 1]
        .chars()
        .filter(|&c| c != ' ')
        .collect();

    let mut ops_idx = 0;
    let mut queue: Vec<u64> = Vec::new();
    let mut total: u64 = 0;
    for col in columns.iter() {
        if col.trim().is_empty() {
            // Perform operatiion and flush queue.
            match ops[ops_idx] {
                '+' => total += queue.iter().sum::<u64>(),
                '*' => total += queue.iter().product::<u64>(),
                _ => panic!("Unexpected operation"),
            }
            queue.clear();
            ops_idx += 1;
        } else {
            // Add numbers to the queue.
            if cfg!(test) {
                println!("Parsing: {}", col);
            }
            queue.push(col.trim().parse::<u64>().unwrap());
        }
    }

    // NOTE: After the loop, there's one operation remaining.
    match ops[ops.len() - 1] {
        '+' => total += queue.iter().sum::<u64>(),
        '*' => total += queue.iter().product::<u64>(),
        _ => panic!("Unexpected operation"),
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
