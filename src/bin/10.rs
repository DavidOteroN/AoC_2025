advent_of_code::solution!(10);

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    lights: u16,
    buttons: Vec<u16>,
    joltage: Vec<u16>,
}

enum GroupType {
    Lights,
    Buttons,
    Joltage,
}

impl Machine {
    /// Each line in the input has the following elements:
    /// - Lights: represented as \[[.#]+\]
    /// - Buttons: represented as \((\d,?)+\)+
    /// - Joltage: represented as \{(\d+,?)+\}
    ///
    /// Lights will be represented as a u64 bitmask (on or off). Buttons will be a vector of u64
    /// bitmasks, since toggling a light is an XOR operation.
    ///
    /// Looking at the real input, it seems there are at most 10 lights in a single machine, so all
    /// button numbers range from 0 to 9. This means that both lights and buttons can be u16 ints.
    fn from_str(input: &str) -> Machine {
        // NOTE: This could be done with Regex, but it's probably more efficient to just iterate
        // through the char array once and do it all in one go.
        let mut number: u16 = 0;
        let mut index: u16 = 0;

        let mut lights: u16 = 0;
        let mut button: u16 = 0;
        let mut joltage: Vec<u16> = Vec::new();
        let mut buttons: Vec<u16> = Vec::new();

        let mut group: GroupType = GroupType::Lights;

        for c in input.trim().chars() {
            match c {
                '[' => {
                    // Start processing lights.
                    group = GroupType::Lights;
                }
                '#' => {
                    lights |= 1 << index;
                    index += 1;
                }
                '.' => index += 1,
                ']' => {
                    // Terminate lights group processing.
                    // (no need to do anything here)
                }
                '(' => {
                    // Start processing button.
                    group = GroupType::Buttons;
                }
                ')' => {
                    // Terminate button group processing.
                    // Process last number.
                    button |= 1 << number;

                    buttons.push(button);

                    // Reset number and button.
                    number = 0;
                    button = 0;
                }
                '{' => {
                    // Start processing joltage.
                    group = GroupType::Joltage;
                }
                '}' => {
                    // Terminate joltage group processing.
                    // Process last number.
                    joltage.push(number);

                    // Reset number.
                    number = 0;
                }
                '0'..='9' => {
                    // Process numeric type.
                    number *= 10;
                    number += c.to_digit(10).unwrap() as u16;
                }
                ',' => {
                    // Terminate number processing.
                    match group {
                        GroupType::Buttons => button |= 1 << number,
                        GroupType::Joltage => joltage.push(number),
                        _ => (),
                    }

                    // Reset number:
                    number = 0;
                }
                _ => (),
            }
        }

        Machine {
            lights,
            buttons,
            joltage,
        }
    }

    fn _get_combinations(buttons: &[u16], target: &u16) -> impl Iterator<Item = u16> {
        // The goal in this part is to compute the minimum number of button presses to toggle the
        // lights from all off (0) to the specified configuration.
        // Each button press is a XOR operation (toggle bits).
        // The problem can be one in reverse: start with the pre-determined lights configuration and
        // toggle until 0 is reached.
        //
        // If B1 ... BN are the buttons, then find X1 ... XN such that:
        // L ^ (X1 * B1) ^ ... ^ (XN * BN) == 0,
        // Where L is the lights configuration and X1 ... XN are either 0 or 1,
        // and X1 + ... + XN is minimized.
        // NOTE: Even though a button can be pressed multiple times, pressing it an odd number of
        // times is equivalent to pressing just once, and pressing it an even number of times is
        // equivalent to not pressing it. Therefore, the optimal solution is pressing each button
        // either zero or one times.

        // Generate all possible combinations:
        // Since there are only a few buttons, and X1 ... XN can only be 0 or 1, the combinations
        // can be represented as a bitmask, with the maximum value being 1 ... 1 (as many ones as
        // buttons). Then, they can be sorted by the number of ones.

        let max = 2_u16.pow(buttons.len() as u32) - 1;

        // NOTE: There can be a special case in part 2, where
        // after halving the new target, all numbers are even, and therefore what we need to do is
        // halve is count 0 presses and halve it again.
        let mut combinations: Vec<_> = (0..=max).collect();
        combinations.sort_by_key(|a| a.count_ones());

        // After sorting, the combinations will be (example for 4 buttons):
        //  0001
        //  0010
        //  0100
        //  ...
        //  0011
        //  0101
        //  ...
        //  0111
        //  1011
        //  ...
        //  1111
        combinations.into_iter().filter(|&c| {
            let output = buttons
                .iter()
                .enumerate()
                .fold(*target, |acc, (i, &b)| acc ^ ((1 & (c >> i)) * b));
            output == 0
        })
    }

    fn _bitmask_to_idx_vec(bitmask: u16) -> Vec<usize> {
        (0..16)
            .filter(|&i| 1 & (bitmask >> i) != 0)
            .collect::<Vec<usize>>()
    }

    /// Computes the number of required button presses to achieve the required lights
    /// configuration.
    fn configure_lights(&self) -> u64 {
        // Get the first possible combination and return the number of button presses (ones)
        Self::_get_combinations(&self.buttons, &self.lights)
            .next()
            .unwrap()
            .count_ones() as u64
    }

    fn configure_joltage(&self) -> u64 {
        // Similar to configuring the lights, but now using the joltage as target. Each button
        // press increases the joltage counters matching the button indices by one. Each button can
        // be pressed multiple times.
        //
        // Now, the goal is solve an under-constrained system of linear equations, where now X1 ...
        // XN are positive integers.
        // NOTE: in the real input data, some joltages have values larger than the number of
        // buttons, so now the buttons WILL have to be pressed multiple times.
        //
        // The equaltions system can be written as follows (using Einstein's notation):
        //  Ji = Xj * Bij
        // Where Ji is the ith joltaje indicator, Xj is the number of presses for the jth button,
        // and Bij is the the ontribution to the ith joltage indicator of the jth button (0 or 1).
        //
        // Implemetation based on some of the concepts from this reddit thread:
        // https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
        let mut cache: HashMap<(Vec<u16>, Vec<u16>), Option<u64>> = HashMap::new();
        Self::_configure_joltage_inner(&self.buttons.clone(), &self.joltage.clone(), &mut cache)
            .unwrap()
    }

    /// Static method. This is here only for namespacing purposes.
    fn _configure_joltage_inner(
        buttons: &[u16],
        target: &[u16],
        cache: &mut HashMap<(Vec<u16>, Vec<u16>), Option<u64>>,
    ) -> Option<u64> {
        // base case:
        if target.iter().all(|t| *t == 0) {
            // If target is all 0, then we're done!
            return Some(0);
        }

        // Return from cache if available:
        if let Some(result) = cache.get(&(Vec::from(buttons), Vec::from(target))) {
            return *result;
        }

        // Convert the joltage vector to a bitmask (mod 2), and solve using part one logic.
        let lights = target
            .iter()
            .enumerate()
            .fold(0_u16, |acc, (i, t)| acc | ((*t % 2) << i));

        // Get all possible patterns to reach the desired "lights" state:
        let mut count: Option<u64> = None;
        'outer: for p in Self::_get_combinations(buttons, &lights) {
            // Convert the pattern to button indices and compute new target:
            let mut new_target = Vec::from(target); // implicit copy
            for button_idx in Self::_bitmask_to_idx_vec(p) {
                let button = Self::_bitmask_to_idx_vec(buttons[button_idx]);
                for idx in button {
                    if new_target[idx] == 0 {
                        continue 'outer;
                    }
                    new_target[idx] -= 1;
                }
            }
            // Halve the new target.
            new_target.iter_mut().for_each(|t| *t /= 2);

            let initial_presses = p.count_ones() as u64;
            let half_presses = Self::_configure_joltage_inner(buttons, &new_target, cache);
            if half_presses.is_none() {
                continue 'outer;
            }
            if let Some(c) = count {
                // `count` is not None.
                count = Some(c.min(initial_presses + 2 * half_presses.unwrap()));
            } else {
                count = Some(initial_presses + 2 * half_presses.unwrap());
            }
        }
        count
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Lights:\n")?;
        f.write_fmt(format_args!("\t{:016b}\n", self.lights))?;
        f.write_str("Buttons:\n")?;
        for b in self.buttons.clone() {
            f.write_fmt(format_args!(
                "\t{:016b} | {:?}\n",
                b,
                Self::_bitmask_to_idx_vec(b)
            ))?;
        }
        f.write_str("Joltage:\n")?;
        f.write_str("\t")?;
        for j in self.joltage.clone() {
            f.write_fmt(format_args!("{} ", j))?;
        }

        Ok(())
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    let n = input.trim().lines().count();

    // NOTE: pre-allocate with capacity for performance.
    let mut machines: Vec<Machine> = Vec::with_capacity(n);
    for line in input.lines() {
        machines.push(Machine::from_str(line));
    }

    machines
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);

    let total = machines.iter().map(|m| m.configure_lights()).sum::<u64>();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    for line in input.trim().lines() {
        total += Machine::from_str(line).configure_joltage();
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
