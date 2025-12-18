pub static INPUT: &str = include_str!("../inputs/09.txt");

/// Part 1: largest axis-aligned rectangle that uses red tiles for two opposite corners.
pub fn part1(input: &str) -> Result<u64, String> {
    let points = parse_points(input)?;
    if points.len() < 2 {
        return Err("need at least two red tiles".into());
    }
    Ok(max_area_any(&points))
}

/// Part 2: largest rectangle whose tiles are all red or green (inside the loop).
pub fn part2(input: &str) -> Result<u64, String> {
    let points = parse_points(input)?;
    let coverage = build_coverage(&points)?;
    Ok(max_area_within_green(&points, &coverage))
}

/// Compute both parts with a shared parse.
pub fn both(input: &str) -> Result<(u64, u64), String> {
    let points = parse_points(input)?;
    if points.len() < 2 {
        return Err("need at least two red tiles".into());
    }
    let part1 = max_area_any(&points);
    let coverage = build_coverage(&points)?;
    let part2 = max_area_within_green(&points, &coverage);
    Ok((part1, part2))
}

#[inline(always)]
fn parse_points(input: &str) -> Result<Vec<[i32; 2]>, String> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut idx = 0usize;
    let mut points: Vec<[i32; 2]> = Vec::with_capacity(512);

    while idx < len {
        // Skip blank lines.
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

        // Consume the rest of the line.
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

        points.push([x, y]);
    }

    Ok(points)
}

/// Part 1: Find largest rectangle with red tiles at opposite corners.
/// Simple O(n²) max scan.
#[inline(always)]
fn max_area_any(points: &[[i32; 2]]) -> u64 {
    let n = points.len();
    if n < 2 {
        return 0;
    }

    let mut max_area: u64 = 0;
    for i in 0..n {
        let pi = unsafe { *points.get_unchecked(i) };
        for j in (i + 1)..n {
            let pj = unsafe { *points.get_unchecked(j) };
            let dx = (pi[0] - pj[0]).unsigned_abs() as u64 + 1;
            let dy = (pi[1] - pj[1]).unsigned_abs() as u64 + 1;
            let area = dx * dy;
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}

struct Coverage {
    prefix: Vec<u64>,
    stride: usize,
    xs: Vec<i32>,
    ys: Vec<i32>,
}

#[inline(always)]
fn build_coverage(points: &[[i32; 2]]) -> Result<Coverage, String> {
    if points.len() < 3 {
        return Err("need at least three red tiles to form a loop".into());
    }

    let mut xs: Vec<i32> = Vec::with_capacity(points.len() * 2 + 1);
    let mut ys: Vec<i32> = Vec::with_capacity(points.len() * 2 + 1);
    for p in points {
        xs.push(p[0]);
        xs.push(p[0] + 1);
        ys.push(p[1]);
        ys.push(p[1] + 1);
    }
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    let width = xs.len() - 1;
    let height = ys.len() - 1;
    let mut crossings: Vec<Vec<i32>> = vec![Vec::new(); height];
    let mut spans: Vec<Vec<(i32, i32)>> = vec![Vec::new(); height];

    // Edges of the loop: connect each point to the next, wrapping.
    for idx in 0..points.len() {
        let a = unsafe { *points.get_unchecked(idx) };
        let b = unsafe { *points.get_unchecked((idx + 1) % points.len()) };
        if a[0] == b[0] {
            // Vertical edge.
            let x = a[0];
            let (y_start, y_end) = if a[1] < b[1] {
                (a[1], b[1])
            } else {
                (b[1], a[1])
            };
            let start_idx = lower_bound(&ys, y_start);
            let end_idx = lower_bound(&ys, y_end);
            for y_idx in start_idx..end_idx {
                unsafe { crossings.get_unchecked_mut(y_idx) }.push(x);
            }
        } else {
            // Horizontal edge.
            let y = a[1];
            let x1 = a[0].min(b[0]);
            let x2 = a[0].max(b[0]);
            let row = lower_bound(&ys, y);
            unsafe { spans.get_unchecked_mut(row) }.push((x1, x2));
        }
    }

    // Resolve each scanline into merged intervals of green coverage.
    let mut merged_spans: Vec<Vec<(i32, i32)>> = Vec::with_capacity(height);
    for y_idx in 0..height {
        let mut row_spans = unsafe { std::mem::take(spans.get_unchecked_mut(y_idx)) };
        let mut cross = unsafe { std::mem::take(crossings.get_unchecked_mut(y_idx)) };
        if cross.len() % 2 != 0 {
            return Err("scanline intersections not even; loop malformed".into());
        }
        cross.sort_unstable();
        for pair in cross.chunks_exact(2) {
            row_spans.push((pair[0], pair[1]));
        }
        row_spans.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let mut merged: Vec<(i32, i32)> = Vec::new();
        for (start, end) in row_spans {
            if let Some(last) = merged.last_mut() {
                if start <= last.1 + 1 {
                    if end > last.1 {
                        last.1 = end;
                    }
                    continue;
                }
            }
            merged.push((start, end));
        }
        merged_spans.push(merged);
    }

    // Mark covered cells in a compressed grid.
    let mut green: Vec<Vec<u8>> = vec![vec![0u8; width]; height];
    for (y_idx, spans) in merged_spans.iter().enumerate() {
        for &(l, r) in spans {
            let xi_start = lower_bound(&xs, l);
            let xi_end = lower_bound(&xs, r + 1);
            for x_idx in xi_start..xi_end {
                unsafe { *green.get_unchecked_mut(y_idx).get_unchecked_mut(x_idx) = 1 };
            }
        }
    }

    let stride = width + 1;
    let mut prefix: Vec<u64> = vec![0u64; (height + 1) * stride];
    for y_idx in 0..height {
        let dy =
            (unsafe { ys.get_unchecked(y_idx + 1) } - unsafe { ys.get_unchecked(y_idx) }) as u64;
        let prev_row = y_idx * stride;
        let cur_row = (y_idx + 1) * stride;
        for x_idx in 0..width {
            let dx = (unsafe { xs.get_unchecked(x_idx + 1) } - unsafe { xs.get_unchecked(x_idx) })
                as u64;
            let cell_area = if unsafe { *green.get_unchecked(y_idx).get_unchecked(x_idx) } == 1 {
                dx * dy
            } else {
                0
            };
            let a = unsafe { *prefix.get_unchecked(cur_row + x_idx) };
            let b = unsafe { *prefix.get_unchecked(prev_row + x_idx + 1) };
            let c = unsafe { *prefix.get_unchecked(prev_row + x_idx) };
            unsafe {
                *prefix.get_unchecked_mut(cur_row + x_idx + 1) = a + b - c + cell_area;
            }
        }
    }

    Ok(Coverage {
        prefix,
        stride,
        xs,
        ys,
    })
}

/// Part 2: Find largest rectangle that is entirely covered by green/red tiles.
/// Uses precomputed O(1) index lookups instead of binary search.
#[inline(always)]
fn max_area_within_green(points: &[[i32; 2]], coverage: &Coverage) -> u64 {
    let n = points.len();
    if n < 2 {
        return 0;
    }

    // Precompute index lookups for each point's coordinates.
    let mut point_indices: Vec<(usize, usize, usize, usize)> = Vec::with_capacity(n);
    for &[x, y] in points {
        let xi = lower_bound(&coverage.xs, x);
        let xi1 = lower_bound(&coverage.xs, x + 1);
        let yi = lower_bound(&coverage.ys, y);
        let yi1 = lower_bound(&coverage.ys, y + 1);
        point_indices.push((xi, xi1, yi, yi1));
    }

    let mut max_area: u64 = 0;
    let stride = coverage.stride;
    let prefix = &coverage.prefix;

    for i in 0..n {
        let pi = unsafe { *points.get_unchecked(i) };
        let (xi1, xi1_plus, yi1, yi1_plus) = unsafe { *point_indices.get_unchecked(i) };

        for j in (i + 1)..n {
            let pj = unsafe { *points.get_unchecked(j) };
            let dx = (pi[0] - pj[0]).unsigned_abs() as u64 + 1;
            let dy = (pi[1] - pj[1]).unsigned_abs() as u64 + 1;
            let area = dx * dy;

            // Skip if this pair can't beat current max
            if area <= max_area {
                continue;
            }

            let (xj1, xj1_plus, yj1, yj1_plus) = unsafe { *point_indices.get_unchecked(j) };

            // Determine the rectangle bounds in index space
            let (xi_lo, xi_hi) = if pi[0] <= pj[0] {
                (xi1, xj1_plus)
            } else {
                (xj1, xi1_plus)
            };
            let (yi_lo, yi_hi) = if pi[1] <= pj[1] {
                (yi1, yj1_plus)
            } else {
                (yj1, yi1_plus)
            };

            // Compute green area using 2D prefix sum (O(1) lookup)
            let a = unsafe { *prefix.get_unchecked(yi_hi * stride + xi_hi) };
            let b = unsafe { *prefix.get_unchecked(yi_lo * stride + xi_hi) };
            let c = unsafe { *prefix.get_unchecked(yi_hi * stride + xi_lo) };
            let d = unsafe { *prefix.get_unchecked(yi_lo * stride + xi_lo) };
            let green_area = a + d - b - c;

            if green_area == area {
                max_area = area;
            }
        }
    }

    max_area
}

#[inline(always)]
fn lower_bound(values: &[i32], target: i32) -> usize {
    values.partition_point(|&v| v < target)
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

#[cfg(test)]
mod tests {
    use super::{both, max_area_any, max_area_within_green, parse_points};

    const EXAMPLE: &str = "7,1\n\
11,1\n\
11,7\n\
9,7\n\
9,5\n\
2,5\n\
2,3\n\
7,3\n";

    #[test]
    fn example_max_area() {
        let pts = parse_points(EXAMPLE).unwrap();
        assert_eq!(max_area_any(&pts), 50);
    }

    #[test]
    fn example_part2() {
        let pts = parse_points(EXAMPLE).unwrap();
        let cov = super::build_coverage(&pts).unwrap();
        assert_eq!(max_area_within_green(&pts, &cov), 24);
    }

    #[test]
    fn parse_handles_crlf_and_blanks() {
        let data = "1,2\r\n\r\n-3,4\n";
        let pts = parse_points(data).unwrap();
        assert_eq!(pts, vec![[1, 2], [-3, 4]]);
    }

    #[test]
    fn puzzle_input_runs() {
        // Should not error on provided puzzle input.
        let _ = both(super::INPUT).unwrap();
    }
}
