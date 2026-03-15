use anyhow::{Context, Result};
use std::fs;

fn is_valid_part1(n: i64) -> bool {
    let s = n.to_string();
    if !s.len().is_multiple_of(2) {
        return true;
    }

    let b = s.as_bytes();
    let offset = b.len() / 2;
    for i in 0..b.len() / 2 {
        if b[i] != b[i + offset] {
            return true;
        }
    }

    false
}

fn is_valid_part2(n: i64) -> bool {
    let s = n.to_string();
    let b = s.as_bytes();

    'n: for n in 2..=b.len() {
        if !s.len().is_multiple_of(n) {
            continue;
        }

        let chunk_size = b.len() / n;
        for i in 1..n {
            for j in 0..chunk_size {
                if b[j] != b[i * chunk_size + j] {
                    continue 'n;
                }
            }
        }
        return false;
    }
    true
}

fn parse_line(ln: &str) -> Result<(i64, i64)> {
    let (a, b) = ln
        .split_once('-')
        .with_context(|| format!("missing '-' in line {}", ln))?;
    Ok((a.parse()?, b.parse()?))
}

fn main() -> Result<()> {
    let ranges: Vec<(i64, i64)> = fs::read_to_string("inputs/day02.txt")?
        .split(',')
        .map(parse_line)
        .collect::<Result<_>>()?;

    let part1: i64 = ranges
        .iter()
        .flat_map(|&(a, b)| a..=b)
        .filter(|&n| !is_valid_part1(n))
        .sum();

    let part2: i64 = ranges
        .into_iter()
        .flat_map(|(a, b)| a..=b)
        .filter(|&n| !is_valid_part2(n))
        .sum();

    println!("Part 1: {}", part1);
    assert_eq!(part1, 30608905813);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 31898925685);

    Ok(())
}
