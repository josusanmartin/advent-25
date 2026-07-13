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

type BackSubRow = (i32, i32, i32, u8, [(u8, i32); 16]);

pub fn part1(input: &str) -> Result<u64, String> {
    let machines = parse(input)?;
    #[cfg(feature = "parallel")]
    {
        Ok(machines.par_iter().map(|m| solve_lights(m) as u64).sum())
    }
    #[cfg(not(feature = "parallel"))]
    {
        Ok(machines.iter().map(|m| solve_lights(m) as u64).sum())
    }
}

pub fn part2(input: &str) -> Result<u64, String> {
    let machines = parse(input)?;
    #[cfg(feature = "parallel")]
    {
        Ok(machines.par_iter().map(|m| solve_joltage(m) as u64).sum())
    }
    #[cfg(not(feature = "parallel"))]
    {
        Ok(machines.iter().map(|m| solve_joltage(m) as u64).sum())
    }
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
    for (btn_idx, &btn) in m.buttons.iter().enumerate() {
        let mut bits = btn;
        while bits != 0 {
            let light = bits.trailing_zeros() as usize;
            mat[light] |= 1 << btn_idx;
            bits &= bits - 1;
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

    let mut free_cols = [0u8; 16];
    let mut free_count = 0usize;
    for col in 0..num_buttons {
        if free_mask & (1 << col) != 0 {
            free_cols[free_count] = col as u8;
            free_count += 1;
        }
    }

    let mut pivot_cols = [0u8; 16];
    let mut other_masks = [0u16; 16];
    for row in 0..rank {
        let row_val = mat[row];
        let pc = row_val.trailing_zeros() as usize;
        pivot_cols[row] = pc as u8;
        other_masks[row] = row_val & !(1 << pc);
    }

    let mut best = u32::MAX;
    for free_bits in 0u16..(1 << num_free) {
        let mut solution = 0u16;
        for i in 0..num_free {
            if (free_bits >> i) & 1 == 1 {
                solution |= 1 << (free_cols[i] as usize);
            }
        }

        for row in (0..rank).rev() {
            let pivot_col = pivot_cols[row] as usize;
            let rhs = (target >> row) & 1;
            let xor_sum = (other_masks[row] & solution).count_ones() & 1;
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
    // Every press contributes at most one unit to any given counter.
    let lower_bound = m.joltages.iter().copied().max().unwrap_or(0) as u32;

    // Compute upper bounds per button.
    let mut bounds = [u16::MAX; 16];
    let mut mat = [[0i32; 17]; 16];
    for (col, &btn) in m.buttons.iter().enumerate() {
        let mut bits = btn;
        while bits != 0 {
            let row = bits.trailing_zeros() as usize;
            bounds[col] = bounds[col].min(unsafe { *m.joltages.get_unchecked(row) });
            mat[row][col] = 1;
            bits &= bits - 1;
        }
    }
    for row in 0..m.n {
        mat[row][num_buttons] = unsafe { *m.joltages.get_unchecked(row) } as i32;
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
                let v = mat[r][c].abs();
                if v == 0 {
                    continue;
                }
                g = gcd(g, v);
                if g == 1 {
                    break;
                }
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

    // Map column index -> free variable index (if free).
    let mut col_to_free = [u8::MAX; 16];
    for (idx, &col) in free_cols[..num_free].iter().enumerate() {
        col_to_free[col] = idx as u8;
    }

    // Precompute back-substitution coefficients for faster evaluation.
    // Store sparse representation in terms of *free indices* (not columns).
    // Format: pivot_val, rhs, bound, num_terms, [(free_idx, coeff); 16]
    let mut backsub: [BackSubRow; 16] = [(1i32, 0i32, 0i32, 0u8, [(0u8, 0i32); 16]); 16];
    for r in 0..rank {
        let pc = pivot_col_for_row[r];
        let pivot_val = mat[r][pc];
        let rhs = mat[r][num_buttons];
        let mut terms = [(0usize, 0i32); 16];
        let mut num_terms = 0;
        for c in 0..num_buttons {
            if c != pc && mat[r][c] != 0 {
                let free_idx = unsafe { *col_to_free.get_unchecked(c) };
                if free_idx == u8::MAX {
                    return u32::MAX;
                }
                terms[num_terms] = (free_idx as usize, mat[r][c]);
                num_terms += 1;
            }
        }
        let mut packed = [(0u8, 0i32); 16];
        for i in 0..num_terms {
            packed[i] = (terms[i].0 as u8, terms[i].1);
        }
        backsub[r] = (pivot_val, rhs, bounds[pc] as i32, num_terms as u8, packed);
    }

    let mut best = u32::MAX;

    // Compute max values for each free variable (sorted by bound).
    let mut max_vals = [0u32; 16];
    for i in 0..num_free {
        max_vals[i] = free_bounds[i] as u32;
    }

    // For fixed values of the other free variables, total presses are affine in one free
    // variable. An exact rational slope therefore tells us which end contains the cheapest
    // feasible value. This lets the specialized loops stop after their first feasible innermost
    // value instead of evaluating every dominated value in that slice.
    let mut descending = [None; 16];
    for (free_idx, direction) in descending[..num_free].iter_mut().enumerate() {
        *direction = objective_descends_with_value(&backsub, rank, free_idx);
    }
    let mut free_vals = [0i32; 16];
    match num_free {
        0 => {
            if let Some(cost) = eval_fast(0, &free_vals, 0, &backsub, rank, best) {
                if cost == lower_bound {
                    return cost;
                }
                best = cost;
            }
        }
        1 => {
            let Some((min0, max0)) =
                feasible_last_range(&free_vals, 0, max_vals[0] as i32, &backsub, rank)
            else {
                return best;
            };
            for step0 in 0..=max0 - min0 {
                let v0 = ordered_value_in_range(step0, min0, max0, descending[0].unwrap_or(false));
                if (v0 as u32) >= best {
                    continue;
                }
                free_vals[0] = v0;
                let limit = if descending[0].is_some() {
                    u32::MAX
                } else {
                    best
                };
                if let Some(cost) = eval_fast(1, &free_vals, v0 as u32, &backsub, rank, limit) {
                    if cost == lower_bound {
                        return cost;
                    }
                    if descending[0].is_some() {
                        return cost;
                    }
                    best = cost;
                }
            }
        }
        2 => {
            let max0 = max_vals[0] as i32;
            let max1 = max_vals[1] as i32;
            for step0 in 0..=max0 {
                let v0 = ordered_value(step0, max0, descending[0].unwrap_or(false));
                if (v0 as u32) >= best {
                    continue;
                }
                free_vals[0] = v0;
                let max1_allowed = max_vals[1]
                    .min(best.saturating_sub(v0 as u32 + 1))
                    .min(max1 as u32) as i32;
                let Some((min1, max1_feasible)) =
                    feasible_last_range(&free_vals, 1, max1_allowed, &backsub, rank)
                else {
                    continue;
                };
                for step1 in 0..=max1_feasible - min1 {
                    let v1 = ordered_value_in_range(
                        step1,
                        min1,
                        max1_feasible,
                        descending[1].unwrap_or(false),
                    );
                    let partial = (v0 + v1) as u32;
                    if partial >= best {
                        continue;
                    }
                    free_vals[1] = v1;
                    let limit = if descending[1].is_some() {
                        u32::MAX
                    } else {
                        best
                    };
                    if let Some(cost) = eval_fast(2, &free_vals, partial, &backsub, rank, limit) {
                        if cost == lower_bound {
                            return cost;
                        }
                        best = best.min(cost);
                        if descending[1].is_some() {
                            break;
                        }
                    }
                }
            }
        }
        3 => {
            let max0 = max_vals[0] as i32;
            let max1 = max_vals[1] as i32;
            let max2 = max_vals[2] as i32;
            for step0 in 0..=max0 {
                let v0 = ordered_value(step0, max0, descending[0].unwrap_or(false));
                if (v0 as u32) >= best {
                    continue;
                }
                free_vals[0] = v0;
                let max1_allowed = max_vals[1]
                    .min(best.saturating_sub(v0 as u32 + 1))
                    .min(max1 as u32) as i32;
                for step1 in 0..=max1_allowed {
                    let v1 = ordered_value(step1, max1_allowed, descending[1].unwrap_or(false));
                    let partial01 = v0 + v1;
                    if (partial01 as u32) >= best {
                        continue;
                    }
                    free_vals[1] = v1;

                    let remaining = best.saturating_sub(partial01 as u32 + 1);
                    let max2_allowed = (remaining.min(max_vals[2]) as i32).min(max2);
                    let Some((min2, max2_feasible)) =
                        feasible_last_range(&free_vals, 2, max2_allowed, &backsub, rank)
                    else {
                        continue;
                    };

                    for step2 in 0..=max2_feasible - min2 {
                        let v2 = ordered_value_in_range(
                            step2,
                            min2,
                            max2_feasible,
                            descending[2].unwrap_or(false),
                        );
                        let partial = (partial01 + v2) as u32;
                        free_vals[2] = v2;
                        let limit = if descending[2].is_some() {
                            u32::MAX
                        } else {
                            best
                        };
                        if let Some(cost) = eval_fast(3, &free_vals, partial, &backsub, rank, limit)
                        {
                            if cost == lower_bound {
                                return cost;
                            }
                            best = best.min(cost);
                            if descending[2].is_some() {
                                break;
                            }
                        }
                    }
                }
            }
        }
        _ => {
            // Fallback to a generic depth-first search.
            let mut partial_costs = [0u32; 16];
            let mut idx = 0usize;
            loop {
                if idx == num_free {
                    let partial = partial_costs[idx - 1];
                    if let Some(cost) =
                        eval_fast(num_free, &free_vals, partial, &backsub, rank, best)
                    {
                        if cost == lower_bound {
                            return cost;
                        }
                        best = cost;
                    }
                    if idx == 0 {
                        break;
                    }
                    idx -= 1;
                    unsafe {
                        *free_vals.get_unchecked_mut(idx) += 1;
                    }
                    continue;
                }

                let partial = if idx == 0 { 0 } else { partial_costs[idx - 1] };
                if partial >= best {
                    if idx == 0 {
                        break;
                    }
                    idx -= 1;
                    unsafe {
                        *free_vals.get_unchecked_mut(idx) += 1;
                    }
                    continue;
                }

                let max_allowed = max_vals[idx].min(best.saturating_sub(partial + 1)) as i32;
                if unsafe { *free_vals.get_unchecked(idx) } > max_allowed {
                    unsafe {
                        *free_vals.get_unchecked_mut(idx) = 0;
                    }
                    if idx == 0 {
                        break;
                    }
                    idx -= 1;
                    unsafe {
                        *free_vals.get_unchecked_mut(idx) += 1;
                    }
                    continue;
                }

                partial_costs[idx] = partial + unsafe { *free_vals.get_unchecked(idx) } as u32;
                idx += 1;
            }
        }
    }

    best
}

#[inline(always)]
fn eval_fast(
    _num_free: usize,
    free_vals: &[i32; 16],
    total_free: u32,
    backsub: &[BackSubRow; 16],
    rank: usize,
    best: u32,
) -> Option<u32> {
    let mut total = total_free;

    for r in 0..rank {
        let (pivot_val, rhs, bound, num_terms, ref terms) = backsub[r];
        let mut sum = rhs;
        for i in 0..(num_terms as usize) {
            let (fi, coeff) = unsafe { *terms.get_unchecked(i) };
            sum -= coeff * unsafe { *free_vals.get_unchecked(fi as usize) };
        }

        let val = if pivot_val == 1 {
            sum
        } else if pivot_val == -1 {
            -sum
        } else {
            if sum % pivot_val != 0 {
                return None;
            }
            sum / pivot_val
        };

        if val < 0 || val > bound {
            return None;
        }

        total += val as u32;
        if total >= best {
            return None;
        }
    }

    Some(total)
}

#[inline(always)]
fn ordered_value(step: i32, max: i32, descending: bool) -> i32 {
    if descending {
        max - step
    } else {
        step
    }
}

#[inline(always)]
fn ordered_value_in_range(step: i32, min: i32, max: i32, descending: bool) -> i32 {
    if descending {
        max - step
    } else {
        min + step
    }
}

/// Intersect the exact box constraints for every pivot variable to bound the final free variable
/// in a specialized search slice. Divisibility is intentionally left to `eval_fast`.
fn feasible_last_range(
    free_vals: &[i32; 16],
    last_free: usize,
    max_value: i32,
    backsub: &[BackSubRow; 16],
    rank: usize,
) -> Option<(i32, i32)> {
    let mut lower = 0i64;
    let mut upper = max_value as i64;

    for &(pivot, rhs, bound, num_terms, ref terms) in &backsub[..rank] {
        let mut base = rhs as i64;
        let mut last_coefficient = 0i64;
        for &(free_idx, coefficient) in &terms[..num_terms as usize] {
            if free_idx as usize == last_free {
                last_coefficient = coefficient as i64;
            } else {
                base -= coefficient as i64 * free_vals[free_idx as usize] as i64;
            }
        }

        let bound_sum = pivot as i64 * bound as i64;
        let required_low = bound_sum.min(0);
        let required_high = bound_sum.max(0);
        let slope = -last_coefficient;
        if slope == 0 {
            if base < required_low || base > required_high {
                return None;
            }
            continue;
        }

        let (row_lower, row_upper) = if slope > 0 {
            (
                div_ceil(required_low - base, slope),
                div_floor(required_high - base, slope),
            )
        } else {
            (
                div_ceil(required_high - base, slope),
                div_floor(required_low - base, slope),
            )
        };
        lower = lower.max(row_lower);
        upper = upper.min(row_upper);
        if lower > upper {
            return None;
        }
    }

    Some((lower as i32, upper as i32))
}

#[inline(always)]
fn div_floor(numerator: i64, denominator: i64) -> i64 {
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    if remainder != 0 && (remainder > 0) != (denominator > 0) {
        quotient - 1
    } else {
        quotient
    }
}

#[inline(always)]
fn div_ceil(numerator: i64, denominator: i64) -> i64 {
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    if remainder != 0 && (remainder > 0) == (denominator > 0) {
        quotient + 1
    } else {
        quotient
    }
}

/// Returns whether increasing `free_idx` decreases the exact affine objective. `None` means the
/// checked i128 rational arithmetic overflowed, in which case callers retain exhaustive behavior.
fn objective_descends_with_value(
    backsub: &[BackSubRow; 16],
    rank: usize,
    free_idx: usize,
) -> Option<bool> {
    let mut numerator = 1i128;
    let mut denominator = 1i128;

    for &(pivot, _, _, num_terms, ref terms) in &backsub[..rank] {
        let mut coefficient = 0i32;
        for &(idx, value) in &terms[..num_terms as usize] {
            if idx as usize == free_idx {
                coefficient = value;
                break;
            }
        }
        if coefficient == 0 {
            continue;
        }

        let mut term_numerator = -(coefficient as i128);
        let mut term_denominator = pivot as i128;
        if term_denominator < 0 {
            term_numerator = -term_numerator;
            term_denominator = -term_denominator;
        }
        let term_reduce = gcd_i128(term_numerator.checked_abs()?, term_denominator);
        term_numerator /= term_reduce;
        term_denominator /= term_reduce;

        let shared = gcd_i128(denominator, term_denominator);
        let lhs_scale = term_denominator / shared;
        let rhs_scale = denominator / shared;
        numerator = numerator
            .checked_mul(lhs_scale)?
            .checked_add(term_numerator.checked_mul(rhs_scale)?)?;
        denominator = denominator.checked_mul(lhs_scale)?;

        let reduce = gcd_i128(numerator.checked_abs()?, denominator);
        numerator /= reduce;
        denominator /= reduce;
    }

    Some(numerator < 0)
}

#[inline]
fn gcd_i128(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a.max(1)
}

#[inline(always)]
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

    #[test]
    fn monotone_search_matches_bruteforce() {
        let mut state = 0x9e37_79b9u32;
        for case in 0..128 {
            let n = 3 + case % 2;
            let num_buttons = 2 + (case / 3) % 3;
            let mut buttons = Vec::with_capacity(num_buttons);
            while buttons.len() < num_buttons {
                state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                let mut mask = ((state >> 16) as u16) & ((1 << n) - 1);
                if mask == 0 {
                    mask = 1;
                }
                if !buttons.contains(&mask) {
                    buttons.push(mask);
                }
            }
            buttons.sort_unstable();

            let mut presses = [0u16; 16];
            for value in &mut presses[..num_buttons] {
                state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                *value = ((state >> 16) % 3) as u16;
            }
            let mut joltages = vec![0u16; n];
            for (button_idx, &mask) in buttons.iter().enumerate() {
                let mut bits = mask;
                while bits != 0 {
                    let row = bits.trailing_zeros() as usize;
                    joltages[row] += presses[button_idx];
                    bits &= bits - 1;
                }
            }

            let machine = Machine {
                target: 0,
                buttons,
                joltages,
                n,
            };
            assert_eq!(solve_joltage(&machine), brute_joltage(&machine));
        }
    }

    fn brute_joltage(machine: &Machine) -> u32 {
        let mut bounds = [0u16; 16];
        for (button_idx, &mask) in machine.buttons.iter().enumerate() {
            let mut bits = mask;
            let mut bound = u16::MAX;
            while bits != 0 {
                let row = bits.trailing_zeros() as usize;
                bound = bound.min(machine.joltages[row]);
                bits &= bits - 1;
            }
            bounds[button_idx] = bound;
        }

        fn visit(
            machine: &Machine,
            bounds: &[u16; 16],
            values: &mut [u16; 16],
            idx: usize,
            partial: u32,
            best: &mut u32,
        ) {
            if partial >= *best {
                return;
            }
            if idx == machine.buttons.len() {
                let mut counters = [0u16; 16];
                for (button_idx, &mask) in machine.buttons.iter().enumerate() {
                    let mut bits = mask;
                    while bits != 0 {
                        let row = bits.trailing_zeros() as usize;
                        counters[row] += values[button_idx];
                        bits &= bits - 1;
                    }
                }
                if counters[..machine.n] == machine.joltages[..] {
                    *best = partial;
                }
                return;
            }

            for value in 0..=bounds[idx] {
                values[idx] = value;
                visit(
                    machine,
                    bounds,
                    values,
                    idx + 1,
                    partial + value as u32,
                    best,
                );
            }
        }

        let mut values = [0u16; 16];
        let mut best = u32::MAX;
        visit(machine, &bounds, &mut values, 0, 0, &mut best);
        best
    }
}
