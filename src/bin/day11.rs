use std::{collections::HashMap, vec};

use anyhow::{Context, Result};
use aoc_2025::input;

fn parse_line(ln: &str) -> Result<(&str, Vec<&str>)> {
    let (src, rest) = ln
        .split_once(": ")
        .context("invalid input, could not find :")?;
    Ok((src, rest.split_ascii_whitespace().collect()))
}

struct PathCounter<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
    cache: HashMap<(&'a str, &'a str), usize>,
}

impl<'a> PathCounter<'a> {
    fn new(edges: HashMap<&'a str, Vec<&'a str>>) -> Self {
        Self {
            edges,
            cache: HashMap::new(),
        }
    }

    fn count_paths(&mut self, src: &'a str, dst: &'a str) -> usize {
        if src == dst {
            return 1;
        }

        if let Some(count) = self.cache.get(&(src, dst)) {
            return *count;
        }

        let neighbours = self.edges.get(src).cloned().unwrap_or(vec![]);

        let count = neighbours
            .into_iter()
            .map(|n| self.count_paths(n, dst))
            .sum::<usize>();

        self.cache.insert((src, dst), count);

        count
    }
}

fn main() -> Result<()> {
    let raw = input::read_input_day(11)?;
    let input = raw
        .lines()
        .map(parse_line)
        .collect::<Result<HashMap<&str, Vec<&str>>>>()?;

    let mut pc = PathCounter::new(input);
    let part1 = pc.count_paths("you", "out");
    println!("Part 1: {}", part1);
    assert_eq!(part1, 534);

    let part2 = {
        // Two routes from svr to out, each passing through fft and dac in opposite orders:
        //   Route A: svr ->..-> fft ->..-> dac ->..-> out
        //   Route B: svr ->..-> dac ->..-> fft ->..-> out
        // The total path count for each route is the product of paths per segment
        // (segments are independent), and the two routes are summed.
        let svr_to_fft = pc.count_paths("svr", "fft");
        let fft_to_dac = pc.count_paths("fft", "dac");
        let dac_to_out = pc.count_paths("dac", "out");

        let svr_to_dac = pc.count_paths("svr", "dac");
        let dac_to_fft = pc.count_paths("dac", "fft");
        let fft_to_out = pc.count_paths("fft", "out");

        let route_a = svr_to_fft * fft_to_dac * dac_to_out;
        let route_b = svr_to_dac * dac_to_fft * fft_to_out;

        route_a + route_b
    };

    println!("Part 2: {}", part2);
    assert_eq!(part2, 499645520864100);

    Ok(())
}
