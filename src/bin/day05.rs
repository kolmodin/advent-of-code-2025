use std::cmp::max;
use std::ops::RangeInclusive;

use anyhow::{Context, Result};

use aoc_2025::input;

fn merge_all(mut ranges: Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    if ranges.is_empty() {
        return vec![];
    }

    ranges.sort_unstable_by_key(|r| *r.start());
    let mut merged = Vec::with_capacity(ranges.len());

    for next in ranges {
        match merged.last_mut() {
            None => merged.push(next),
            Some(last) => {
                if next.start() <= last.end() {
                    *last = *last.start()..=max(*last.end(), *next.end());
                } else {
                    merged.push(next);
                }
            }
        }
    }

    merged
}

fn parse_range(ln: &str) -> Result<RangeInclusive<i64>> {
    let (s, e) = ln.split_once('-').context("expected '-' in range")?;
    Ok(s.parse()?..=e.parse()?)
}

fn main() -> Result<()> {
    let input = input::read_input_day(5)?;
    let (ranges_str, products_str) = input.split_once("\n\n").context("invalid input")?;

    let ranges: Vec<_> = ranges_str.lines().map(parse_range).collect::<Result<_>>()?;

    let products: Vec<i64> = products_str
        .lines()
        .map(|p| p.parse().context("could not parse product number"))
        .collect::<Result<_>>()?;

    let part1 = products
        .iter()
        .filter(|&p| ranges.iter().any(|r| r.contains(p)))
        .count();
    println!("Part 1: {}", part1);
    assert_eq!(part1, 525);

    let part2: i64 = merge_all(ranges)
        .into_iter()
        .map(|r| r.end() - r.start() + 1)
        .sum();
    println!("Part 2: {}", part2);
    assert_eq!(part2, 333892124923577);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_all_empty() {
        let ranges: Vec<RangeInclusive<i64>> = vec![];
        assert_eq!(merge_all(ranges), vec![]);
    }

    #[test]
    fn test_merge_all_single() {
        let ranges = vec![10..=20];
        assert_eq!(merge_all(ranges), vec![10..=20]);
    }

    #[test]
    fn test_merge_all_before() {
        let ranges = vec![10..=20, 1..=5];
        assert_eq!(merge_all(ranges), vec![1..=5, 10..=20]);
    }

    #[test]
    fn test_merge_all_after() {
        let ranges = vec![10..=20, 25..=30];
        assert_eq!(merge_all(ranges), vec![10..=20, 25..=30]);
    }

    #[test]
    fn test_merge_all_overlap() {
        let ranges = vec![10..=20, 30..=40, 15..=35];
        assert_eq!(merge_all(ranges), vec![10..=40]);
    }

    #[test]
    fn test_merge_all_partial_overlap() {
        let ranges = vec![10..=20, 30..=40, 5..=15];
        assert_eq!(merge_all(ranges), vec![5..=20, 30..=40]);
    }

    #[test]
    fn test_merge_all_multiple() {
        let ranges = vec![10..=20, 30..=40, 50..=60, 15..=55];
        assert_eq!(merge_all(ranges), vec![10..=60]);
    }
}
