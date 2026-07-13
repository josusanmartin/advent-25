pub static INPUT: &str = include_str!("../inputs/08.txt");

/// Part 1: connect the 1000 closest pairs and multiply the three largest circuit sizes.
pub fn part1(input: &str) -> Result<u64, String> {
    let points = parse_validated_points(input)?;
    let total_edges = points.len() * (points.len() - 1) / 2;
    Ok(component_product(
        points.len(),
        smallest_edges(&points, 1000.min(total_edges)),
    ))
}

/// Part 2: keep connecting closest pairs until fully connected; return product of the X coords of
/// the final connecting edge.
pub fn part2(input: &str) -> Result<u64, String> {
    let points = parse_validated_points(input)?;
    let (_, a, b) =
        mst_last_edge(&points).ok_or_else(|| "graph never became fully connected".to_string())?;
    Ok(edge_x_product(&points, a, b))
}

pub fn both(input: &str) -> Result<(u64, u64), String> {
    solve_with_limit(input, 1000)
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Edge {
    dist: u64,
    a: u16,
    b: u16,
}

#[derive(Copy, Clone)]
struct Frontier {
    min_dist: u64,
    point: [i32; 3],
    parent: u16,
    id: u16,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist
            .cmp(&other.dist)
            .then_with(|| self.a.cmp(&other.a))
            .then_with(|| self.b.cmp(&other.b))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_with_limit(input: &str, pair_limit: usize) -> Result<(u64, u64), String> {
    let points = parse_validated_points(input)?;
    let n = points.len();
    let total_edges = n * (n - 1) / 2;
    let limit = pair_limit.min(total_edges);

    #[cfg(feature = "parallel")]
    let (max_edge, smallest) =
        rayon::join(|| mst_last_edge(&points), || smallest_edges(&points, limit));

    #[cfg(not(feature = "parallel"))]
    let (max_edge, smallest) = (mst_last_edge(&points), smallest_edges(&points, limit));

    let (_, max_a, max_b) =
        max_edge.ok_or_else(|| "graph never became fully connected".to_string())?;
    Ok((
        component_product(n, smallest),
        edge_x_product(&points, max_a, max_b),
    ))
}

fn parse_validated_points(input: &str) -> Result<Vec<[i32; 3]>, String> {
    let points = parse_points(input)?;
    let n = points.len();
    if n == 0 {
        return Err("input contained no points".into());
    }
    if n > u16::MAX as usize {
        return Err("too many points".into());
    }
    if n < 2 {
        return Err("need at least two points".into());
    }
    Ok(points)
}

// Prim's algorithm over the implicit complete graph. Keeping this loop independent from the
// top-k edge selection makes the serial dependency chain substantially shorter in parallel mode.
fn mst_last_edge(points: &[[i32; 3]]) -> Option<(u64, u16, u16)> {
    let n = points.len();
    let mut max_edge: Option<(u64, u16, u16)> = None;
    let mut remaining: Vec<Frontier> = points
        .iter()
        .enumerate()
        .map(|(id, &point)| Frontier {
            min_dist: if id == 0 { 0 } else { u64::MAX },
            point,
            parent: 0,
            id: id as u16,
        })
        .collect();
    for _ in 0..n {
        let mut best = u64::MAX;
        let mut best_pos = 0usize;
        let mut best_id = unsafe { remaining.get_unchecked(0).id };
        for (pos, node) in remaining.iter().enumerate() {
            if node.min_dist < best || (node.min_dist == best && node.id < best_id) {
                best = node.min_dist;
                best_pos = pos;
                best_id = node.id;
            }
        }
        let u = remaining.swap_remove(best_pos);

        if u.id != 0 {
            match max_edge {
                Some((d, _, _)) if d >= best => {}
                _ => max_edge = Some((best, u.parent, u.id)),
            }
        }

        for v in &mut remaining {
            let dist = sq_dist(&u.point, &v.point);
            if dist < v.min_dist || (dist == v.min_dist && u.id < v.parent) {
                v.min_dist = dist;
                v.parent = u.id;
            }
        }
    }
    max_edge
}

fn smallest_edges(points: &[[i32; 3]], limit: usize) -> BinaryHeap<Edge> {
    let mut smallest = BinaryHeap::with_capacity(limit.saturating_add(1));
    if limit == 0 {
        return smallest;
    }

    // A modest prefix gives us a strong, exact upper bound for the global top-k. Every edge
    // rejected from the prefix is already dominated by `limit` prefix edges, so it cannot enter
    // the global result. The rest of the graph therefore only needs a cheap threshold check,
    // followed by one linear-time selection, instead of maintaining a heap for every close edge.
    let n = points.len();
    let mut sample_rows = 64.min(n - 1);
    let mut sampled_edges = sample_rows * (2 * n - sample_rows - 1) / 2;
    while sampled_edges < limit {
        sample_rows += 1;
        sampled_edges += n - sample_rows;
    }

    let mut heap_full = false;
    let mut heap_max = Edge {
        dist: u64::MAX,
        a: 0,
        b: 0,
    };
    for a in 0..sample_rows {
        let pa = unsafe { points.get_unchecked(a) };
        for b in a + 1..n {
            let dist = sq_dist(pa, unsafe { points.get_unchecked(b) });
            if !heap_full {
                smallest.push(Edge {
                    dist,
                    a: a as u16,
                    b: b as u16,
                });
                if smallest.len() == limit {
                    heap_full = true;
                    heap_max = unsafe { *smallest.peek().unwrap_unchecked() };
                }
            } else if dist < heap_max.dist {
                smallest.pop();
                smallest.push(Edge {
                    dist,
                    a: a as u16,
                    b: b as u16,
                });
                heap_max = unsafe { *smallest.peek().unwrap_unchecked() };
            } else if dist == heap_max.dist {
                let edge = Edge {
                    dist,
                    a: a as u16,
                    b: b as u16,
                };
                if edge < heap_max {
                    smallest.pop();
                    smallest.push(edge);
                    heap_max = unsafe { *smallest.peek().unwrap_unchecked() };
                }
            }
        }
    }

    if sample_rows == n - 1 {
        return smallest;
    }

    let threshold = unsafe { *smallest.peek().unwrap_unchecked() };
    let expected_survivors = limit
        .saturating_mul(n * (n - 1) / 2)
        .checked_div(sampled_edges)
        .unwrap_or(limit);
    let mut candidates = smallest.into_vec();
    candidates.reserve(expected_survivors.saturating_sub(limit));

    for a in sample_rows..n - 1 {
        let pa = unsafe { points.get_unchecked(a) };
        for b in a + 1..n {
            let dist = sq_dist(pa, unsafe { points.get_unchecked(b) });
            if dist < threshold.dist {
                candidates.push(Edge {
                    dist,
                    a: a as u16,
                    b: b as u16,
                });
            } else if dist == threshold.dist {
                let edge = Edge {
                    dist,
                    a: a as u16,
                    b: b as u16,
                };
                if edge < threshold {
                    candidates.push(edge);
                }
            }
        }
    }

    if candidates.len() > limit {
        candidates.select_nth_unstable(limit);
        candidates.truncate(limit);
    }
    BinaryHeap::from(candidates)
}

fn component_product(n: usize, smallest: BinaryHeap<Edge>) -> u64 {
    let mut parents: Vec<u16> = (0..n as u16).collect();
    let mut sizes = vec![1u32; n];
    for edge in smallest.into_iter() {
        let _ = union(edge.a, edge.b, &mut parents, &mut sizes);
    }
    let top_after_limit = top_three(&parents, &sizes);

    top_after_limit[0] as u64 * top_after_limit[1] as u64 * top_after_limit[2] as u64
}

#[inline(always)]
fn edge_x_product(points: &[[i32; 3]], a: u16, b: u16) -> u64 {
    let ax = unsafe { *points.get_unchecked(a as usize) }[0] as i64;
    let bx = unsafe { *points.get_unchecked(b as usize) }[0] as i64;
    (ax * bx) as u64
}

#[inline(always)]
fn sq_dist(a: &[i32; 3], b: &[i32; 3]) -> u64 {
    let dx = unsafe { *a.get_unchecked(0) } as i64 - unsafe { *b.get_unchecked(0) } as i64;
    let dy = unsafe { *a.get_unchecked(1) } as i64 - unsafe { *b.get_unchecked(1) } as i64;
    let dz = unsafe { *a.get_unchecked(2) } as i64 - unsafe { *b.get_unchecked(2) } as i64;
    (dx * dx + dy * dy + dz * dz) as u64
}

#[inline(always)]
fn top_three(parents: &[u16], sizes: &[u32]) -> [u32; 3] {
    let mut top = [0u32; 3];
    for idx in 0..parents.len() {
        if unsafe { *parents.get_unchecked(idx) } == idx as u16 {
            let sz = unsafe { *sizes.get_unchecked(idx) };
            if sz > top[0] {
                top[2] = top[1];
                top[1] = top[0];
                top[0] = sz;
            } else if sz > top[1] {
                top[2] = top[1];
                top[1] = sz;
            } else if sz > top[2] {
                top[2] = sz;
            }
        }
    }
    top
}

fn parse_points(input: &str) -> Result<Vec<[i32; 3]>, String> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut idx = 0usize;
    let mut points: Vec<[i32; 3]> = Vec::with_capacity(1024);

    while idx < len {
        while idx < len {
            let b = unsafe { *bytes.get_unchecked(idx) };
            if b == b'\n' || b == b'\r' {
                idx += 1;
            } else {
                break;
            }
        }
        if idx >= len {
            break;
        }

        let x = parse_int(bytes, len, &mut idx)?;
        expect_char(bytes, len, &mut idx, b',')?;
        let y = parse_int(bytes, len, &mut idx)?;
        expect_char(bytes, len, &mut idx, b',')?;
        let z = parse_int(bytes, len, &mut idx)?;

        while idx < len {
            let b = unsafe { *bytes.get_unchecked(idx) };
            idx += 1;
            if b == b'\n' {
                break;
            }
            if b == b'\r' {
                if idx < len && unsafe { *bytes.get_unchecked(idx) } == b'\n' {
                    idx += 1;
                }
                break;
            }
        }

        points.push([x, y, z]);
    }

    Ok(points)
}

#[inline(always)]
fn parse_int(bytes: &[u8], len: usize, idx: &mut usize) -> Result<i32, String> {
    if *idx >= len {
        return Err("unexpected end of input".into());
    }
    let mut neg = false;
    let mut b = unsafe { *bytes.get_unchecked(*idx) };
    if b == b'-' {
        neg = true;
        *idx += 1;
        if *idx >= len {
            return Err("unexpected end of input".into());
        }
        b = unsafe { *bytes.get_unchecked(*idx) };
    }
    if b < b'0' || b > b'9' {
        return Err(format!("expected digit, found '{}'", b as char));
    }

    let mut val: i32 = 0;
    while *idx < len {
        let b = unsafe { *bytes.get_unchecked(*idx) };
        if b < b'0' || b > b'9' {
            break;
        }
        val = val * 10 + (b - b'0') as i32;
        *idx += 1;
    }

    Ok(if neg { -val } else { val })
}

#[inline(always)]
fn expect_char(bytes: &[u8], len: usize, idx: &mut usize, expected: u8) -> Result<(), String> {
    if *idx >= len || unsafe { *bytes.get_unchecked(*idx) } != expected {
        return Err("invalid input format".into());
    }
    *idx += 1;
    Ok(())
}

#[inline(always)]
fn find(x: u16, parents: &mut [u16]) -> u16 {
    let mut p = x;
    while unsafe { *parents.get_unchecked(p as usize) } != p {
        p = unsafe { *parents.get_unchecked(p as usize) };
    }
    let mut cur = x;
    while cur != p {
        let next = unsafe { *parents.get_unchecked(cur as usize) };
        unsafe {
            *parents.get_unchecked_mut(cur as usize) = p;
        }
        cur = next;
    }
    p
}

#[inline(always)]
fn union(a: u16, b: u16, parents: &mut [u16], sizes: &mut [u32]) -> bool {
    let mut ra = find(a, parents);
    let mut rb = find(b, parents);
    if ra == rb {
        return false;
    }
    let sa = unsafe { *sizes.get_unchecked(ra as usize) };
    let sb = unsafe { *sizes.get_unchecked(rb as usize) };
    if sa < sb {
        std::mem::swap(&mut ra, &mut rb);
    }
    unsafe {
        *parents.get_unchecked_mut(rb as usize) = ra;
        *sizes.get_unchecked_mut(ra as usize) = sa + sb;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{both, part2, smallest_edges, solve_with_limit, sq_dist, Edge};

    const EXAMPLE: &str = "162,817,812\n\
57,618,57\n\
906,360,560\n\
592,479,940\n\
352,342,300\n\
466,668,158\n\
542,29,236\n\
431,825,988\n\
739,650,466\n\
52,470,668\n\
216,146,977\n\
819,987,18\n\
117,168,530\n\
805,96,715\n\
346,949,466\n\
970,615,88\n\
941,993,340\n\
862,61,35\n\
984,92,344\n\
425,690,689\n";

    #[test]
    fn example_top10() {
        assert_eq!(solve_with_limit(EXAMPLE, 10).unwrap().0, 40);
    }

    #[test]
    fn example_full_connect() {
        assert_eq!(part2(EXAMPLE).unwrap(), 25_272);
    }

    #[test]
    fn puzzle_input_runs() {
        let _ = both(super::INPUT).unwrap();
    }

    #[test]
    fn threshold_selection_matches_full_sort() {
        let mut state = 0x9e37_79b9u32;
        for n in 2..=80 {
            let points: Vec<[i32; 3]> = (0..n)
                .map(|_| {
                    let mut coordinate = || {
                        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                        ((state >> 8) % 41) as i32 - 20
                    };
                    [coordinate(), coordinate(), coordinate()]
                })
                .collect();
            let total = n * (n - 1) / 2;
            for limit in [1, 2, 7, 31, 1000, total] {
                let limit = limit.min(total);
                let actual = smallest_edges(&points, limit).into_sorted_vec();
                let mut expected = Vec::with_capacity(total);
                for a in 0..n - 1 {
                    for b in a + 1..n {
                        expected.push(Edge {
                            dist: sq_dist(&points[a], &points[b]),
                            a: a as u16,
                            b: b as u16,
                        });
                    }
                }
                expected.sort_unstable();
                expected.truncate(limit);
                assert_eq!(actual, expected, "n={n}, limit={limit}");
            }
        }
    }
}
