use std::str::FromStr;

use anyhow::{Context, Result, anyhow};
use aoc_2025::input;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Pos2 {
    x: i64,
    y: i64,
}

impl Pos2 {
    fn area(self, other: &Self) -> i64 {
        let width = (self.x - other.x).abs() + 1;
        let height = (self.y - other.y).abs() + 1;
        width * height
    }
}

impl FromStr for Pos2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split(',')
            .map(|s| s.parse::<i64>().context("failed to parse coordinate"))
            .collect_tuple()
            .ok_or_else(|| anyhow!("expected 2 coordinates"))?;

        Ok(Pos2 { x: x?, y: y? })
    }
}

fn main() -> Result<()> {
    let input: Vec<Pos2> = input::read_input_day(9)?
        .lines()
        .map(Pos2::from_str)
        .collect::<Result<_>>()?;

    for x in input.iter().take(10) {
        println!("{:?}", x);
    }

    let part1 = input
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.area(b))
        .max()
        .context("no input")?;

    println!("Part 1: {}", part1);
    assert_eq!(part1, 4750092396);

    Ok(())
}
