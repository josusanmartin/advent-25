fn main() {
    let rows: Vec<&[u8]> = include_str!("../inputs/07.txt")
        .lines()
        .map(str::as_bytes)
        .collect();
    let width = rows[0].len();
    let start = rows
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&b| b == b'S').map(|c| (r, c)))
        .unwrap();
    let mut current = vec![0u128; width];
    let mut next = vec![0u128; width];
    current[start.1] = 1;
    let mut total_active = 0usize;
    let mut max_active = 0usize;
    let mut rows_seen = 0usize;
    for row in rows.iter().skip(start.0 + 1) {
        let active = current.iter().filter(|&&n| n != 0).count();
        total_active += active;
        max_active = max_active.max(active);
        rows_seen += 1;
        next.fill(0);
        for col in 0..width {
            let count = current[col];
            if count == 0 {
                continue;
            }
            if row[col] == b'^' {
                if col > 0 {
                    next[col - 1] += count;
                }
                if col + 1 < width {
                    next[col + 1] += count;
                }
            } else {
                next[col] += count;
            }
        }
        std::mem::swap(&mut current, &mut next);
    }
    println!(
        "width={width} rows={rows_seen} avg_active={:.1} max_active={max_active}",
        total_active as f64 / rows_seen as f64
    );
}
