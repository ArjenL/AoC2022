// Advent of Code 2022
// Day 9

use anyhow::{anyhow, Result};
use std::{collections::HashSet, io, io::BufRead, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord(isize, isize);

impl Coord {
    fn catch_up(&self, other: &Self) -> Self {
        let diff_hor = other.0 - self.0;
        let diff_ver = other.1 - self.1;

        if diff_hor.abs() > 1 || diff_ver.abs() > 1 {
            return Coord(self.0 + diff_hor.signum(), self.1 + diff_ver.signum());
        }

        *self
    }

    fn step(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self(self.0, self.1 + 1),
            Direction::Down => Self(self.0, self.1 - 1),
            Direction::Left => Self(self.0 - 1, self.1),
            Direction::Right => Self(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(anyhow!("Invalid input {}", s)),
        }
    }
}

#[derive(Debug)]
struct Simulation {
    input: Vec<Direction>,
    rope: Vec<Coord>,
    tail_motions: HashSet<Coord>,
}

impl Simulation {
    fn from_reader<B: BufRead>(reader: B, tail_size: usize) -> Result<Self> {
        let input = reader
            .lines()
            .flat_map(|line| {
                let line = line.unwrap();
                let mut it = line.split_whitespace();
                let direction = it.next().unwrap().parse::<Direction>().unwrap();
                let steps = it.next().unwrap().parse::<usize>().unwrap();
                (0..steps).map(move |_| direction)
            })
            .collect::<Vec<Direction>>();

        let rope = (0..tail_size + 1)
            .map(|_| Coord(0, 0))
            .collect::<Vec<Coord>>();
        let tail_motions = HashSet::new();

        Ok(Self {
            input,
            rope,
            tail_motions,
        })
    }

    #[allow(dead_code)] fn display(&self) {
        for r in (0..5).rev() {
            for c in 0..6 {
                let mut displayed = false;
                for (i, t) in self.rope.iter().enumerate() {
                    if *t == Coord(c, r) && !displayed {
                        let elem = if i == 0 {
                            "H".to_string()
                        }
                        else {
                            format!("{}", i)
                        };
                        print!("{elem}");
                        displayed = true;
                    }
                }
                if !displayed {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn run(&mut self) {
        for d in self.input.iter() {
            // Update head
            self.rope[0] = self.rope[0].step(*d);

            // Update tail
            for i in 1..self.rope.len() {
                self.rope[i] = self.rope[i].catch_up(&self.rope[i-1]);
            }

            let last = self.rope.last().unwrap();
            self.tail_motions.insert(*last);
        }
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut sim = Simulation::from_reader(handle, 9)?;
    let now = std::time::Instant::now();
    sim.run();
    let elapsed = now.elapsed();
    println!("{} ({:?})", sim.tail_motions.len(), elapsed);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn horizontal() {
        let (h, t) = (Coord(2, 4), Coord(4, 4));
        dbg!(&(h, t));
        let cu = t.catch_up(&h);
        assert_eq!(cu, Coord(3, 4));
    }

    #[test]
    fn vertical() {
        let (h, t) = (Coord(4, 2), Coord(4, 4));
        dbg!(&(h, t));
        let cu = t.catch_up(&h);
        assert_eq!(cu, Coord(4, 3));
    }

    #[test]
    fn tail_x_gt_head_x() {
        // Tail x > Head x
        let (h, t) = (Coord(2, 4), Coord(4, 3));
        dbg!(&(h, t));
        let cu = t.catch_up(&h);
        assert_eq!(cu, Coord(3, 4));
    }

    #[test]
    fn head_x_gt_tail_x() {
        // Head x > Tail x
        let (t, h) = (Coord(2, 4), Coord(4, 3));
        let cu = t.catch_up(&h);
        assert_eq!(cu, Coord(3, 3));
    }

    #[test]
    fn tail_y_gt_head_y() {
        let (h, t) = (Coord(2, 4), Coord(3, 2));
        let cu = t.catch_up(&h);
        assert_eq!(cu, Coord(2, 3));
    }

    #[test]
    fn head_y_gt_tail_y() {
        let (t, h) = (Coord(2, 4), Coord(3, 2));
        let cu = t.catch_up(&h);
        assert_eq!(cu, Coord(3, 3));
    }

    #[test]
    fn tail_xy_gt_head_xy() {
        let (t, h) = (Coord(2, 4), Coord(4, 2));
        let cu = t.catch_up(&h);
        assert_eq!(cu, Coord(3, 3));
    }

    #[test]
    fn head_xy_gt_tail_xy() {
        let (t, h) = (Coord(2, 4), Coord(4, 2));
        let cu = t.catch_up(&h);
        assert_eq!(cu, Coord(3, 3));
    }
}
