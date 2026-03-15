use anyhow::{Context, Result};
use itertools::Itertools;
use std::fs;

fn parse_line(ln: &str) -> Vec<u8> {
    ln.as_bytes().iter().map(|b| *b - b'0').collect()
}

fn max(arr: &[u8]) -> Result<(usize, u8)> {
    arr.iter()
        .cloned()
        .enumerate()
        .max_by_key(|&(i, u)| (u, -(i as i64)))
        .context("arr empty")
}

fn search(arr: &[u8], n: usize) -> Result<usize> {
    if arr.len() < n {
        panic!("arr too short");
    }
    let mut i = 0;
    let mut res = 0;
    for m in (0..n).rev() {
        let (j, u) = max(&arr[i..arr.len() - m])?;
        i += j + 1;
        res = res * 10 + u as usize;
    }
    Ok(res)
}

fn main() -> Result<()> {
    let lines: Vec<Vec<u8>> = fs::read_to_string("inputs/day03.txt")?
        .lines()
        .map(parse_line)
        .collect();

    let part1: usize = lines
        .iter()
        .map(|v| search(v, 2))
        .process_results(|vec| vec.sum())?;
    let part2: usize = lines
        .iter()
        .map(|v| search(v, 12))
        .process_results(|vec| vec.sum())?;



    println!("Part 1: {}", part1);
    assert_eq!(part1, 17087);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 169019504359949);

    Ok(())
}
