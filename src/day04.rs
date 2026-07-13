pub static INPUT: &str = include_str!("../inputs/04.txt");

/// Part 1: count rolls of paper (`@`) with fewer than four neighboring rolls in
/// the eight surrounding positions.
pub fn part1(input: &str) -> Result<usize, String> {
    solve_part1(input)
}

/// Part 2: repeatedly remove accessible rolls, updating neighbors as access
/// opens up.
pub fn part2(input: &str) -> Result<usize, String> {
    let (_, removed) = both(input)?;
    Ok(removed)
}

/// Solve both parts with a single parse and neighbor pass.
pub fn both(input: &str) -> Result<(usize, usize), String> {
    let mut grid = parse_grid(input)?;
    let mut counts = neighbor_counts(&grid);
    let mut queue = Vec::with_capacity(grid.data.len());
    let mut accessible = 0usize;

    for (idx, (&cell, &count)) in grid.data.iter().zip(&counts).enumerate() {
        if cell == 1 && count < 4 {
            accessible += 1;
            queue.push(idx);
        }
    }
    let mut head = 0;
    let mut removed = 0usize;

    while head < queue.len() {
        let idx = queue[head];
        head += 1;
        if grid.data[idx] == 0 || counts[idx] >= 4 {
            continue;
        }

        grid.data[idx] = 0;
        removed += 1;

        update_neighbors(idx, &mut grid, &mut counts, &mut queue);
    }

    Ok((accessible, removed))
}

#[derive(Clone)]
struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
    stride: usize,
}

fn solve_part1(input: &str) -> Result<usize, String> {
    let grid = parse_grid(input)?;
    let counts = neighbor_counts(&grid);
    Ok(counts
        .iter()
        .zip(&grid.data)
        .filter(|(&count, &cell)| cell == 1 && count < 4)
        .count())
}

fn parse_grid(input: &str) -> Result<Grid, String> {
    let mut width = None;
    let mut height = 0usize;
    let mut data = Vec::with_capacity(input.len() + 512);

    for raw_line in input.lines() {
        let line = raw_line.trim_end_matches('\r').as_bytes();
        if line.is_empty() {
            continue;
        }
        let expected = *width.get_or_insert(line.len());
        if line.len() != expected {
            return Err(format!(
                "inconsistent row width: expected {}, found {} on row {}",
                expected,
                line.len(),
                height + 1
            ));
        }
        if height == 0 {
            data.resize(expected + 2, 0);
        }

        data.push(0);
        for (col, &b) in line.iter().enumerate() {
            data.push(match b {
                b'.' => 0,
                b'@' => 1,
                other => {
                    return Err(format!(
                        "invalid character '{}' at line {} column {}",
                        other as char,
                        height + 1,
                        col + 1
                    ));
                }
            });
        }
        data.push(0);
        height += 1;
    }

    let Some(width) = width else {
        return Err("input is empty".into());
    };
    if height == 0 {
        return Err("input is empty".into());
    }
    let stride = width + 2;
    data.resize(data.len() + stride, 0);

    Ok(Grid {
        data,
        width,
        height,
        stride,
    })
}

fn neighbor_counts(grid: &Grid) -> Vec<u8> {
    let mut counts = vec![0u8; grid.data.len()];
    let w = grid.width;
    let h = grid.height;
    let stride = grid.stride;
    let data = &grid.data;

    for r in 1..=h {
        let base = r * stride;
        for c in 1..=w {
            let idx = base + c;
            if unsafe { *data.get_unchecked(idx) } == 0 {
                continue;
            }
            let up = idx - stride;
            let down = idx + stride;
            counts[idx] = unsafe {
                *data.get_unchecked(idx - 1)
                    + *data.get_unchecked(idx + 1)
                    + *data.get_unchecked(up - 1)
                    + *data.get_unchecked(up)
                    + *data.get_unchecked(up + 1)
                    + *data.get_unchecked(down - 1)
                    + *data.get_unchecked(down)
                    + *data.get_unchecked(down + 1)
            };
        }
    }

    counts
}

#[inline(always)]
fn update_neighbors(idx: usize, grid: &mut Grid, counts: &mut [u8], queue: &mut Vec<usize>) {
    let stride = grid.stride;

    let mut push_neighbor = |n_idx: usize| {
        if grid.data[n_idx] == 1 {
            let val = &mut counts[n_idx];
            if *val > 0 {
                *val -= 1;
            }
            if *val == 3 {
                queue.push(n_idx);
            }
        }
    };

    push_neighbor(idx - 1);
    push_neighbor(idx + 1);
    push_neighbor(idx - stride - 1);
    push_neighbor(idx - stride);
    push_neighbor(idx - stride + 1);
    push_neighbor(idx + stride - 1);
    push_neighbor(idx + stride);
    push_neighbor(idx + stride + 1);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn example_accessible_rolls() {
        assert_eq!(part1(EXAMPLE).unwrap(), 13);
    }

    #[test]
    fn example_total_removed() {
        assert_eq!(part2(EXAMPLE).unwrap(), 43);
    }
}
