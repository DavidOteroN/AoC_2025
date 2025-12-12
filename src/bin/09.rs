advent_of_code::solution!(9);

type Point = (u64, u64);
type Segment = (Point, Point);

fn parse_input(input: &str) -> Vec<Point> {
    let mut output: Vec<_> = Vec::new();
    for line in input.trim().lines() {
        if let [x, y] = line.split(",").collect::<Vec<&str>>()[..] {
            output.push((x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()));
        }
    }
    output
}

fn compute_area(p1: &Point, p2: &Point) -> u64 {
    let (a, b) = *p1;
    let (c, d) = *p2;

    (a.abs_diff(c) + 1) * (b.abs_diff(d) + 1)
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let mut largest: u64 = 0;
    for (i, p1) in points.iter().enumerate() {
        for p2 in points[i + 1..].iter() {
            largest = largest.max(compute_area(p1, p2));
        }
    }
    Some(largest)
}

fn intersect(s1: &Segment, s2: &Segment) -> bool {
    // Defining points A, B as the end points of segment s1, and C, D as the end points of segment
    // s2, and calling u the vector from A to B, and v the vector from C to D, we can construct the
    // following system of equations:
    //
    //  A + alpha * u = C + beta * v
    //
    // Where u = B - A, and v = D - C. Therefore:
    //
    //  Ax + alpha * (Bx - Ax) = Cx + beta * (Dx - Cx)
    //  Ay + alpha * (By - Ay) = Cy + beta * (Dy - Cy)
    //
    // Re-arranging and writing in matrix form:
    //
    //  | (Bx - Ax)   - (Dx - Cx) || alpha |   | Cx - Ax |
    //  |                         ||       | = |         |
    //  | (By - Ay)   - (Dy - Cy) || beta  |   | Cy - Ay |
    //
    // The system can be solved by computing the inverse matrix.
    // NOTE: This is what we call "matar moscas a cañonazos" in spanish.

    let (a, b) = *s1;
    let (c, d) = *s2;

    let (ax, ay) = (a.0 as f64, a.1 as f64);
    let (bx, by) = (b.0 as f64, b.1 as f64);
    let (cx, cy) = (c.0 as f64, c.1 as f64);
    let (dx, dy) = (d.0 as f64, d.1 as f64);

    let det = (by - ay) * (dx - cx) + (bx - ax) * (dy - cy);

    // If the determinant is 0, it means that the segments are parallel, so no intersection.
    if det == 0f64 {
        return false;
    }

    // Calling the components of the inverse matrix (p, q, r, s):
    //
    //  | p  q |
    //  | r  s |
    //
    let p = -(dy - cy) / det;
    let q = -(by - ay) / det;
    let r = -(dx - cx) / det;
    let s = -(bx - ax) / det;

    // So finally the solution of the system is:
    let alpha = p * (cx - ax) + q * (cy - ay);
    let beta = r * (cx - ax) + s * (cy - ay);

    // There is an intersection point contained within both segments if both alpha and beta are > 0
    // and < 1.
    if alpha > 0f64 && beta > 0f64 && alpha < 1f64 && beta < 1f64 {
        return true;
    }
    false
}

fn rectangle_segs(p1: &Point, p2: &Point) -> impl Iterator<Item = Segment> {
    let (x1, y1) = *p1;
    let (x2, y2) = *p2;

    // Define the points of the rectangle in a clockwise way:
    // A ---- B
    // |      |
    // D -----C
    let a: Point = *p1;
    let b: Point = (x2, y1);
    let c: Point = *p2;
    let d: Point = (x1, y2);

    // Create an iterator over the four segments:
    // A -> B
    // B -> C
    // C -> D
    // D -> A
    [(a, b), (b, c), (c, d), (d, a)].into_iter()
}

pub fn part_two(input: &str) -> Option<u64> {
    // Now the provided coordinates form a loop. The goal is to make the largest possible rectangle
    // (by area) using a pair of coordinates as opposite coordinates that is fully contained within
    // the loop.
    // One possible approach is to create a compressed coordinate system, encoding "empty" rows or
    // columns as their height or width.
    // Another appsoach is to compute intersections of the sides of each possible rectangle with
    // the segments in the loop. If there's an intersection, then the rectangle is not valid.
    let mut points = parse_input(input);

    // Append the first element to the back of the vector so that all the closing segment is also
    // created.
    points.push(points[0]);

    // Create a segments vector.
    let segments: Vec<_> = points[0..points.len() - 1]
        .iter()
        .cloned()
        .zip(points[1..].iter().cloned())
        .collect();

    // Iterate though the points vector and create the rectangles. Intersect each side of the
    // rectangle against all segments and continue to next rectangle if there's an intersection.
    // NOTE: `segments` is the same length as `points`, so this implementation ends up being O(n³),
    // which is not great.
    let mut largest: u64 = 0;
    for (i, p1) in points.iter().enumerate() {
        'a: for p2 in points[i + 1..].iter() {
            // NOTE: There is a corner case where the full rectangle lies outside the loop.
            // This is precisely wht happens in the example data.

            // Compute intersections.
            for s in rectangle_segs(p1, p2) {
                for s2 in &segments {
                    if intersect(&s, s2) {
                        continue 'a;
                    }
                }
            }
            largest = largest.max(compute_area(p1, p2));
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
