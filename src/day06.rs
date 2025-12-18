pub const INPUT: &str = include_str!("../inputs/06.txt");

/// Part 1: evaluate each vertical problem and sum their results.
pub fn part1(input: &str) -> Result<u128, String> {
    let (p1, _) = both(input)?;
    Ok(p1)
}

/// Part 2: columns are numbers read top-to-bottom, and problems read right-to-left.
pub fn part2(input: &str) -> Result<u128, String> {
    let (_, p2) = both(input)?;
    Ok(p2)
}

/// Compute both parts in a single pass to avoid double parsing.
pub fn both(input: &str) -> Result<(u128, u128), String> {
    let (lines, width) = parse_lines(input)?;
    let height = lines.len();
    let segments = find_segments(&lines, width)?;
    let op_row = height - 1;
    let mut total_row: u128 = 0;
    let mut total_col: u128 = 0;

    for (problem_idx, &(seg_start, seg_end)) in segments.iter().enumerate() {
        let op = operator_for_segment(&lines[op_row], seg_start, seg_end, problem_idx + 1)?;

        let mut result_row = if op == b'+' { 0 } else { 1 };
        for row in 0..op_row {
            let val = parse_row_number(&lines[row], seg_start, seg_end, problem_idx + 1, row + 1)?;
            if op == b'+' {
                result_row += val;
            } else {
                result_row *= val;
            }
        }

        let mut result_col = if op == b'+' { 0 } else { 1 };
        for col in (seg_start..seg_end).rev() {
            let mut value: u128 = 0;
            let mut found = false;
            for row in 0..op_row {
                let ch = unsafe { *lines[row].get_unchecked(col) };
                if ch == b' ' {
                    continue;
                }
                if ch < b'0' || ch > b'9' {
                    return Err(format!(
                        "non-digit character '{}' in problem {} at row {} column {}",
                        ch as char,
                        problem_idx + 1,
                        row + 1,
                        col + 1
                    ));
                }
                value = value * 10 + (ch - b'0') as u128;
                found = true;
            }
            if found {
                if op == b'+' {
                    result_col += value;
                } else {
                    result_col *= value;
                }
            }
        }

        total_row += result_row;
        total_col += result_col;
    }

    Ok((total_row, total_col))
}

fn parse_lines(input: &str) -> Result<(Vec<Vec<u8>>, usize), String> {
    let mut lines: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.trim_end_matches('\r').as_bytes().to_vec())
        .collect();

    let height = lines.len();
    if height < 2 {
        return Err(
            "input must contain at least one row of numbers and one row of operators".into(),
        );
    }

    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    if width == 0 {
        return Err("input is empty".into());
    }

    for line in &mut lines {
        if line.len() < width {
            line.resize(width, b' ');
        }
    }

    Ok((lines, width))
}

fn find_segments(lines: &[Vec<u8>], width: usize) -> Result<Vec<(usize, usize)>, String> {
    let mut blank_cols = vec![true; width];
    for line in lines {
        for col in 0..width {
            let ch = unsafe { *line.get_unchecked(col) };
            if ch != b' ' {
                blank_cols[col] = false;
            }
        }
    }

    let mut segments = Vec::new();
    let mut in_segment = false;
    let mut start = 0usize;

    for col in 0..width {
        if blank_cols[col] {
            if in_segment {
                segments.push((start, col));
                in_segment = false;
            }
        } else if !in_segment {
            start = col;
            in_segment = true;
        }
    }
    if in_segment {
        segments.push((start, width));
    }

    if segments.is_empty() {
        return Err("input contained no problems".into());
    }

    Ok(segments)
}

fn operator_for_segment(
    op_line: &[u8],
    start: usize,
    end: usize,
    problem_idx: usize,
) -> Result<u8, String> {
    let mut op = None;
    for col in start..end {
        let ch = unsafe { *op_line.get_unchecked(col) };
        if ch != b' ' {
            if op.is_some() {
                return Err(format!(
                    "multiple operator characters found for problem {}",
                    problem_idx
                ));
            }
            op = Some(ch);
        }
    }
    let op = op.ok_or_else(|| format!("missing operator for problem {}", problem_idx))?;
    Ok(op)
}

#[inline]
fn parse_row_number(
    line: &[u8],
    start: usize,
    end: usize,
    problem_idx: usize,
    row_idx: usize,
) -> Result<u128, String> {
    let mut lo = start;
    while lo < end && unsafe { *line.get_unchecked(lo) } == b' ' {
        lo += 1;
    }
    let mut hi = end;
    while hi > lo && unsafe { *line.get_unchecked(hi - 1) } == b' ' {
        hi -= 1;
    }
    if lo == hi {
        return Err(format!(
            "missing number for problem {} on row {}",
            problem_idx, row_idx
        ));
    }

    let mut value: u128 = 0;
    for idx in lo..hi {
        let b = unsafe { *line.get_unchecked(idx) };
        if !(b'0'..=b'9').contains(&b) {
            return Err(format!(
                "non-digit character '{}' in problem {} on row {}",
                b as char, problem_idx, row_idx
            ));
        }
        value = value * 10 + (b - b'0') as u128;
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE).unwrap(), 4_277_556);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE).unwrap(), 3_263_827);
    }
}
