// Advent of Code 2022
// Day 2: Rock Paper Scissors
// Part 1

use anyhow::anyhow;

#[derive(Copy, Clone, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Choice {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(anyhow!("Invalid character '{}'", value)),
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Choice,
    player: Choice,
}

impl Round {
    fn score(&self) -> u32 {
        match self.opponent {
            Choice::Rock => match self.player {
                Choice::Rock => 1 + 3,
                Choice::Paper => 2 + 6,
                Choice::Scissors => 3,
            },
            Choice::Paper => match self.player {
                Choice::Rock => 1,
                Choice::Paper => 2 + 3,
                Choice::Scissors => 3 + 6,
            },
            Choice::Scissors => match self.player {
                Choice::Rock => 1 + 6,
                Choice::Paper => 2,
                Choice::Scissors => 3 + 3,
            },
        }
    }
}

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn from_str(string: &str) -> Result<Self, anyhow::Error> {
        let rounds = string
            .lines()
            .map(|line| {
                let mut it = line.split_whitespace();
                let opponent =
                    Choice::try_from(it.next().ok_or_else(|| anyhow!("Unexpected input"))?)?;
                let player =
                    Choice::try_from(it.next().ok_or_else(|| anyhow!("Unexpected input"))?)?;
                Ok(Round { opponent, player })
            })
            .collect::<anyhow::Result<Vec<Round>>>()?;

        Ok(Self { rounds })
    }

    fn play(&self) -> u32 { self.rounds.iter().map(|r| r.score()).sum() }
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let now = std::time::Instant::now();
    let input = std::io::read_to_string(stdin)?;
    let game = Game::from_str(&input)?;
    println!("score: {} ({:?})", game.play(), now.elapsed());

    Ok(())
}
