pub static INPUT: &str = include_str!("../inputs/01.txt");
/// Count how many times the dial points at zero after processing all rotations.
///
/// Assumes trusted input: each line is `L`/`R` followed by digits, optional `\r`, ending with `\n`.
#[inline(always)]
pub fn part1(input: &str) -> Result<usize, String> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut idx = 0;
    let mut position: u16 = 50;
    let mut zero_hits: usize = 0;

    // Tight single-pass scanner with unchecked indexing and branchless direction math.
    while idx < len {
        // Skip blank lines and newlines.
        while idx < len {
            let b = unsafe { *bytes.get_unchecked(idx) };
            if b != b'\n' && b != b'\r' {
                break;
            }
            idx += 1;
        }
        if idx >= len {
            break;
        }

        let dir = unsafe { *bytes.get_unchecked(idx) };
        idx += 1;

        let mut dist: u16 = 0;
        while idx < len {
            let b = unsafe { *bytes.get_unchecked(idx) };
            idx += 1;
            if b == b'\n' {
                break;
            }
            if b == b'\r' {
                continue;
            }
            dist = ((dist << 3) + (dist << 1) + (b - b'0') as u16) % 100;
        }

        let add = (dir == b'R') as u16 * dist;
        let sub = (dir == b'L') as u16 * dist;
        let mut pos = position + add + 100 - sub;
        if pos >= 200 {
            pos -= 200;
        } else if pos >= 100 {
            pos -= 100;
        }
        position = pos;

        zero_hits += (position == 0) as usize;
    }

    Ok(zero_hits)
}

/// Count how many times any click (including in-flight) lands on zero.
///
/// Assumes trusted input: each line is `L`/`R` followed by digits, optional `\r`, ending with `\n`.
pub fn part2(input: &str) -> Result<usize, String> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut idx = 0;
    let mut position: u16 = 50;
    let mut zero_hits: usize = 0;

    while idx < len {
        while idx < len {
            let b = unsafe { *bytes.get_unchecked(idx) };
            if b != b'\n' && b != b'\r' {
                break;
            }
            idx += 1;
        }
        if idx >= len {
            break;
        }

        let dir = unsafe { *bytes.get_unchecked(idx) };
        idx += 1;

        let mut dist_mod: u16 = 0;
        let mut dist_full: u64 = 0;
        while idx < len {
            let b = unsafe { *bytes.get_unchecked(idx) };
            idx += 1;
            if b == b'\n' {
                break;
            }
            if b == b'\r' {
                continue;
            }
            let digit = (b - b'0') as u16;
            dist_mod = ((dist_mod as u32 * 10 + digit as u32) % 100) as u16;
            dist_full = dist_full * 10 + digit as u64;
        }

        // Count zero hits during the rotation (including if it ends on zero).
        let first_hit = if dir == b'R' {
            if position == 0 {
                100
            } else {
                100 - position as u64
            }
        } else {
            if position == 0 {
                100
            } else {
                position as u64
            }
        };
        if dist_full >= first_hit {
            zero_hits += 1 + ((dist_full - first_hit) / 100) as usize;
        }

        let add = (dir == b'R') as u16 * dist_mod;
        let sub = (dir == b'L') as u16 * dist_mod;
        let mut pos = position + add + 100 - sub;
        if pos >= 200 {
            pos -= 200;
        } else if pos >= 100 {
            pos -= 100;
        }
        position = pos;
    }

    Ok(zero_hits)
}

/// Convenience helper that runs part 1 against the bundled puzzle input file.
pub fn part1_puzzle() -> Result<usize, String> {
    part1(INPUT)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn example_input() {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

        assert_eq!(part1(input).unwrap(), 3);
        assert_eq!(part2(input).unwrap(), 6);
    }
}
