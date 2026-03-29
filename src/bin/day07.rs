use anyhow::{Context, Result};
use aoc_2025::input;

fn main() -> Result<()> {
    let input = input::read_input_day(7)?;
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let first_row = lines.first().context("empty input")?;
    let width = first_row.len();
    let start_pos = first_row.iter().position(|&u| u == b'S').context("no S")?;

    let mut beams = vec![0u64; width];
    beams[start_pos] = 1;

    let mut part1 = 0;
    for row in &lines {
        let mut next = vec![0u64; width];
        for (x, &count) in beams.iter().enumerate() {
            if count == 0 {
                continue;
            }
            if row[x] == b'^' {
                part1 += 1;
                if x > 0 {
                    next[x - 1] += count;
                }
                if x + 1 < width {
                    next[x + 1] += count;
                }
            } else {
                next[x] += count;
            }
        }
        beams = next;
    }

    println!("Part 1: {}", part1);
    assert_eq!(part1, 1570);

    let part2: u64 = beams.iter().sum();
    println!("Part 2: {}", part2);

    Ok(())
}
