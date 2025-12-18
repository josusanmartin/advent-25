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

#[derive(Copy, Clone)]
struct Edge {
    dist: u64,
    a: u16,
    b: u16,
}

fn solve_with_limit(input: &str, pair_limit: usize) -> Result<(u64, u64), String> {
    let points = parse_points(input)?;
    let n = points.len();
    if n == 0 {
        return Err("input contained no points".into());
    }

    let mut edges = build_edges(&points);
    let limit = pair_limit.min(edges.len());

    let mut parents: Vec<u16> = (0..n as u16).collect();
    let mut sizes = vec![1u32; n];
    let mut comps = n as u32;
    let mut part1_done = limit == 0;
    let mut top_after_limit = [0u32; 3];
    let mut part2_product: Option<u64> = None;

    let mut processed = 0usize;
    let mut edges_seen = 0usize;
    let mut chunk = 8192usize.min(edges.len().max(1));

    while processed < edges.len() {
        let tail = &mut edges[processed..];
        let take = chunk.min(tail.len());
        if take == 0 {
            break;
        }

        tail.select_nth_unstable_by(take - 1, |a, b| a.dist.cmp(&b.dist));
        radix_sort_edges(&mut tail[..take]);

        for edge in &tail[..take] {
            edges_seen += 1;
            if union(edge.a, edge.b, &mut parents, &mut sizes) {
                comps -= 1;
                if part2_product.is_none() && comps == 1 {
                    let ax = unsafe { *points.get_unchecked(edge.a as usize) }[0] as i64;
                    let bx = unsafe { *points.get_unchecked(edge.b as usize) }[0] as i64;
                    part2_product = Some((ax * bx) as u64);
                }
            }

            if !part1_done && edges_seen == limit {
                top_after_limit = top_three(&parents, &sizes);
                part1_done = true;
                if part2_product.is_some() {
                    break;
                }
            }
        }

        if part1_done && part2_product.is_some() {
            break;
        }

        processed += take;
        let remaining = edges.len() - processed;
        if remaining == 0 {
            break;
        }
        chunk = (chunk.saturating_mul(2)).min(remaining);
    }

    if !part1_done {
        top_after_limit = top_three(&parents, &sizes);
    }

    let p1 = top_after_limit[0] as u64 * top_after_limit[1] as u64 * top_after_limit[2] as u64;
    let p2 = part2_product.ok_or_else(|| "graph never became fully connected".to_string())?;
    Ok((p1, p2))
}

#[inline(always)]
fn radix_sort_edges(edges: &mut [Edge]) {
    let len = edges.len();
    if len <= 1 {
        return;
    }

    let mut buf = vec![Edge { dist: 0, a: 0, b: 0 }; len];

    const RADIX: usize = 1 << 16;
    let mut counts = [0usize; RADIX];
    let mut offsets = [0usize; RADIX];

    // Track which buffer contains the result
    let mut in_buf = false;

    let mut shift = 0u32;
    while shift < 64 {
        counts.fill(0);

        let src: &[Edge] = if in_buf { &buf } else { edges };
        for e in src.iter() {
            counts[((e.dist >> shift) & 0xFFFF) as usize] += 1;
        }

        let mut sum = 0usize;
        for i in 0..RADIX {
            let c = unsafe { *counts.get_unchecked(i) };
            unsafe { *offsets.get_unchecked_mut(i) = sum };
            sum += c;
        }

        let (src, dst): (&[Edge], &mut [Edge]) = if in_buf {
            (&buf, edges)
        } else {
            (edges, &mut buf)
        };

        for e in src.iter() {
            let idx = ((e.dist >> shift) & 0xFFFF) as usize;
            let pos = unsafe { *offsets.get_unchecked(idx) };
            unsafe {
                *dst.get_unchecked_mut(pos) = *e;
                *offsets.get_unchecked_mut(idx) = pos + 1;
            }
        }

        in_buf = !in_buf;
        shift += 16;
    }

    if in_buf {
        edges.copy_from_slice(&buf);
    }
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

fn build_edges(points: &[[i32; 3]]) -> Vec<Edge> {
    let n = points.len();
    let total = n * (n - 1) / 2;
    let mut edges: Vec<Edge> = Vec::with_capacity(total);
    unsafe { edges.set_len(total) };

    let mut idx = 0usize;
    for i in 0..n {
        let pi = points[i];
        for j in (i + 1)..n {
            let pj = points[j];
            let dx = (pi[0] as i64) - (pj[0] as i64);
            let dy = (pi[1] as i64) - (pj[1] as i64);
            let dz = (pi[2] as i64) - (pj[2] as i64);
            let dist = (dx * dx + dy * dy + dz * dz) as u64;
            unsafe {
                *edges.get_unchecked_mut(idx) = Edge {
                    dist,
                    a: i as u16,
                    b: j as u16,
                };
            }
            idx += 1;
        }
    }
    edges
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
