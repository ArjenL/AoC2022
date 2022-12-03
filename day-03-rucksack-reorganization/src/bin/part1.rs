// Advent of Code 2022
// Day 3
// Part 1

use anyhow::Result;
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

    let mut prio_sum: u64 = 0;
    let now = std::time::Instant::now();
    for line in input.lines() {
        let prios: Vec<_> = line.as_bytes().iter().map(item_priority).collect();
        let first: HashSet<_> = prios[..prios.len() / 2].iter().collect();
        let second: HashSet<_> = prios[prios.len() / 2..].iter().collect();

        prio_sum += **first.intersection(&second).next().unwrap();
    }

    println!("{prio_sum} ({:?})", now.elapsed());

    Ok(())
}
