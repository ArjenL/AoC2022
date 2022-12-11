// Advent of Code 2022
// Day 8

use std::{collections::HashSet, io};

fn visibles(grid: &[u8], rows: usize, cols: usize) -> HashSet<usize> {
    let mut visible = HashSet::new();

    // Horizontal
    for row in 0..rows {
        // From left
        visible.insert(row * cols);
        let mut max = grid[row * cols]; // First element
        for col in 1..cols - 1 {
            let idx = row * cols + col;
            let v = grid[idx];
            if v > max {
                max = v;
                visible.insert(idx);
            }
        }

        // From right
        visible.insert(row * cols + cols - 1);
        max = grid[row * cols + cols - 1]; // Last element
        for col in (1..cols - 1).rev() {
            let idx = row * cols + col;
            let v = grid[idx];
            if v > max {
                max = v;
                visible.insert(idx);
            }
        }
    }

    // Vertical
    for col in 0..cols {
        // From top
        visible.insert(col);
        let mut max = grid[col]; // Top element
        for row in 1..rows - 1 {
            let idx = row * rows + col;
            let v = grid[idx];
            if v > max {
                max = v;
                visible.insert(idx);
            }
        }

        // From bottom
        max = grid[rows * (cols - 1) + col]; // Bottom element
        visible.insert(rows * (cols - 1) + col);
        for row in (1..rows - 1).rev() {
            let idx = row * rows + col;
            let v = grid[idx];
            if v > max {
                max = v;
                visible.insert(idx);
            }
        }
    }

    visible
}

fn score_up(grid: &[Vec<u8>], row: usize, column: usize) -> u32 {
    let mut score = 0;
    for r in (0..=row - 1).rev() {
        if grid[r][column] >= grid[row][column] {
            return score + 1;
        }
        score += 1;
    }

    score
}

fn score_down(grid: &[Vec<u8>], row: usize, column: usize) -> u32 {
    let mut score = 0;
    for r in row + 1..grid.len() {
        if grid[r][column] >= grid[row][column] {
            return score + 1;
        }
        score += 1;
    }

    score
}

fn score_right(grid: &[Vec<u8>], row: usize, column: usize) -> u32 {
    let mut score = 0;
    for c in column + 1..grid[0].len() {
        if grid[row][c] >= grid[row][column] {
            return score + 1;
        }
        score += 1;
    }

    score
}

fn score_left(grid: &[Vec<u8>], row: usize, column: usize) -> u32 {
    let mut score = 0;
    for c in (0..column).rev() {
        if grid[row][c] >= grid[row][column] {
            return score + 1;
        }
        score += 1;
    }

    score
}

fn max_scenic_score(grid: &[Vec<u8>]) -> u32 {
    let (width, height) = (grid[0].len(), grid.len());
    let mut max_score = 0;

    for r in 1..height - 1 {
        for c in 1..width - 1 {
            let score = score_up(grid, r, c)
                * score_left(grid, r, c)
                * score_down(grid, r, c)
                * score_right(grid, r, c);
            max_score = if score > max_score { score } else { max_score };
        }
    }

    max_score
}

fn main() -> anyhow::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let grid = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&v| v - b'0')
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let (rows, cols) = (grid.len(), grid[0].len());

    let ngrid = grid
        .iter()
        .flat_map(|row| row.iter().copied())
        .collect::<Vec<u8>>();

    let now = std::time::Instant::now();
    let part1 = visibles(&ngrid, rows, cols).len();
    let part2 = max_scenic_score(&grid);
    let elapsed = now.elapsed();
    println!("{part1} {part2} ({:?})", elapsed);

    Ok(())
}
