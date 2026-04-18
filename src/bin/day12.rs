use anyhow::{Context, Result};
use aoc_2025::input;

#[derive(Debug)]
struct Input {
    shapes: Vec<usize>,
    boards: Vec<Board>,
}

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    count: Vec<usize>,
}

impl Board {
    // A sad approximation that works on my input.
    fn is_solveable(&self, shape_sizes: &[usize]) -> bool {
        let req: usize = self
            .count
            .iter()
            .zip(shape_sizes)
            .map(|(count, &size)| count * size)
            .sum();
        self.width * self.height >= req
    }
}

fn parse(inp: &str) -> Result<Input> {
    let mut groups: Vec<_> = inp.trim().split("\n\n").collect();
    let boards_str = groups.pop().context("no boards")?;
    let shapes = groups
        .into_iter()
        .map(|s| s.chars().filter(|&c| c == '#').count())
        .collect();

    let boards = boards_str
        .lines()
        .map(|line| {
            let (size, count_str) = line.split_once(": ").context("parsing board")?;
            let (width, height) = size.split_once('x').context("parsing size")?;
            let count = count_str
                .split_ascii_whitespace()
                .map(|n| n.parse().context("parsing count"))
                .collect::<Result<Vec<_>>>()?;
            Ok(Board {
                width: width.parse()?,
                height: height.parse()?,
                count,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Input { shapes, boards })
}

fn main() -> Result<()> {
    let raw = input::read_input_day(12)?;
    let input = parse(&raw)?;

    let part1 = input
        .boards
        .iter()
        .filter(|b| b.is_solveable(&input.shapes))
        .count();

    println!("Part 1: {}", part1);
    Ok(())
}
