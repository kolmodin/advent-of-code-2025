use std::fs;

use anyhow::{Context, Result, bail, ensure};

fn parse_line(ln: &str) -> Result<i64> {
    ensure!(ln.len() > 1, "line too short: '{}'", ln);
    let n: i64 = ln[1..]
        .parse()
        .with_context(|| format!("suffix not a number '{}'", ln))?;
    match ln.as_bytes()[0] {
        b'L' => Ok(n),
        b'R' => Ok(-n),
        _ => bail!("expected L or R when parsing '{}'", ln),
    }
}

fn main() -> Result<()> {
    let lines: Vec<i64> = fs::read_to_string("inputs/day01.txt")?
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>>>()?;

    let part1 = lines
        .into_iter()
        .scan(50, |state, x| {
            *state = (*state + x) % 100;
            Some(*state)
        })
        .filter(|x| *x == 0)
        .count();

    println!("Part 1: {}", part1);
    Ok(())
}
