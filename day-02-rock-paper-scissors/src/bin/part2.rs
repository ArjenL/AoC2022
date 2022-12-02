// Advent of Code 2022
// Day 2: Rock Paper Scissors
// Part 2

use anyhow::anyhow;

#[derive(Copy, Clone, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone, Debug)]
enum Goal {
    Win,
    Tie,
    Loss,
}

impl TryFrom<&str> for Choice {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            _ => Err(anyhow!("Invalid character '{}'", value)),
        }
    }
}

impl TryFrom<&str> for Goal {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Goal::Loss),
            "Y" => Ok(Goal::Tie),
            "Z" => Ok(Goal::Win),
            _ => Err(anyhow!("Invalid character '{}'", value)),
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Choice,
    player: Goal,
}

impl Round {
    fn score(&self) -> u32 {
        match self.player {
            Goal::Win => match self.opponent {
                Choice::Rock => 2 + 6,
                Choice::Paper => 3 + 6,
                Choice::Scissors => 1 + 6,
            },
            Goal::Tie => match self.opponent {
                Choice::Rock => 1 + 3,
                Choice::Paper => 2 + 3,
                Choice::Scissors => 3 + 3,
            },
            Goal::Loss => match self.opponent {
                Choice::Rock => 3,
                Choice::Paper => 1,
                Choice::Scissors => 2,
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
                    Goal::try_from(it.next().ok_or_else(|| anyhow!("Unexpected input"))?)?;
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
