// Advent of Code 2022
// Day 1: Calorie Counting

use anyhow::Result;
use std::{
    collections::BinaryHeap,
    io::{self},
};

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let mut pq = io::read_to_string(io::stdin())?
        .split("\n\n")
        .map(|elf| elf.lines().filter_map(|l| l.parse::<u32>().ok()).sum())
        .collect::<BinaryHeap<u32>>();

    println!("Calories carried by the top elf: {}", pq.peek().unwrap());
    println!(
        "Calories carried by the top 3 elves: {}",
        (0..3).map(|_| pq.pop().unwrap()).sum::<u32>()
    );
    println!("{:?}", now.elapsed());

    Ok(())
}
