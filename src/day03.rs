pub static INPUT: &str = include_str!("../inputs/03.txt");
const PART2_DIGITS: usize = 12;
const STACK_CAP: usize = 128;

#[derive(Clone)]
struct DigitStack {
    buf: [u8; STACK_CAP],
    len: usize,
}

impl DigitStack {
    #[inline(always)]
    const fn new() -> Self {
        Self {
            buf: [0; STACK_CAP],
            len: 0,
        }
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.len = 0;
    }
}

/// Part 1: for each line of digits, pick two in order to form the largest
/// possible two-digit number and sum those maxima across all lines.
pub fn part1(input: &str) -> Result<u64, String> {
    solve_single_pick::<2>(input)
}

/// Part 2: pick twelve digits (in order) per line to form the largest possible
/// 12-digit number and sum them.
pub fn part2(input: &str) -> Result<u64, String> {
    solve_single_pick::<PART2_DIGITS>(input)
}

/// Solve both parts in one pass over the input to avoid duplicate scanning.
pub fn both(input: &str) -> Result<(u64, u64), String> {
    let mut total_2: u64 = 0;
    let mut total_12: u64 = 0;
    let mut stack_2 = DigitStack::new();
    let mut stack_12 = DigitStack::new();
    let bytes = input.as_bytes();
    let mut line_idx = 0;
    let mut start = 0;

    for (idx, &b) in bytes.iter().enumerate() {
        if b == b'\n' {
            let mut end = idx;
            if end > start && bytes[end - 1] == b'\r' {
                end -= 1;
            }
            if end > start {
                let (v2, v12) = max_numbers_from_line(
                    &bytes[start..end],
                    line_idx,
                    &mut stack_2,
                    &mut stack_12,
                )?;
                total_2 += v2;
                total_12 += v12;
            }
            line_idx += 1;
            start = idx + 1;
        }
    }

    if start < bytes.len() {
        let mut end = bytes.len();
        if end > start && bytes[end - 1] == b'\r' {
            end -= 1;
        }
        if end > start {
            let (v2, v12) =
                max_numbers_from_line(&bytes[start..end], line_idx, &mut stack_2, &mut stack_12)?;
            total_2 += v2;
            total_12 += v12;
        }
    }

    Ok((total_2, total_12))
}

#[inline(always)]
fn max_numbers_from_line(
    line: &[u8],
    line_idx: usize,
    stack_2: &mut DigitStack,
    stack_12: &mut DigitStack,
) -> Result<(u64, u64), String> {
    if line.len() < 2 {
        return Err(format!(
            "line {} must contain at least {} digits, found {}",
            line_idx + 1,
            2,
            line.len()
        ));
    }
    if line.len() < PART2_DIGITS {
        return Err(format!(
            "line {} must contain at least {} digits, found {}",
            line_idx + 1,
            PART2_DIGITS,
            line.len()
        ));
    }

    let mut remove_2 = line.len() - 2;
    let mut remove_12 = line.len() - PART2_DIGITS;
    stack_2.reset();
    stack_12.reset();

    for &b in line {
        if !(b'0'..=b'9').contains(&b) {
            return Err(format!(
                "line {} contains non-digit character '{}'",
                line_idx + 1,
                b as char
            ));
        }
        let digit = b - b'0';

        while remove_2 > 0
            && stack_2.len > 0
            && unsafe { *stack_2.buf.get_unchecked(stack_2.len - 1) } < digit
        {
            stack_2.len -= 1;
            remove_2 -= 1;
        }
        unsafe { *stack_2.buf.get_unchecked_mut(stack_2.len) = digit };
        stack_2.len += 1;

        while remove_12 > 0
            && stack_12.len > 0
            && unsafe { *stack_12.buf.get_unchecked(stack_12.len - 1) } < digit
        {
            stack_12.len -= 1;
            remove_12 -= 1;
        }
        unsafe { *stack_12.buf.get_unchecked_mut(stack_12.len) = digit };
        stack_12.len += 1;
    }

    if remove_2 > 0 {
        stack_2.len -= remove_2;
    }
    if remove_12 > 0 {
        stack_12.len -= remove_12;
    }

    let value_2 = (unsafe { *stack_2.buf.get_unchecked(0) } as u64) * 10
        + unsafe { *stack_2.buf.get_unchecked(1) } as u64;

    let mut value_12: u64 = 0;
    for i in 0..stack_12.len {
        value_12 = value_12 * 10 + unsafe { *stack_12.buf.get_unchecked(i) } as u64;
    }

    Ok((value_2, value_12))
}

fn solve_single_pick<const PICK: usize>(input: &str) -> Result<u64, String> {
    let mut total: u64 = 0;
    let mut stack = DigitStack::new();
    let bytes = input.as_bytes();
    let mut line_idx = 0;
    let mut start = 0;

    for (idx, &b) in bytes.iter().enumerate() {
        if b == b'\n' {
            let mut end = idx;
            if end > start && bytes[end - 1] == b'\r' {
                end -= 1;
            }
            if end > start {
                total +=
                    max_number_from_line_pick::<PICK>(&bytes[start..end], line_idx, &mut stack)?;
            }
            line_idx += 1;
            start = idx + 1;
        }
    }

    if start < bytes.len() {
        let mut end = bytes.len();
        if end > start && bytes[end - 1] == b'\r' {
            end -= 1;
        }
        if end > start {
            total += max_number_from_line_pick::<PICK>(&bytes[start..end], line_idx, &mut stack)?;
        }
    }

    Ok(total)
}

#[inline(always)]
fn max_number_from_line_pick<const PICK: usize>(
    line: &[u8],
    line_idx: usize,
    stack: &mut DigitStack,
) -> Result<u64, String> {
    if line.len() < PICK {
        return Err(format!(
            "line {} must contain at least {} digits, found {}",
            line_idx + 1,
            PICK,
            line.len()
        ));
    }

    let mut remove = line.len() - PICK;
    stack.reset();

    for &b in line {
        if !(b'0'..=b'9').contains(&b) {
            return Err(format!(
                "line {} contains non-digit character '{}'",
                line_idx + 1,
                b as char
            ));
        }
        let digit = b - b'0';
        while remove > 0
            && stack.len > 0
            && unsafe { *stack.buf.get_unchecked(stack.len - 1) } < digit
        {
            stack.len -= 1;
            remove -= 1;
        }
        unsafe { *stack.buf.get_unchecked_mut(stack.len) = digit };
        stack.len += 1;
    }

    if remove > 0 {
        stack.len -= remove;
    }

    let mut value: u64 = 0;
    for i in 0..stack.len {
        value = value * 10 + unsafe { *stack.buf.get_unchecked(i) } as u64;
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn example_input_part1() {
        let input = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

        assert_eq!(part1(input).unwrap(), 357);
    }

    #[test]
    fn example_input_part2() {
        let input = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

        assert_eq!(part2(input).unwrap(), 3_121_910_778_619);
    }
}
