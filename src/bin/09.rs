use std::collections::{HashMap, HashSet};

advent_of_code::solution!(9);

type Point = (u64, u64);

#[inline]
fn parse_input(input: &str) -> Vec<Point> {
    let mut output: Vec<_> = Vec::new();
    for line in input.trim().lines() {
        if let [x, y] = line.split(",").collect::<Vec<&str>>()[..] {
            output.push((x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()));
        }
    }
    output
}

#[derive(Clone, Debug)]
struct Mapping {
    _mapping_x: HashMap<u64, u64>,
    _mapping_y: HashMap<u64, u64>,
    max: Point,
}

impl Mapping {
    /// Get a list of (x, y) coordinates and creates a mapping to the coordinates in the compressed
    /// grid.
    pub fn from(points: &[Point]) -> Self {
        let mut xs: Vec<_> = points.iter().map(|(x, _)| *x).collect();
        let mut ys: Vec<_> = points.iter().map(|(_, y)| *y).collect();

        // Append a row and column before and after the data, so that there's a perimeter around
        // it.
        // NOTE: This is done so that later on, when running a flood fill algorythm, compressed
        // point (0,0) is ensured to be outside the boundary, and there's a continuous path aound
        // the boundary so that all outside points are correctly tagged.
        xs.push(xs.iter().min().unwrap() - 1);
        xs.push(xs.iter().max().unwrap() + 1);
        ys.push(ys.iter().min().unwrap() - 1);
        ys.push(ys.iter().max().unwrap() + 1);

        // Sort and remove duplicates.
        xs.sort();
        ys.sort();
        xs.dedup();
        ys.dedup();

        // Create mappings / lookuup tables.
        let map_x: HashMap<u64, u64> = xs
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, 2 * i as u64))
            .collect();
        let map_y: HashMap<u64, u64> = ys
            .into_iter()
            .enumerate()
            .map(|(j, y)| (y, 2 * j as u64))
            .collect();

        let max_x = map_x.iter().map(|(_, &v)| v).max().unwrap();
        let max_y = map_y.iter().map(|(_, &v)| v).max().unwrap();

        Self {
            _mapping_x: map_x,
            _mapping_y: map_y,
            max: (max_x, max_y),
        }
    }

    // fn inverse(&self) -> Self {
    //     let map_x: HashMap<u64, u64> = self._mapping_x.iter().map(|(&k, &v)| (v, k)).collect();
    //     let map_y: HashMap<u64, u64> = self._mapping_y.iter().map(|(&k, &v)| (v, k)).collect();

    //     Self {
    //         _mapping_x: map_x,
    //         _mapping_y: map_y,
    //     }
    // }

    fn get(&self, key: &Point) -> Point {
        let (x, y) = key;
        let out_x = self
            ._mapping_x
            .get(x)
            .unwrap_or_else(|| panic!("Could not find {x} in x values."));
        let out_y = self
            ._mapping_y
            .get(y)
            .unwrap_or_else(|| panic!("Could not find {y} in y values."));

        (*out_x, *out_y)
    }
}

#[derive(Debug, Clone)]
struct Rectangle {
    start: Point, // Top left corner
    height: u64,
    width: u64,
}

impl Rectangle {
    fn from_points(p1: Point, p2: Point) -> Self {
        let (x1, y1) = p1;
        let (x2, y2) = p2;

        let height = x1.abs_diff(x2) + 1;
        let width = y1.abs_diff(y2) + 1;

        let x = x1.min(x2);
        let y = y1.min(y2);

        Rectangle {
            start: (x, y),
            height,
            width,
        }
    }

    fn area(&self) -> u64 {
        self.height * self.width
    }
}

struct RectangleIntoIter {
    rectangle: Rectangle,
    index: usize,
}

impl IntoIterator for Rectangle {
    type Item = Point;
    type IntoIter = RectangleIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            rectangle: self,
            index: 0,
        }
    }
}

impl Iterator for RectangleIntoIter {
    type Item = Point;

    /// Iterate through a rectangle, row-wise.
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= (self.rectangle.width * self.rectangle.height) as usize {
            return None;
        }
        let i = self.index as u64 / self.rectangle.width;
        let j = self.index as u64 % self.rectangle.width;

        self.index += 1;

        Some((self.rectangle.start.0 + i, self.rectangle.start.1 + j))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let mut largest: u64 = 0;
    for (i, p1) in points.iter().enumerate() {
        for p2 in points[i + 1..].iter() {
            largest = largest.max(Rectangle::from_points(*p1, *p2).area());
        }
    }
    Some(largest)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Create a compressed coordinate system, enconding empty rows and columns as their width.
    let mut points = parse_input(input);

    // Append the first point to the back of the array to create the closing segment.
    points.push(points[0]);

    // Generate a mapping from the original coordinates to the compressed grid.
    let grid_mapping = Mapping::from(&points);

    // Create the boundary in the compressed grid coordinates.
    let mut boundary: HashSet<(u64, u64)> = HashSet::new();
    for (p1, p2) in points[..points.len() - 1].iter().zip(points[1..].iter()) {
        // Get compressed coordinates for p1 and p2:
        let c1 = grid_mapping.get(p1);
        let c2 = grid_mapping.get(p2);

        // Append the compressed coordinates of p1 and p2, and all points in between to the
        // boundary set.
        for p in Rectangle::from_points(c1, c2) {
            boundary.insert(p);
        }
    }

    // Fill in all outside points.
    let mut outside: HashSet<Point> = HashSet::new();
    let mut queue: Vec<Point> = Vec::new();

    // Starting point:
    queue.push((0, 0));

    let (x_max, y_max) = grid_mapping.max;
    while let Some(p) = queue.pop() {
        // If point is not boundary, add it to the `outside` set, and add its neighbours to the
        // queue.
        // NOTE: Skip if point is already in the set.
        if outside.contains(&p) {
            continue;
        }

        // If point is in the boundary, skip.
        if boundary.contains(&p) {
            continue;
        }

        outside.insert(p);

        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            let (dx, dy) = dir;
            let (x, y) = p;

            // Skip neighbour if outside the bounds.
            if x as i64 + *dx < 0
                || y as i64 + *dy < 0
                || x as i64 + *dx > x_max as i64
                || y as i64 + *dy > y_max as i64
            {
                continue;
            }

            let neighbour = (x.strict_add_signed(*dx), y.strict_add_signed(*dy));
            if outside.contains(&neighbour) {
                continue;
            }
            queue.push(neighbour);
        }
    }

    // Finally, compute rectangles like in part 1, but check if any of their points are outside.
    let mut largest: u64 = 0;
    for (i, p1) in points.iter().enumerate() {
        'second: for p2 in points[i + 1..].iter() {
            let c1 = grid_mapping.get(p1);
            let c2 = grid_mapping.get(p2);
            let rect = Rectangle::from_points(c1, c2);

            // Since the boundary is one single closed line, we only need to look at the perimeter
            // of the rectangle.
            let a = rect.start;
            let b = (rect.start.0 + rect.height, rect.start.1);
            let c = (rect.start.0 + rect.height, rect.start.1 + rect.width);
            let d = (rect.start.0, rect.start.1 + rect.width);
            for p in Rectangle::from_points(a, b) {
                if outside.contains(&p) {
                    continue 'second;
                }
            }
            for p in Rectangle::from_points(b, c) {
                if outside.contains(&p) {
                    continue 'second;
                }
            }
            for p in Rectangle::from_points(c, d) {
                if outside.contains(&p) {
                    continue 'second;
                }
            }
            for p in Rectangle::from_points(d, a) {
                if outside.contains(&p) {
                    continue 'second;
                }
            }
            largest = largest.max(Rectangle::from_points(*p1, *p2).area());
        }
    }

    Some(largest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
