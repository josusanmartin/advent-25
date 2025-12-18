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
    let bytes = input.as_bytes();
    let mut data = Vec::with_capacity(bytes.len());
    let mut width = 0usize;
    let mut height = 0usize;
    let mut col = 0usize;

    for &b in bytes {
        match b {
            b'\n' => {
                if col == 0 {
                    continue;
                }
                if width == 0 {
                    width = col;
                } else if col != width {
                    return Err(format!(
                        "inconsistent row width: expected {}, found {} on row {}",
                        width,
                        col,
                        height + 1
                    ));
                }
                height += 1;
                col = 0;
            }
            b'\r' => {}
            b'.' => {
                data.push(0);
                col += 1;
            }
            b'@' => {
                data.push(1);
                col += 1;
            }
            other => {
                return Err(format!(
                    "invalid character '{}' at line {} column {}",
                    other as char,
                    height + 1,
                    col + 1
                ));
            }
        }
    }

    if col > 0 {
        if width == 0 {
            width = col;
        } else if col != width {
            return Err(format!(
                "inconsistent row width: expected {}, found {} on row {}",
                width,
                col,
                height + 1
            ));
        }
        height += 1;
    }

    if width == 0 || height == 0 {
        return Err("input is empty".into());
    }

    Ok(Grid {
        data,
        width,
        height,
    })
}

fn neighbor_counts(grid: &Grid) -> Vec<u8> {
    let mut counts = vec![0u8; grid.data.len()];
    let w = grid.width;
    let h = grid.height;
    let data = &grid.data;

    for r in 0..h {
        let base = r * w;
        let has_up = r > 0;
        let has_down = r + 1 < h;
        for c in 0..w {
            let idx = base + c;
            if data[idx] == 0 {
                continue;
            }
            let mut total = 0u8;
            if c > 0 {
                total += data[idx - 1];
            }
            if c + 1 < w {
                total += data[idx + 1];
            }
            if has_up {
                let up = idx - w;
                total += data[up];
                if c > 0 {
                    total += data[up - 1];
                }
                if c + 1 < w {
                    total += data[up + 1];
                }
            }
            if has_down {
                let down = idx + w;
                total += data[down];
                if c > 0 {
                    total += data[down - 1];
                }
                if c + 1 < w {
                    total += data[down + 1];
                }
            }
            counts[idx] = total;
        }
    }

    counts
}

#[inline(always)]
fn update_neighbors(idx: usize, grid: &mut Grid, counts: &mut [u8], queue: &mut Vec<usize>) {
    let w = grid.width;
    let h = grid.height;
    let r = idx / w;
    let c = idx - r * w;

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

    if c > 0 {
        push_neighbor(idx - 1);
        if r > 0 {
            push_neighbor(idx - w - 1);
        }
        if r + 1 < h {
            push_neighbor(idx + w - 1);
        }
    }
    if c + 1 < w {
        push_neighbor(idx + 1);
        if r > 0 {
            push_neighbor(idx - w + 1);
        }
        if r + 1 < h {
            push_neighbor(idx + w + 1);
        }
    }
    if r > 0 {
        push_neighbor(idx - w);
    }
    if r + 1 < h {
        push_neighbor(idx + w);
    }
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
