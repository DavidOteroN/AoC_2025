advent_of_code::solution!(8);

use std::collections::HashMap;

type Point = (u64, u64, u64);

fn square_dist(p1: &Point, p2: &Point) -> u64 {
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;

    let dx = x1.abs_diff(*x2);
    let dy = y1.abs_diff(*y2);
    let dz = z1.abs_diff(*z2);

    dx * dx + dy * dy + dz * dz
}

fn parse_input(input: &str) -> Vec<Point> {
    let mut output: Vec<_> = Vec::new();
    for line in input.trim().lines() {
        if let [x, y, z] = line.split(",").collect::<Vec<&str>>()[..] {
            output.push((
                x.parse::<u64>().unwrap(),
                y.parse::<u64>().unwrap(),
                z.parse::<u64>().unwrap(),
            ));
        }
    }
    output
}

#[allow(clippy::needless_range_loop)]
pub fn part_one(input: &str) -> Option<u64> {
    const CONNECTIONS: usize = if cfg!(test) { 10 } else { 1000 };
    let points = parse_input(input);

    // Compute all distances:
    let mut distances: Vec<_> = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for p2 in points[i + 1..].iter() {
            let dist = square_dist(p1, p2);
            distances.push((*p1, *p2, dist));
        }
    }

    // Sort by distance:
    distances.sort_by(|(_, _, d1), (_, _, d2)| d1.cmp(d2));

    // Create coneections:
    // - If any point in the pair is already in a group, add the other point to the group.
    // - If none of the points in the pair ar in a group, then add them to a new group.
    let mut circuits: HashMap<Point, u64> = HashMap::new();
    let mut id_counter: u64 = 0;
    for (p1, p2, _) in distances.iter().take(CONNECTIONS) {
        let p1_id = circuits.get(p1).cloned();
        let p2_id = circuits.get(p2).cloned();

        match (p1_id, p2_id) {
            (Some(id), None) => {
                let _ = circuits.insert(*p2, id);
            }
            (None, Some(id)) => {
                let _ = circuits.insert(*p1, id);
            }
            (None, None) => {
                circuits.insert(*p1, id_counter);
                circuits.insert(*p2, id_counter);
                id_counter += 1;
            }
            (Some(id_1), Some(id_2)) => {
                for (_, id) in circuits.iter_mut() {
                    if *id == id_2 {
                        *id = id_1;
                    }
                }
            }
        }
    }

    if cfg!(test) {
        circuits
            .iter()
            .for_each(|(p, id)| println!("{:?} : {}", p, id));
    }

    // Count ids:
    let mut ids: HashMap<u64, u64> = HashMap::new();
    for (_, id) in circuits {
        ids.entry(id).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut ids: Vec<u64> = ids.iter().map(|(_, &id)| id).collect();
    ids.sort();
    let result = ids.iter().rev().take(3).product::<u64>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Similar to part 1, but keep connecting boxes untill all boxes are in a single circuit.
    // Output the product of the x coordinates of the last boxes to be connected.
    // NOTE: the stop condition for the iteration in this case is that the ammount of points added
    // to the `circuits` HashMap matches the length of the input vector.
    let points = parse_input(input);

    // Compute all distances:
    let mut distances: Vec<_> = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for p2 in points[i + 1..].iter() {
            let dist = square_dist(p1, p2);
            distances.push((*p1, *p2, dist));
        }
    }

    // Sort by distance:
    distances.sort_by(|(_, _, d1), (_, _, d2)| d1.cmp(d2));

    // Create coneections:
    // - If any point in the pair is already in a group, add the other point to the group.
    // - If none of the points in the pair ar in a group, then add them to a new group.
    let mut circuits: HashMap<Point, u64> = HashMap::new();
    let mut id_counter: u64 = 0;
    let mut result: u64 = 0;
    let mut removed_ids: u64 = 0;
    for (p1, p2, _) in &distances {
        let p1_id = circuits.get(p1).cloned();
        let p2_id = circuits.get(p2).cloned();

        match (p1_id, p2_id) {
            (Some(id), None) => {
                let _ = circuits.insert(*p2, id);
            }
            (None, Some(id)) => {
                let _ = circuits.insert(*p1, id);
            }
            (None, None) => {
                circuits.insert(*p1, id_counter);
                circuits.insert(*p2, id_counter);
                id_counter += 1;
            }
            (Some(id_1), Some(id_2)) => {
                if id_1 == id_2 {
                    continue;
                } else {
                    removed_ids += 1;
                    for (_, id) in circuits.iter_mut() {
                        if *id == id_2 {
                            *id = id_1;
                        }
                    }
                }
            }
        }
        result = p1.0 * p2.0;

        // Stop condition: all ids in the circuits HashMap are the same.
        if removed_ids == id_counter {
            break;
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
