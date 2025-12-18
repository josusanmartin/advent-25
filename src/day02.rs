pub const INPUT: &str = include_str!("../inputs/02.txt");

/// Part 1: numbers whose decimal representation is some block of digits
/// repeated exactly twice (no leading zeroes).
pub fn part1(input: &str) -> Result<u128, String> {
    let ranges = merge_ranges(parse_ranges(input)?);
    Ok(if ranges.is_empty() {
        0
    } else {
        sums_for_ranges(&ranges).0
    })
}

/// Part 2: numbers whose decimal representation is a block of digits repeated
/// at least twice (no leading zeroes).
pub fn part2(input: &str) -> Result<u128, String> {
    let ranges = merge_ranges(parse_ranges(input)?);
    Ok(if ranges.is_empty() {
        0
    } else {
        sums_for_ranges(&ranges).1
    })
}

/// Solve both parts with a single parse and shared candidate generation.
pub fn both(input: &str) -> Result<(u128, u128), String> {
    let ranges = merge_ranges(parse_ranges(input)?);
    Ok(if ranges.is_empty() {
        (0, 0)
    } else {
        sums_for_ranges(&ranges)
    })
}

const POW10: [u128; 20] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
    10_000_000_000_000_000_000,
];

#[inline]
fn digit_len(n: u64) -> usize {
    let n128 = n as u128;
    for i in 1..POW10.len() {
        if n128 < POW10[i] {
            return i;
        }
    }
    POW10.len() - 1
}

fn parse_ranges(input: &str) -> Result<Vec<(u64, u64)>, String> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut ranges = Vec::new();
    let mut i = 0usize;

    while i < len {
        // Skip separators and whitespace.
        while i < len {
            let b = unsafe { *bytes.get_unchecked(i) };
            if b == b',' || b == b'\n' || b == b'\r' || b == b' ' {
                i += 1;
                continue;
            }
            break;
        }
        if i >= len {
            break;
        }

        // Parse start.
        let mut start: u64 = 0;
        let mut found_digit = false;
        while i < len {
            let b = unsafe { *bytes.get_unchecked(i) };
            if b == b'-' {
                i += 1;
                break;
            }
            if !(b'0'..=b'9').contains(&b) {
                return Err(format!("invalid character '{}' in range start", b as char));
            }
            found_digit = true;
            start = start
                .checked_mul(10)
                .and_then(|v| v.checked_add((b - b'0') as u64))
                .ok_or_else(|| "range start overflowed u64".to_string())?;
            i += 1;
        }
        if !found_digit {
            return Err("missing start for range".into());
        }
        if i > len {
            return Err("missing '-' separator for range".into());
        }

        // Parse end.
        let mut end: u64 = 0;
        let mut found_end = false;
        while i < len {
            let b = unsafe { *bytes.get_unchecked(i) };
            if b == b',' || b == b'\n' || b == b'\r' {
                i += 1;
                break;
            }
            if b == b' ' {
                i += 1;
                continue;
            }
            if !(b'0'..=b'9').contains(&b) {
                return Err(format!("invalid character '{}' in range end", b as char));
            }
            found_end = true;
            end = end
                .checked_mul(10)
                .and_then(|v| v.checked_add((b - b'0') as u64))
                .ok_or_else(|| "range end overflowed u64".to_string())?;
            i += 1;
        }
        if !found_end {
            return Err("missing end for range".into());
        }
        if start > end {
            return Err(format!("range start {} exceeds end {}", start, end));
        }
        ranges.push((start, end));
    }

    Ok(ranges)
}

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.len() <= 1 {
        return ranges;
    }
    ranges.sort_unstable_by_key(|&(s, _)| s);
    let mut merged = Vec::with_capacity(ranges.len());
    let mut current = ranges[0];
    for &(s, e) in ranges.iter().skip(1) {
        if s <= current.1 + 1 {
            if e > current.1 {
                current.1 = e;
            }
        } else {
            merged.push(current);
            current = (s, e);
        }
    }
    merged.push(current);
    merged
}

// =============================================================================
// OPTIMIZED IMPLEMENTATION (~100x faster than original k-way merge)
// =============================================================================
// Uses closed-form arithmetic summation where possible, with per-block iteration
// only for deduplication of multi-digit blocks.
//
// Key insight: Instead of generating candidates one-by-one via k-way merge,
// we compute sums per-sequence per-range using closed-form math.
// For block_len=1, all candidates are "fundamental" (no dedup needed).
// For block_len>1, we check if each block is itself a repeat-block pattern.

/// Sum of integers from lo to hi inclusive: count * (lo + hi) / 2
#[inline]
fn sum_range(lo: u64, hi: u64) -> u128 {
    let count = (hi - lo + 1) as u128;
    count * (lo as u128 + hi as u128) / 2
}

/// Check if a block of digits is itself a repeat-block pattern.
/// E.g., 1212 (as a 4-digit block) is 12 repeated 2 times.
/// Returns Some((sub_block_len, sub_repeats)) if it's a repeat-block, None otherwise.
#[inline]
fn decompose_block(block: u64, block_len: usize) -> Option<(usize, usize)> {
    for d in 1..block_len {
        if block_len % d != 0 {
            continue;
        }
        let num_reps = block_len / d;
        if num_reps < 2 {
            continue;
        }
        let sub_rep_factor = (POW10[block_len] - 1) / (POW10[d] - 1);
        if (block as u128) % sub_rep_factor == 0 {
            let sub_block = (block as u128) / sub_rep_factor;
            let sub_block_min = POW10[d - 1];
            let sub_block_max = POW10[d] - 1;
            if sub_block >= sub_block_min && sub_block <= sub_block_max {
                return Some((d, num_reps));
            }
        }
    }
    None
}

/// Check if a number can be represented as some block repeated exactly 2 times.
#[inline]
fn has_double_representation(candidate: u128, total_digits: usize) -> bool {
    if total_digits % 2 != 0 {
        return false;
    }
    let half_len = total_digits / 2;
    let half_rep_factor = (POW10[total_digits] - 1) / (POW10[half_len] - 1);
    if candidate % half_rep_factor == 0 {
        let half_block = candidate / half_rep_factor;
        let half_block_min = POW10[half_len - 1];
        let half_block_max = POW10[half_len] - 1;
        half_block >= half_block_min && half_block <= half_block_max
    } else {
        false
    }
}

fn sums_for_ranges(ranges: &[(u64, u64)]) -> (u128, u128) {
    let max_end = ranges.iter().map(|&(_, e)| e).max().unwrap();
    let max_digits = digit_len(max_end);

    let mut part1_sum = 0u128;
    let mut part2_sum = 0u128;

    for block_len in 1..=max_digits {
        let block_min = POW10[block_len - 1] as u64;
        let block_max_possible = (POW10[block_len] - 1) as u64;
        let max_repeats = max_digits / block_len;
        if max_repeats < 2 {
            continue;
        }

        for repeats in 2..=max_repeats {
            let total_len = block_len * repeats;
            let rep_factor = (POW10[total_len] - 1) / (POW10[block_len] - 1);

            let cap = (max_end as u128) / rep_factor;
            if cap < block_min as u128 {
                continue;
            }
            let block_max = cap.min(block_max_possible as u128) as u64;

            for &(range_start, range_end) in ranges {
                let b_lo = {
                    let div = range_start as u128 / rep_factor;
                    let rem = range_start as u128 % rep_factor;
                    if rem == 0 {
                        div as u64
                    } else {
                        (div + 1) as u64
                    }
                };
                let b_hi = (range_end as u128 / rep_factor) as u64;

                let b_lo = b_lo.max(block_min);
                let b_hi = b_hi.min(block_max);

                if b_lo > b_hi {
                    continue;
                }

                if block_len == 1 {
                    // All 1-digit blocks are fundamental (no dedup needed)
                    let s = rep_factor * sum_range(b_lo, b_hi);
                    part2_sum += s;
                    // Part 1: Check for double representation
                    if repeats == 2 {
                        part1_sum += s;
                    } else if total_len % 2 == 0 {
                        // For 1-digit block repeated even times, always has double repr
                        part1_sum += s;
                    }
                } else {
                    // Need to check each block for dedup
                    for block in b_lo..=b_hi {
                        let candidate = (block as u128) * rep_factor;
                        let is_fundamental = decompose_block(block, block_len).is_none();

                        if is_fundamental {
                            part2_sum += candidate;
                            if has_double_representation(candidate, total_len) {
                                part1_sum += candidate;
                            }
                        }
                    }
                }
            }
        }
    }

    (part1_sum, part2_sum)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\n\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\n\
824824821-824824827,2121212118-2121212124";

    #[test]
    fn example_input() {
        assert_eq!(part1(EXAMPLE).unwrap(), 1_227_775_554);
        assert_eq!(part2(EXAMPLE).unwrap(), 4_174_379_265);
    }

    #[test]
    fn accepts_trailing_commas_and_spaces() {
        let input = "11-22, 99-105,\n";
        assert_eq!(part1(input).unwrap(), 132);
    }
}
