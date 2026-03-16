use anyhow::{Context, Result, bail, ensure};
use std::fs;

fn parse_line(ln: &str) -> Result<i64> {
    ensure!(ln.len() > 1, "line too short: '{}'", ln);
    let n: i64 = ln[1..]
        .parse()
        .with_context(|| format!("suffix is not a number '{}'", ln))?;
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

    let mut part1 = 0;
    let mut part2 = 0;

    let _: i64 = lines.into_iter().fold(50, |state, dir| {
        let rotated = state + dir;
        if rotated.rem_euclid(100) == 0 {
            part1 += 1;
        }
        part2 += (rotated / 100).abs();
        if state * rotated < 0 || rotated == 0 {
            part2 += 1;
        }

        rotated.rem_euclid(100)
    });

    println!("Part 1: {}", part1);
    assert_eq!(part1, 997);
    println!("Part 2: {}", part2);
    assert_eq!(part2, 5978);

    Ok(())
}
