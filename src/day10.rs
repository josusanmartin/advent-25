pub static INPUT: &str = include_str!("../inputs/10.txt");

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Parsed machine data.
struct Machine {
    target: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
    n: usize,
}

pub fn part1(input: &str) -> Result<u64, String> {
    let machines = parse(input)?;
    Ok(machines.iter().map(|m| solve_lights(m) as u64).sum())
}

pub fn part2(input: &str) -> Result<u64, String> {
    let machines = parse(input)?;
    Ok(machines.iter().map(|m| solve_joltage(m) as u64).sum())
}

#[cfg(not(feature = "parallel"))]
pub fn both(input: &str) -> Result<(u64, u64), String> {
    let machines = parse(input)?;
    let mut p1 = 0u64;
    let mut p2 = 0u64;
    for m in &machines {
        p1 += solve_lights(m) as u64;
        p2 += solve_joltage(m) as u64;
    }
    Ok((p1, p2))
}

#[cfg(feature = "parallel")]
pub fn both(input: &str) -> Result<(u64, u64), String> {
    let machines = parse(input)?;
    let (p1, p2) = machines
        .par_iter()
        .map(|m| (solve_lights(m) as u64, solve_joltage(m) as u64))
        .reduce(|| (0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2));
    Ok((p1, p2))
}

// =============================================================================
// Part 1: Lights Out over GF(2) - optimized with u16 bitmasks
// =============================================================================

fn solve_lights(m: &Machine) -> u32 {
    if m.target == 0 {
        return 0;
    }
    let num_buttons = m.buttons.len();
    if num_buttons == 0 {
        return u32::MAX;
    }

    // Build matrix: mat[light] = bitmask of buttons that affect it.
    let mut mat = [0u16; 16];
    for light in 0..m.n {
        for (btn_idx, &btn) in m.buttons.iter().enumerate() {
            if (btn >> light) & 1 == 1 {
                mat[light] |= 1 << btn_idx;
            }
        }
    }

    // Gaussian elimination over GF(2).
    let mut target = m.target;
    let mut pivot_mask = 0u16;
    let mut rank = 0;

    for col in 0..num_buttons {
        let col_bit = 1u16 << col;
        let mut pivot = None;
        for row in rank..m.n {
            if mat[row] & col_bit != 0 {
                pivot = Some(row);
                break;
            }
        }
        let Some(prow) = pivot else { continue };

        mat.swap(rank, prow);
        // Swap target bits.
        let t_rank = (target >> rank) & 1;
        let t_prow = (target >> prow) & 1;
        target = (target & !(1 << rank) & !(1 << prow)) | (t_prow << rank) | (t_rank << prow);

        let pivot_row = mat[rank];

        for row in 0..m.n {
            if row != rank && mat[row] & col_bit != 0 {
                mat[row] ^= pivot_row;
                let t1 = (target >> row) & 1;
                let t2 = (target >> rank) & 1;
                target = (target & !(1 << row)) | ((t1 ^ t2) << row);
            }
        }

        pivot_mask |= col_bit;
        rank += 1;
    }

    let num_free = num_buttons - rank;
    let free_mask = !pivot_mask & ((1u16 << num_buttons) - 1);

    let mut best = u32::MAX;
    for free_bits in 0u16..(1 << num_free) {
        let mut solution = 0u16;
        let mut fi = 0;
        for col in 0..num_buttons {
            if free_mask & (1 << col) != 0 {
                if (free_bits >> fi) & 1 == 1 {
                    solution |= 1 << col;
                }
                fi += 1;
            }
        }

        for row in (0..rank).rev() {
            let row_val = mat[row];
            let pivot_col = row_val.trailing_zeros() as usize;
            let rhs = (target >> row) & 1;
            let other_bits = row_val ^ (1 << pivot_col);
            let xor_sum = (other_bits & solution).count_ones() & 1;
            if (rhs as u32) ^ xor_sum == 1 {
                solution |= 1 << pivot_col;
            }
        }

        let cost = solution.count_ones();
        if cost < best {
            best = cost;
        }
    }

    best
}

// =============================================================================
// Part 2: Integer counter - iterative search
// =============================================================================

fn solve_joltage(m: &Machine) -> u32 {
    if m.joltages.iter().all(|&v| v == 0) {
        return 0;
    }
    let num_buttons = m.buttons.len();
    if num_buttons == 0 {
        return u32::MAX;
    }

    // Compute upper bounds per button.
    let mut bounds = [u16::MAX; 16];
    for (col, &btn) in m.buttons.iter().enumerate() {
        for row in 0..m.n {
            if (btn >> row) & 1 == 1 {
                bounds[col] = bounds[col].min(m.joltages[row]);
            }
        }
    }

    // Build augmented matrix.
    let mut mat = [[0i32; 17]; 16];
    for row in 0..m.n {
        for (col, &btn) in m.buttons.iter().enumerate() {
            if (btn >> row) & 1 == 1 {
                mat[row][col] = 1;
            }
        }
        mat[row][num_buttons] = m.joltages[row] as i32;
    }

    // Gaussian elimination.
    let mut pivot_col_for_row = [usize::MAX; 16];
    let mut pivot_row = 0;

    for col in 0..num_buttons {
        let mut found = None;
        for r in pivot_row..m.n {
            if mat[r][col] != 0 {
                found = Some(r);
                break;
            }
        }
        let Some(pr) = found else { continue };

        mat.swap(pivot_row, pr);
        pivot_col_for_row[pivot_row] = col;
        let pivot_val = mat[pivot_row][col];

        for r in 0..m.n {
            if r == pivot_row || mat[r][col] == 0 {
                continue;
            }
            let factor = mat[r][col];
            for c in 0..=num_buttons {
                mat[r][c] = mat[r][c] * pivot_val - factor * mat[pivot_row][c];
            }
            let mut g = 0i32;
            for c in 0..=num_buttons {
                g = gcd(g, mat[r][c].abs());
            }
            if g > 1 {
                for c in 0..=num_buttons {
                    mat[r][c] /= g;
                }
            }
        }

        pivot_row += 1;
    }

    let rank = pivot_row;

    // Collect free columns sorted by bound.
    let mut is_pivot = [false; 16];
    for r in 0..rank {
        is_pivot[pivot_col_for_row[r]] = true;
    }

    let mut free_data = [(0usize, 0u16); 16];
    let mut num_free = 0;
    for col in 0..num_buttons {
        if !is_pivot[col] {
            free_data[num_free] = (col, bounds[col]);
            num_free += 1;
        }
    }
    free_data[..num_free].sort_by_key(|&(_, b)| b);

    let free_cols: [usize; 16] = {
        let mut arr = [0; 16];
        for i in 0..num_free {
            arr[i] = free_data[i].0;
        }
        arr
    };
    let free_bounds: [u16; 16] = {
        let mut arr = [0; 16];
        for i in 0..num_free {
            arr[i] = free_data[i].1;
        }
        arr
    };

    // Precompute back-substitution coefficients for faster evaluation.
    // Store sparse representation: (col, coeff) pairs for non-zero coefficients.
    // Format: pivot_col, pivot_val, rhs, bound, num_terms, [(col, coeff); 16]
    let mut backsub = [(0usize, 1i32, 0i32, 0i32, 0usize, [(0usize, 0i32); 16]); 16];
    for r in 0..rank {
        let pc = pivot_col_for_row[r];
        let pivot_val = mat[r][pc];
        let rhs = mat[r][num_buttons];
        let mut terms = [(0usize, 0i32); 16];
        let mut num_terms = 0;
        for c in 0..num_buttons {
            if c != pc && mat[r][c] != 0 {
                terms[num_terms] = (c, mat[r][c]);
                num_terms += 1;
            }
        }
        backsub[r] = (pc, pivot_val, rhs, bounds[pc] as i32, num_terms, terms);
    }

    if num_free == 0 {
        // No free variables - evaluate directly.
        return eval_fast(&free_cols, num_free, &[0u32; 16], &backsub, rank)
            .unwrap_or(u32::MAX);
    }

    // Iterative search with inline evaluation.
    let mut best = u32::MAX;
    let mut free_vals = [0u32; 16];
    let mut partial_costs = [0u32; 16];

    // Compute max values for each free variable.
    let mut max_vals = [0u32; 16];
    for i in 0..num_free {
        max_vals[i] = free_bounds[i] as u32;
    }

    let mut idx = 0usize;

    loop {
        if idx == num_free {
            // Evaluate solution.
            let cost = eval_fast(&free_cols, num_free, &free_vals, &backsub, rank);
            if let Some(c) = cost {
                if c < best {
                    best = c;
                }
            }
            // Backtrack.
            if idx == 0 {
                break;
            }
            idx -= 1;
            free_vals[idx] += 1;
            continue;
        }

        let partial = if idx == 0 { 0 } else { partial_costs[idx - 1] };

        // Prune if we can't improve.
        if partial >= best {
            if idx == 0 {
                break;
            }
            idx -= 1;
            free_vals[idx] += 1;
            continue;
        }

        let max_allowed = max_vals[idx].min(best.saturating_sub(partial + 1));
        if free_vals[idx] > max_allowed {
            // Backtrack.
            free_vals[idx] = 0;
            if idx == 0 {
                break;
            }
            idx -= 1;
            free_vals[idx] += 1;
            continue;
        }

        partial_costs[idx] = partial + free_vals[idx];
        idx += 1;
    }

    best
}

#[inline(always)]
fn eval_fast(
    free_cols: &[usize; 16],
    num_free: usize,
    free_vals: &[u32; 16],
    backsub: &[(usize, i32, i32, i32, usize, [(usize, i32); 16]); 16],
    rank: usize,
) -> Option<u32> {
    let mut sol = [0i32; 16];
    let mut total = 0u32;

    for i in 0..num_free {
        let v = free_vals[i] as i32;
        sol[free_cols[i]] = v;
        total += v as u32;
    }

    for r in (0..rank).rev() {
        let (pc, pivot_val, rhs, bound, num_terms, ref terms) = backsub[r];
        let mut sum = rhs;
        for i in 0..num_terms {
            let (c, coeff) = terms[i];
            sum -= coeff * sol[c];
        }
        if pivot_val == 1 {
            if sum < 0 || sum > bound {
                return None;
            }
            sol[pc] = sum;
            total += sum as u32;
        } else if pivot_val == -1 {
            let val = -sum;
            if val < 0 || val > bound {
                return None;
            }
            sol[pc] = val;
            total += val as u32;
        } else {
            if sum % pivot_val != 0 {
                return None;
            }
            let val = sum / pivot_val;
            if val < 0 || val > bound {
                return None;
            }
            sol[pc] = val;
            total += val as u32;
        }
    }

    Some(total)
}

#[inline]
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs().max(1)
}

// =============================================================================
// Parsing
// =============================================================================

fn parse(input: &str) -> Result<Vec<Machine>, String> {
    let mut machines = Vec::with_capacity(200);
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        while i < len && bytes[i] != b'[' {
            i += 1;
        }
        if i >= len {
            break;
        }
        i += 1;

        let mut target = 0u16;
        let mut n = 0;
        while i < len && bytes[i] != b']' {
            if bytes[i] == b'#' {
                target |= 1 << n;
            }
            n += 1;
            i += 1;
        }
        i += 1;

        let mut buttons: Vec<u16> = Vec::with_capacity(16);
        while i < len {
            while i < len && bytes[i] != b'(' && bytes[i] != b'{' && bytes[i] != b'\n' {
                i += 1;
            }
            if i >= len || bytes[i] == b'{' || bytes[i] == b'\n' {
                break;
            }
            i += 1;

            let mut mask = 0u16;
            while i < len && bytes[i] != b')' {
                if bytes[i].is_ascii_digit() {
                    let mut val = 0usize;
                    while i < len && bytes[i].is_ascii_digit() {
                        val = val * 10 + (bytes[i] - b'0') as usize;
                        i += 1;
                    }
                    mask |= 1 << val;
                } else {
                    i += 1;
                }
            }
            if mask != 0 {
                buttons.push(mask);
            }
            if i < len {
                i += 1;
            }
        }

        while i < len && bytes[i] != b'{' {
            i += 1;
        }
        if i >= len {
            break;
        }
        i += 1;

        let mut joltages: Vec<u16> = Vec::with_capacity(n);
        while i < len && bytes[i] != b'}' {
            if bytes[i].is_ascii_digit() {
                let mut val = 0u16;
                while i < len && bytes[i].is_ascii_digit() {
                    val = val * 10 + (bytes[i] - b'0') as u16;
                    i += 1;
                }
                joltages.push(val);
            } else {
                i += 1;
            }
        }

        buttons.sort_unstable();
        buttons.dedup();

        machines.push(Machine {
            target,
            buttons,
            joltages,
            n,
        });
    }

    Ok(machines)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE).unwrap(), 7);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE).unwrap(), 33);
    }

    #[test]
    fn test_both() {
        let (p1, p2) = both(EXAMPLE).unwrap();
        assert_eq!(p1, 7);
        assert_eq!(p2, 33);
    }
}
