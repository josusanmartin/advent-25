pub static INPUT: &str = include_str!("../inputs/07.txt");

/// Part 1: count how many splitters are activated by at least one beam.
pub fn part1(input: &str) -> Result<u128, String> {
    let (splits, _) = simulate(input)?;
    Ok(splits)
}

/// Part 2: count total timelines after all quantum splits.
pub fn part2(input: &str) -> Result<u128, String> {
    let (_, timelines) = simulate(input)?;
    Ok(timelines)
}

/// Solve both parts in a single pass to avoid duplicate parsing.
pub fn both(input: &str) -> Result<(u128, u128), String> {
    let (splits, timelines) = simulate(input)?;
    Ok((splits, timelines))
}

fn simulate(input: &str) -> Result<(u128, u128), String> {
    let (rows, start_row, start_col) = parse_grid(input)?;
    let width = rows[0].len();
    let mut current_counts = vec![0u128; width];
    let mut next_counts = vec![0u128; width];
    current_counts[start_col] = 1;
    let mut splitters_hit: u128 = 0;
    let mut timelines: u128 = 1;

    for line in rows.iter().skip(start_row + 1) {
        next_counts.fill(0);
        for col in 0..width {
            let count = unsafe { *current_counts.get_unchecked(col) };
            if count == 0 {
                continue;
            }
            let ch = unsafe { *line.get_unchecked(col) };
            if ch == b'^' {
                splitters_hit += 1;
                timelines += count;
                if col > 0 {
                    unsafe {
                        *next_counts.get_unchecked_mut(col - 1) += count;
                    }
                }
                if col + 1 < width {
                    unsafe {
                        *next_counts.get_unchecked_mut(col + 1) += count;
                    }
                }
            } else {
                unsafe { *next_counts.get_unchecked_mut(col) += count };
            }
        }
        std::mem::swap(&mut current_counts, &mut next_counts);
    }

    Ok((splitters_hit, timelines))
}

fn parse_grid(input: &str) -> Result<(Vec<&[u8]>, usize, usize), String> {
    let mut rows: Vec<&[u8]> = Vec::new();
    let mut start_row = None;
    let mut start_col = 0usize;
    let mut expected_width = None;

    for (row_idx, line) in input.lines().enumerate() {
        let mut bytes = line.as_bytes();
        if let Some(&b'\r') = bytes.last() {
            bytes = &bytes[..bytes.len() - 1];
        }
        if bytes.is_empty() {
            return Err(format!("line {} is empty", row_idx + 1));
        }

        if let Some(width) = expected_width {
            if bytes.len() != width {
                return Err(format!(
                    "line {} width {} does not match expected {}",
                    row_idx + 1,
                    bytes.len(),
                    width
                ));
            }
        } else {
            expected_width = Some(bytes.len());
        }

        if let Some(pos) = bytes.iter().position(|&b| b == b'S') {
            if start_row.is_some() {
                return Err(format!(
                    "multiple starting positions found (latest at row {}, col {})",
                    row_idx + 1,
                    pos + 1
                ));
            }
            start_row = Some(row_idx);
            start_col = pos;
        }

        rows.push(bytes);
    }

    let start_row = start_row.ok_or_else(|| "missing starting position 'S'".to_string())?;
    Ok((rows, start_row, start_col))
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &str = ".......S.......\n\
...............\n\
.......^.......\n\
...............\n\
......^.^......\n\
...............\n\
.....^.^.^.....\n\
...............\n\
....^.^...^....\n\
...............\n\
...^.^...^.^...\n\
...............\n\
..^...^.....^..\n\
...............\n\
.^.^.^.^.^...^.\n\
...............\n";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE).unwrap(), 21);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE).unwrap(), 40);
    }
}
