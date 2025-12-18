pub static INPUT: &str = include_str!("../inputs/11.txt");

use std::collections::HashMap;

const OUT_ID: u16 = 0;
const YOU_ID: u16 = 1;
const SVR_ID: u16 = 2;
const DAC_ID: u16 = 3;
const FFT_ID: u16 = 4;

struct Graph {
    adj: Vec<Vec<u16>>,
}

/// Count all paths from "you" to "out".
pub fn part1(input: &str) -> Result<u64, String> {
    let graph = parse_graph(input)?;
    let mut memo = vec![u64::MAX; graph.adj.len()];
    Ok(count_paths_to(&graph, YOU_ID, OUT_ID, u16::MAX, &mut memo))
}

/// Count paths from "svr" to "out" that pass through both "dac" and "fft".
pub fn part2(input: &str) -> Result<u64, String> {
    let graph = parse_graph(input)?;
    Ok(count_paths_through_both(&graph))
}

pub fn both(input: &str) -> Result<(u64, u64), String> {
    let graph = parse_graph(input)?;
    let n = graph.adj.len();

    let mut memo = vec![u64::MAX; n];
    let p1 = count_paths_to(&graph, YOU_ID, OUT_ID, u16::MAX, &mut memo);

    let p2 = count_paths_through_both(&graph);

    Ok((p1, p2))
}

fn count_paths_through_both(graph: &Graph) -> u64 {
    let n = graph.adj.len();

    // Paths: svr -> dac -> fft -> out
    let mut memo1 = vec![u64::MAX; n];
    let svr_to_dac = count_paths_to(graph, SVR_ID, DAC_ID, FFT_ID, &mut memo1);
    let mut memo2 = vec![u64::MAX; n];
    let dac_to_fft = count_paths_to(graph, DAC_ID, FFT_ID, u16::MAX, &mut memo2);
    let mut memo3 = vec![u64::MAX; n];
    let fft_to_out = count_paths_to(graph, FFT_ID, OUT_ID, u16::MAX, &mut memo3);

    let paths_dac_then_fft = svr_to_dac * dac_to_fft * fft_to_out;

    // Paths: svr -> fft -> dac -> out
    let mut memo4 = vec![u64::MAX; n];
    let svr_to_fft = count_paths_to(graph, SVR_ID, FFT_ID, DAC_ID, &mut memo4);
    let mut memo5 = vec![u64::MAX; n];
    let fft_to_dac = count_paths_to(graph, FFT_ID, DAC_ID, u16::MAX, &mut memo5);
    let mut memo6 = vec![u64::MAX; n];
    let dac_to_out = count_paths_to(graph, DAC_ID, OUT_ID, u16::MAX, &mut memo6);

    let paths_fft_then_dac = svr_to_fft * fft_to_dac * dac_to_out;

    paths_dac_then_fft + paths_fft_then_dac
}

#[inline]
fn count_paths_to(graph: &Graph, node: u16, target: u16, forbidden: u16, memo: &mut [u64]) -> u64 {
    if node == target {
        return 1;
    }
    if node == forbidden {
        return 0;
    }

    let idx = node as usize;
    if memo[idx] != u64::MAX {
        return memo[idx];
    }

    let mut total = 0u64;
    for &neighbor in &graph.adj[idx] {
        total += count_paths_to(graph, neighbor, target, forbidden, memo);
    }

    memo[idx] = total;
    total
}

fn parse_graph(input: &str) -> Result<Graph, String> {
    let mut name_to_id: HashMap<&str, u16> = HashMap::with_capacity(256);

    // Pre-assign special IDs
    name_to_id.insert("out", OUT_ID);
    name_to_id.insert("you", YOU_ID);
    name_to_id.insert("svr", SVR_ID);
    name_to_id.insert("dac", DAC_ID);
    name_to_id.insert("fft", FFT_ID);

    let mut next_id = 5u16;

    // First pass: collect all node names
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let colon = line.find(':').ok_or("Invalid line format")?;
        let source = line[..colon].trim();

        if !name_to_id.contains_key(source) {
            name_to_id.insert(source, next_id);
            next_id += 1;
        }

        for target in line[colon + 1..].split_whitespace() {
            if !name_to_id.contains_key(target) {
                name_to_id.insert(target, next_id);
                next_id += 1;
            }
        }
    }

    let num_nodes = next_id as usize;
    let mut adj: Vec<Vec<u16>> = vec![Vec::new(); num_nodes];

    // Second pass: build adjacency lists
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let colon = line.find(':').unwrap();
        let source = line[..colon].trim();
        let source_id = name_to_id[source];

        let targets: Vec<u16> = line[colon + 1..]
            .split_whitespace()
            .map(|t| name_to_id[t])
            .collect();

        adj[source_id as usize] = targets;
    }

    Ok(Graph { adj })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const EXAMPLE2: &str = "\
svr: aaa bbb
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
hhh: out
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE2).unwrap();
        assert_eq!(result, 2);
    }
}
