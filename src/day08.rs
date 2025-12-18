pub static INPUT: &str = include_str!("../inputs/08.txt");

/// Part 1: connect the 1000 closest pairs and multiply the three largest circuit sizes.
pub fn part1(input: &str) -> Result<u64, String> {
    solve_with_limit(input, 1000).map(|(p1, _)| p1)
}

/// Part 2: keep connecting closest pairs until fully connected; return product of the X coords of
/// the final connecting edge.
pub fn part2(input: &str) -> Result<u64, String> {
    solve_with_limit(input, 1000).map(|(_, p2)| p2)
}

pub fn both(input: &str) -> Result<(u64, u64), String> {
    solve_with_limit(input, 1000)
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    dist: u64,
    a: u16,
    b: u16,
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

    let total_edges = n * (n - 1) / 2;
    let limit = pair_limit.min(total_edges);

    let mut smallest: BinaryHeap<Edge> = BinaryHeap::with_capacity(limit.saturating_add(1));

    // Prim's algorithm over the implicit complete graph:
    // - compute the MST's maximum edge weight (last edge Kruskal would add with unique weights)
    // - while computing distances, also maintain the `limit` smallest edges for part 1
    let mut min_dist = vec![u64::MAX; n];
    let mut parent = vec![0usize; n];
    min_dist[0] = 0;

    let mut max_edge: Option<(u64, usize, usize)> = None;
    let mut heap_full = limit == 0;
    let mut heap_max = Edge { dist: u64::MAX, a: 0, b: 0 };

    let mut remaining: Vec<usize> = (0..n).collect();
    for _ in 0..n {
        let mut best = u64::MAX;
        let mut best_pos = 0usize;
        let mut u = unsafe { *remaining.get_unchecked(0) };
        for (pos, &idx) in remaining.iter().enumerate() {
            let d = unsafe { *min_dist.get_unchecked(idx) };
            if d < best || (d == best && idx < u) {
                best = d;
                best_pos = pos;
                u = idx;
            }
        }
        remaining.swap_remove(best_pos);

        if u != 0 {
            let a = unsafe { *parent.get_unchecked(u) };
            let b = u;
            match max_edge {
                Some((d, _, _)) if d >= best => {}
                _ => max_edge = Some((best, a, b)),
            }
        }

        let pu = unsafe { points.get_unchecked(u) };
        for &v in remaining.iter() {
            let pv = unsafe { points.get_unchecked(v) };
            let dist = sq_dist(pu, pv);

            if limit != 0 {
                if !heap_full {
                    let (a, b) = if u < v { (u, v) } else { (v, u) };
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
                    let (a, b) = if u < v { (u, v) } else { (v, u) };
                    smallest.pop();
                    smallest.push(Edge {
                        dist,
                        a: a as u16,
                        b: b as u16,
                    });
                    heap_max = unsafe { *smallest.peek().unwrap_unchecked() };
                } else if dist == heap_max.dist {
                    let (a, b) = if u < v { (u, v) } else { (v, u) };
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

            let cur = unsafe { *min_dist.get_unchecked(v) };
            if dist < cur || (dist == cur && u < unsafe { *parent.get_unchecked(v) }) {
                unsafe {
                    *min_dist.get_unchecked_mut(v) = dist;
                    *parent.get_unchecked_mut(v) = u;
                }
            }
        }
    }

    let (_max_dist, max_a, max_b) =
        max_edge.ok_or_else(|| "graph never became fully connected".to_string())?;

    let mut parents: Vec<u16> = (0..n as u16).collect();
    let mut sizes = vec![1u32; n];
    for edge in smallest.into_iter() {
        let _ = union(edge.a, edge.b, &mut parents, &mut sizes);
    }
    let top_after_limit = top_three(&parents, &sizes);

    let p1 = top_after_limit[0] as u64 * top_after_limit[1] as u64 * top_after_limit[2] as u64;
    let ax = unsafe { *points.get_unchecked(max_a) }[0];
    let bx = unsafe { *points.get_unchecked(max_b) }[0];
    let p2 = (ax * bx) as u64;
    Ok((p1, p2))
}

#[inline(always)]
fn sq_dist(a: &[i64; 3], b: &[i64; 3]) -> u64 {
    let dx = unsafe { *a.get_unchecked(0) } - unsafe { *b.get_unchecked(0) };
    let dy = unsafe { *a.get_unchecked(1) } - unsafe { *b.get_unchecked(1) };
    let dz = unsafe { *a.get_unchecked(2) } - unsafe { *b.get_unchecked(2) };
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

fn parse_points(input: &str) -> Result<Vec<[i64; 3]>, String> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut idx = 0usize;
    let mut points: Vec<[i64; 3]> = Vec::with_capacity(1024);

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

        points.push([x as i64, y as i64, z as i64]);
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
    use super::{both, part2, solve_with_limit};

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
}
