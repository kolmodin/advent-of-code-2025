use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use aoc_2025::input;

type Edegs = (String, Vec<String>);

fn parse_line(ln: &str) -> Result<Edegs> {
    let (src, rest) = ln
        .split_once(": ")
        .context("invalid input, could not find :")?;
    let dests: Vec<String> = rest
        .split_ascii_whitespace()
        .map(|d| d.to_string())
        .collect();

    Ok((src.to_string(), dests))
}

fn count_paths(
    src: &str,
    dst: &str,
    edges: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, usize>,
) -> Result<usize> {
    if src == dst {
        return Ok(1);
    }

    if src == "out" {
        return Ok(0);
    }

    if let Some(count) = cache.get(src) {
        return Ok(*count);
    }

    let neighbours = edges
        .get(src)
        .with_context(|| format!("could not find node '{}'", src))?;

    let count = neighbours
        .iter()
        .map(|n| count_paths(n, dst, edges, cache))
        .sum::<Result<usize>>()?;

    cache.insert(src.to_string(), count);

    Ok(count)
}

fn main() -> Result<()> {
    let input = input::read_input_day(11)?
        .lines()
        .map(parse_line)
        .collect::<Result<HashMap<String, Vec<String>>>>()?;

    let part1 = count_paths("you", "out", &input, &mut HashMap::new())?;
    println!("Part 1: {}", part1);
    assert_eq!(part1, 534);

    let part2 = {
        let fft = count_paths("svr", "fft", &input, &mut HashMap::new())?;
        let dac = count_paths("fft", "dac", &input, &mut HashMap::new())?;
        let out = count_paths("dac", "out", &input, &mut HashMap::new())?;

        let dac2 = count_paths("svr", "dac", &input, &mut HashMap::new())?;
        let fft2 = count_paths("dac", "fft", &input, &mut HashMap::new())?;
        let out2 = count_paths("fft", "out", &input, &mut HashMap::new())?;

        fft * dac * out + fft2 * dac2 * out2
    };

    println!("Part 2: {}", part2);
    assert_eq!(part2, 499645520864100);

    Ok(())
}
