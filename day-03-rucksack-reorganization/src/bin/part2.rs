// Advent of Code 2022
// Day 3
// Part 2

use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashSet, io};

fn item_priority(item: &u8) -> u64 {
    match item {
        b'a'..=b'z' => (item - 96) as u64,
        b'A'..=b'Z' => (item - 38) as u64,
        _ => unreachable!(),
    }
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin().lock())?;

    let mut prio_sum = 0;
    let now = std::time::Instant::now();
    for group in &input.lines().chunks(3) {
        let badges: Vec<HashSet<u64>> = group
            .map(|line| line.as_bytes().iter().map(item_priority).collect())
            .collect();

        prio_sum += badges[0]
            .intersection(&badges[1])
            .copied()
            .collect::<HashSet<u64>>()
            .intersection(&badges[2])
            .next()
            .unwrap();
    }

    println!("{prio_sum} ({:?})", now.elapsed());

    Ok(())
}
