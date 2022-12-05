// Advent of Code 2022
// Day 5
// Part 1

use anyhow::Result;
use std::io::{self};

fn scan_stacks(input: &[&[u8]]) -> Vec<Vec<u8>> {
    let nstacks = (input[0].len() - 3) / 4 + 1;
    let mut stacks: Vec<Vec<u8>> = (0..nstacks).map(|_| Vec::new()).collect();
    for layer in (0..=input.len() - 1).rev() {
        for s in 0..nstacks {
            let c = input[layer][s * 4 + 1];
            if c == b' ' {
                continue;
            }
            stacks[s].push(c);
        }
    }

    stacks
}

fn move_crate(stacks: &mut [Vec<u8>], from: usize, to: usize, n: usize) {
    for _ in 0..n {
        let t = stacks[from].pop().unwrap();
        stacks[to].push(t);
    }
}

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let mut it = input.split("\n\n");
    let (stacks, moves) = (it.next().unwrap(), it.next().unwrap());
    let stacks = stacks
        .split('\n')
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();

    let mut stacks = scan_stacks(&stacks);

    for m in moves.lines() {
        let v = m.split(' ').collect::<Vec<_>>();

        let (n, from, to) = (
            v[1].parse::<usize>().unwrap(),
            v[3].parse::<usize>().unwrap() - 1,
            v[5].parse::<usize>().unwrap() - 1,
        );
        move_crate(&mut stacks, from, to, n);
    }

    let message = stacks
        .iter()
        .map(|s| *s.last().unwrap() as char)
        .collect::<String>();
    println!("{message}");

    Ok(())
}
