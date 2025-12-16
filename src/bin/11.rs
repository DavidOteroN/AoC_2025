advent_of_code::solution!(11);

use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut out: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.trim().lines() {
        if let [id, rest] = line.split(":").collect::<Vec<&str>>()[..] {
            let outports: Vec<&str> = rest.trim().split(" ").collect();
            out.insert(id, outports);
        }
    }
    out
}

fn count_paths(servers: &HashMap<&str, Vec<&str>>, src: &str, dest: &str) -> u64 {
    let mut queue: Vec<&str> = Vec::new();
    let mut count: u64 = 0;
    queue.push(src);
    while let Some(server) = queue.pop() {
        if server == dest {
            count += 1;
        } else if let Some(outports) = servers.get(&server) {
            queue.extend(outports);
        }
    }
    count
}

/// The original queue implementation was waaaaaay too slow for part two, so here's a recursive,
/// memoized implementation.
fn count_paths_mem(
    servers: &HashMap<&str, Vec<&str>>,
    src: &str,
    dest: &str,
    cache: &mut HashMap<(String, String), u64>,
) -> u64 {
    // Base case:
    if src == dest {
        return 1;
    }

    if !servers.contains_key(src) {
        return 0;
    }

    // Return from cache if available:
    if let Some(c) = cache.get(&(src.to_string(), dest.to_string())) {
        return *c;
    }

    let mut count: u64 = 0;
    servers.get(src).unwrap().iter().for_each(|&node| {
        count += count_paths_mem(servers, node, dest, cache);
    });

    // Store result in cache.
    cache.insert((src.to_string(), dest.to_string()), count);
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let servers = parse_input(input);

    Some(count_paths(&servers, "you", "out"))
}

pub fn part_two(input: &str) -> Option<u64> {
    // Compute the paths from "svr" to "out" that also visit both "dac" and "fft" (in any order).
    let servers = parse_input(input);

    // The problem can be broken into parts, since there are no cycles in the graph (DAG).
    // Let's say we want to compute all the paths that go from A to C visiting B. THen the numberr
    // of possible paths is the product of the number of paths from A to B and the number of paths
    // from B to C.

    let mut cache: HashMap<(String, String), u64> = HashMap::new();
    let svr_dac = count_paths_mem(&servers, "svr", "dac", &mut cache);
    let dac_fft = count_paths_mem(&servers, "dac", "fft", &mut cache);
    let fft_out = count_paths_mem(&servers, "fft", "out", &mut cache);
    let svr_fft = count_paths_mem(&servers, "svr", "fft", &mut cache);
    let fft_dac = count_paths_mem(&servers, "fft", "dac", &mut cache);
    let dac_out = count_paths_mem(&servers, "dac", "out", &mut cache);

    let result = svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out;
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        // NOTE: Input for this example is different.
        let input: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let result = part_two(input);
        assert_eq!(result, Some(2));
    }
}
