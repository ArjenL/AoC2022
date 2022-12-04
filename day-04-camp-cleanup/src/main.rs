// Advent of Code 2022
// Day 4

use anyhow::Result;
use std::io;

fn complete_overlap(e: &[usize]) -> bool {
    e[0] <= e[2] && e[1] >= e[3] || e[0] >= e[2] && e[1] <= e[3]
}

fn partial_overlap(e: &[usize]) -> bool { e[1] >= e[2] && e[0] <= e[3] }

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let (part1, part2) = io::read_to_string(io::stdin())?
        .lines()
        .map(|line| {
            line.split(&['-', ','])
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .fold((0, 0), |acc, e| {
            (
                acc.0 + complete_overlap(&e) as usize,
                acc.1 + partial_overlap(&e) as usize,
            )
        });

    let elapsed = now.elapsed();
    println!("part1: {part1}, part2: {part2} ({:?})", elapsed);

    Ok(())
}
