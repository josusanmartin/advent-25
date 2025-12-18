pub static INPUT: &str = include_str!("../inputs/12.txt");

use std::collections::HashSet;

#[derive(Clone)]
struct Orientation {
    cells: Vec<(u8, u8)>,
    width: usize,
    height: usize,
}

#[derive(Clone)]
struct Shape {
    area: usize,
    orientations: Vec<Orientation>,
    max_width: usize,
    max_height: usize,
}

struct Region {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

// Day 12 has no part 2 (it's the final day of Advent of Code)
pub fn part1(input: &str) -> Result<u64, String> {
    let (shapes, regions) = parse(input)?;
    if shapes.is_empty() {
        return Ok(0);
    }

    let (max_w, max_h) = max_shape_dims(&shapes);
    let mut ok = 0u64;

    for region in regions {
        if can_fit(&region, &shapes, max_w, max_h) {
            ok += 1;
        }
    }

    Ok(ok)
}

fn can_fit(region: &Region, shapes: &[Shape], max_w: usize, max_h: usize) -> bool {
    let total_area: usize = shapes
        .iter()
        .zip(&region.counts)
        .map(|(shape, &cnt)| shape.area * cnt)
        .sum();
    let board_area = region.width * region.height;
    if total_area > board_area {
        return false;
    }

    let total_shapes: usize = region.counts.iter().sum();
    if total_shapes == 0 {
        return true;
    }

    let slots = (region.width / max_w) * (region.height / max_h);
    if total_shapes <= slots {
        return true;
    }

    can_fit_exact(region.width, region.height, shapes, &region.counts, total_area)
}

#[derive(Clone)]
struct Placement {
    mask: Vec<u64>,
    cells: usize,
}

fn can_fit_exact(
    width: usize,
    height: usize,
    shapes: &[Shape],
    counts: &[usize],
    total_area: usize,
) -> bool {
    let board_area = width * height;
    if total_area == 0 {
        return true;
    }

    let words = (board_area + 63) / 64;
    let mut placements: Vec<Vec<Placement>> = Vec::with_capacity(shapes.len());
    let mut areas: Vec<usize> = Vec::with_capacity(shapes.len());

    for shape in shapes {
        let list = build_placements(shape, width, height, words);
        placements.push(list);
        areas.push(shape.area);
    }

    for (idx, &cnt) in counts.iter().enumerate() {
        if cnt > 0 && placements[idx].is_empty() {
            return false;
        }
    }

    let mut order: Vec<usize> = (0..shapes.len()).collect();
    order.sort_by_key(|&idx| placements[idx].len());

    let mut counts = counts.to_vec();
    let mut occupied = vec![0u64; words];

    dfs_pack(
        &mut counts,
        &order,
        &placements,
        &mut occupied,
        0,
        total_area,
        board_area,
        &areas,
    )
}

fn dfs_pack(
    counts: &mut [usize],
    order: &[usize],
    placements: &[Vec<Placement>],
    occupied: &mut [u64],
    occupied_cells: usize,
    remaining_area: usize,
    board_area: usize,
    areas: &[usize],
) -> bool {
    if remaining_area == 0 {
        return true;
    }
    if remaining_area > board_area.saturating_sub(occupied_cells) {
        return false;
    }

    let mut shape_idx = None;
    for &idx in order {
        if counts[idx] > 0 {
            shape_idx = Some(idx);
            break;
        }
    }
    let Some(idx) = shape_idx else {
        return true;
    };

    let shape_area = areas[idx];
    for placement in &placements[idx] {
        if overlaps(&placement.mask, occupied) {
            continue;
        }

        apply_mask(occupied, &placement.mask);
        counts[idx] -= 1;
        let next_remaining = remaining_area - shape_area;
        let next_occupied = occupied_cells + placement.cells;

        if dfs_pack(
            counts,
            order,
            placements,
            occupied,
            next_occupied,
            next_remaining,
            board_area,
            areas,
        ) {
            return true;
        }

        counts[idx] += 1;
        remove_mask(occupied, &placement.mask);
    }

    false
}

fn overlaps(mask: &[u64], occupied: &[u64]) -> bool {
    mask.iter().zip(occupied).any(|(m, o)| (m & o) != 0)
}

fn apply_mask(occupied: &mut [u64], mask: &[u64]) {
    for (o, m) in occupied.iter_mut().zip(mask) {
        *o |= m;
    }
}

fn remove_mask(occupied: &mut [u64], mask: &[u64]) {
    for (o, m) in occupied.iter_mut().zip(mask) {
        *o ^= m;
    }
}

fn build_placements(shape: &Shape, width: usize, height: usize, words: usize) -> Vec<Placement> {
    let mut placements = Vec::new();
    for orientation in &shape.orientations {
        if orientation.width > width || orientation.height > height {
            continue;
        }
        for y in 0..=height - orientation.height {
            for x in 0..=width - orientation.width {
                let mut mask = vec![0u64; words];
                for &(dy, dx) in &orientation.cells {
                    let row = y + dy as usize;
                    let col = x + dx as usize;
                    let idx = row * width + col;
                    mask[idx / 64] |= 1u64 << (idx % 64);
                }
                placements.push(Placement {
                    mask,
                    cells: shape.area,
                });
            }
        }
    }
    placements
}

fn max_shape_dims(shapes: &[Shape]) -> (usize, usize) {
    shapes.iter().fold((0, 0), |(mw, mh), shape| {
        (mw.max(shape.max_width), mh.max(shape.max_height))
    })
}

fn parse(input: &str) -> Result<(Vec<Shape>, Vec<Region>), String> {
    let mut lines = input.lines().peekable();
    let mut entries: Vec<(usize, Vec<String>)> = Vec::new();

    while let Some(line) = lines.peek() {
        let line = line.trim();
        if line.is_empty() {
            lines.next();
            continue;
        }
        if parse_region_line(line).is_some() {
            break;
        }

        let header = lines.next().unwrap().trim();
        let Some(idx_text) = header.strip_suffix(':') else {
            return Err(format!("Expected shape header, got '{}'", header));
        };
        let idx: usize = idx_text
            .parse()
            .map_err(|_| format!("Invalid shape index '{}'", idx_text))?;

        let mut grid = Vec::new();
        while let Some(next) = lines.peek() {
            let next = next.trim();
            if next.is_empty() {
                lines.next();
                break;
            }
            if parse_region_line(next).is_some() {
                break;
            }
            if next.ends_with(':') && next[..next.len() - 1].chars().all(|c| c.is_ascii_digit()) {
                break;
            }
            grid.push(next.to_string());
            lines.next();
        }

        if grid.is_empty() {
            return Err(format!("Shape {} had no grid lines", idx));
        }
        entries.push((idx, grid));
    }

    if entries.is_empty() {
        return Err("No shapes found in input".into());
    }

    let max_idx = entries.iter().map(|(idx, _)| *idx).max().unwrap();
    let mut shapes: Vec<Option<Shape>> = vec![None; max_idx + 1];

    for (idx, grid) in entries {
        if shapes[idx].is_some() {
            return Err(format!("Duplicate shape index {}", idx));
        }
        shapes[idx] = Some(parse_shape(&grid)?);
    }

    let shapes: Vec<Shape> = shapes
        .into_iter()
        .enumerate()
        .map(|(idx, shape)| shape.ok_or_else(|| format!("Missing shape index {}", idx)))
        .collect::<Result<_, _>>()?;

    let mut regions = Vec::new();
    while let Some(line) = lines.next() {
        if let Some((width, height, counts)) = parse_region_line(line) {
            if counts.len() != shapes.len() {
                return Err(format!(
                    "Region had {} counts but {} shapes exist",
                    counts.len(),
                    shapes.len()
                ));
            }
            regions.push(Region {
                width,
                height,
                counts,
            });
        }
    }

    Ok((shapes, regions))
}

fn parse_region_line(line: &str) -> Option<(usize, usize, Vec<usize>)> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    let mut split = line.split(':');
    let dims = split.next()?;
    let counts = split.next()?;
    if split.next().is_some() {
        return None;
    }

    let mut dims_iter = dims.split('x');
    let width: usize = dims_iter.next()?.parse().ok()?;
    let height: usize = dims_iter.next()?.parse().ok()?;
    if dims_iter.next().is_some() {
        return None;
    }

    let counts: Vec<usize> = counts
        .split_whitespace()
        .map(|val| val.parse::<usize>().ok())
        .collect::<Option<Vec<_>>>()?;

    Some((width, height, counts))
}

fn parse_shape(lines: &[String]) -> Result<Shape, String> {
    let width = lines.first().map(|row| row.len()).unwrap_or(0);
    if width == 0 {
        return Err("Shape rows were empty".into());
    }

    let mut grid: Vec<Vec<bool>> = Vec::with_capacity(lines.len());
    let mut area = 0usize;
    for line in lines {
        if line.len() != width {
            return Err("Shape rows had inconsistent widths".into());
        }
        let mut row = Vec::with_capacity(width);
        for ch in line.chars() {
            match ch {
                '#' => {
                    row.push(true);
                    area += 1;
                }
                '.' => row.push(false),
                other => return Err(format!("Invalid shape character '{}'", other)),
            }
        }
        grid.push(row);
    }

    if area == 0 {
        return Err("Shape had no filled cells".into());
    }

    let mut seen = HashSet::new();
    let mut orientations = Vec::new();
    let mut max_width = 0usize;
    let mut max_height = 0usize;

    let mut current = grid;
    for _ in 0..4 {
        for flip in 0..2 {
            let variant = if flip == 1 {
                flip_horizontal(&current)
            } else {
                current.clone()
            };
            let trimmed = trim_grid(&variant);
            let encoded = encode_grid(&trimmed);
            if seen.insert(encoded) {
                let height = trimmed.len();
                let width = trimmed.first().map(|row| row.len()).unwrap_or(0);
                let mut cells = Vec::new();
                for (r, row) in trimmed.iter().enumerate() {
                    for (c, &filled) in row.iter().enumerate() {
                        if filled {
                            cells.push((r as u8, c as u8));
                        }
                    }
                }
                max_width = max_width.max(width);
                max_height = max_height.max(height);
                orientations.push(Orientation {
                    cells,
                    width,
                    height,
                });
            }
        }
        current = rotate(&current);
    }

    if orientations.is_empty() {
        return Err("Shape had no orientations".into());
    }

    Ok(Shape {
        area,
        orientations,
        max_width,
        max_height,
    })
}

fn rotate(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut out = vec![vec![false; height]; width];
    for r in 0..height {
        for c in 0..width {
            out[c][height - 1 - r] = grid[r][c];
        }
    }
    out
}

fn flip_horizontal(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    grid.iter()
        .map(|row| row.iter().rev().copied().collect())
        .collect()
}

fn trim_grid(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let height = grid.len();
    let width = grid[0].len();

    let mut top = 0usize;
    while top < height && grid[top].iter().all(|&v| !v) {
        top += 1;
    }

    let mut bottom = height;
    while bottom > top && grid[bottom - 1].iter().all(|&v| !v) {
        bottom -= 1;
    }

    let mut left = 0usize;
    while left < width {
        let mut empty = true;
        for r in top..bottom {
            if grid[r][left] {
                empty = false;
                break;
            }
        }
        if !empty {
            break;
        }
        left += 1;
    }

    let mut right = width;
    while right > left {
        let mut empty = true;
        for r in top..bottom {
            if grid[r][right - 1] {
                empty = false;
                break;
            }
        }
        if !empty {
            break;
        }
        right -= 1;
    }

    let mut out = Vec::new();
    for r in top..bottom {
        out.push(grid[r][left..right].to_vec());
    }
    out
}

fn encode_grid(grid: &[Vec<bool>]) -> String {
    let mut out = String::new();
    for (r, row) in grid.iter().enumerate() {
        if r > 0 {
            out.push('/');
        }
        for &cell in row {
            out.push(if cell { '#' } else { '.' });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn example_count() {
        let result = part1(EXAMPLE).unwrap();
        assert_eq!(result, 2);
    }
}
