pub static INPUT: &str = include_str!("../inputs/05.txt");

/// Part 1: count available ingredient IDs that fall within any fresh range.
pub fn part1(input: &str) -> Result<usize, String> {
    let (ranges, ids) = parse_input(input)?;
    let merged = merge_ranges(ranges);

    let mut fresh = 0usize;
    for id in ids {
        fresh += is_fresh(id, &merged) as usize;
    }

    Ok(fresh)
}

/// Part 2: count how many IDs are fresh across all ranges (size of the union).
pub fn part2(input: &str) -> Result<u128, String> {
    let ranges = parse_ranges(input)?;
    let merged = merge_ranges(ranges);

    let mut total: u128 = 0;
    for (start, end) in merged {
        total += (end - start + 1) as u128;
    }

    Ok(total)
}

/// Solve both parts with a single parse and merge.
pub fn both(input: &str) -> Result<(usize, u128), String> {
    let (ranges, ids) = parse_input(input)?;
    let merged = merge_ranges(ranges);

    let mut fresh = 0usize;
    for id in ids {
        fresh += is_fresh(id, &merged) as usize;
    }

    let mut total: u128 = 0;
    for (start, end) in merged {
        total += (end - start + 1) as u128;
    }

    Ok((fresh, total))
}

fn parse_input(input: &str) -> Result<(Vec<(u64, u64)>, Vec<u64>), String> {
    let bytes = input.as_bytes();
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut line_start = 0usize;
    let mut line_idx = 0usize;
    let mut in_ids = false;
    let mut i = 0usize;

    while i <= bytes.len() {
        if i == bytes.len() || bytes[i] == b'\n' {
            let mut end = i;
            if end > line_start && bytes[end - 1] == b'\r' {
                end -= 1;
            }

            if end == line_start {
                in_ids = true;
            } else {
                let line = &bytes[line_start..end];
                if !in_ids {
                    let hyphen = line
                        .iter()
                        .position(|&b| b == b'-')
                        .ok_or_else(|| format!("missing '-' on line {}", line_idx + 1))?;
                    let start = parse_number(&line[..hyphen], line_idx)?;
                    let end_num = parse_number(&line[hyphen + 1..], line_idx)?;
                    if start > end_num {
                        return Err(format!(
                            "range start {} exceeds end {} on line {}",
                            start,
                            end_num,
                            line_idx + 1
                        ));
                    }
                    ranges.push((start, end_num));
                } else {
                    let id = parse_number(line, line_idx)?;
                    ids.push(id);
                }
            }

            line_idx += 1;
            line_start = i + 1;
        }
        i += 1;
    }

    if ranges.is_empty() {
        return Err("input contained no ranges".to_string());
    }
    if ids.is_empty() {
        return Err("input contained no ingredient ids".to_string());
    }

    Ok((ranges, ids))
}

fn parse_ranges(input: &str) -> Result<Vec<(u64, u64)>, String> {
    let bytes = input.as_bytes();
    let mut ranges = Vec::new();
    let mut line_start = 0usize;
    let mut line_idx = 0usize;
    let mut i = 0usize;

    while i <= bytes.len() {
        if i == bytes.len() || bytes[i] == b'\n' {
            let mut end = i;
            if end > line_start && bytes[end - 1] == b'\r' {
                end -= 1;
            }

            if end == line_start {
                break;
            }

            let line = &bytes[line_start..end];
            let hyphen = line
                .iter()
                .position(|&b| b == b'-')
                .ok_or_else(|| format!("missing '-' on line {}", line_idx + 1))?;
            let start = parse_number(&line[..hyphen], line_idx)?;
            let end_num = parse_number(&line[hyphen + 1..], line_idx)?;
            if start > end_num {
                return Err(format!(
                    "range start {} exceeds end {} on line {}",
                    start,
                    end_num,
                    line_idx + 1
                ));
            }
            ranges.push((start, end_num));

            line_idx += 1;
            line_start = i + 1;
        }
        i += 1;
    }

    if ranges.is_empty() {
        return Err("input contained no ranges".to_string());
    }

    Ok(ranges)
}

#[inline(always)]
fn parse_number(bytes: &[u8], line_idx: usize) -> Result<u64, String> {
    if bytes.is_empty() {
        return Err(format!("missing number on line {}", line_idx + 1));
    }
    let mut value: u64 = 0;
    for &b in bytes {
        if !(b'0'..=b'9').contains(&b) {
            return Err(format!(
                "invalid character '{}' on line {}",
                b as char,
                line_idx + 1
            ));
        }
        value = value * 10 + (b - b'0') as u64;
    }
    Ok(value)
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.len() <= 1 {
        return ranges;
    }

    ranges.sort_unstable_by_key(|&(start, _)| start);
    let mut merged = Vec::with_capacity(ranges.len());
    let mut current = ranges[0];

    for &(start, end) in ranges.iter().skip(1) {
        if start <= current.1 + 1 {
            if end > current.1 {
                current.1 = end;
            }
        } else {
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    merged
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    let idx = ranges.partition_point(|&(start, _)| start <= id);
    if idx == 0 {
        return false;
    }
    let (_, end) = unsafe { *ranges.get_unchecked(idx - 1) };
    id <= end
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn example_counts_fresh_ids() {
        assert_eq!(part1(EXAMPLE).unwrap(), 3);
    }

    #[test]
    fn example_counts_total_fresh_space() {
        assert_eq!(part2(EXAMPLE).unwrap(), 14);
    }
}
