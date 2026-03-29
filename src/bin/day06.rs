use anyhow::{Context, Result};

fn parse_ints(strs: &[&str]) -> Result<Vec<i64>> {
    strs.iter()
        .map(|x| {
            x.parse()
                .with_context(|| format!("could not parse int {}", x))
        })
        .collect()
}

fn evaluate(ops: &[&str], groups: Vec<Vec<i64>>) -> Result<i64> {
    let mut total = 0;
    for (op, group) in ops.iter().zip(groups) {
        total += match *op {
            "+" => group.iter().copied().reduce(|a, b| a + b),
            "*" => group.iter().copied().reduce(|a, b| a * b),
            _ => anyhow::bail!("invalid op"),
        }
        .context("group was empty")?;
    }
    Ok(total)
}

fn main() -> Result<()> {
    let input = aoc_2025::input::read_input_day(6)?;
    let mut input_lines: Vec<Vec<&str>> = input
        .lines()
        .map(|ln| ln.split_whitespace().collect())
        .collect();
    let ops = input_lines.pop().context("expected at least one line")?;

    let grid = input_lines
        .iter()
        .map(|l| parse_ints(l))
        .collect::<Result<Vec<_>>>()?;

    let groups_p1: Vec<Vec<i64>> = (0..ops.len())
        .map(|i| grid.iter().map(|row| row[i]).collect())
        .collect();

    let part1 = evaluate(&ops, groups_p1)?;
    println!("Part 1: {}", part1);
    assert_eq!(part1, 5381996914800);

    let mut groups_p2 = vec![];
    let mut current_group = vec![];
    let rows: Vec<&[u8]> = input
        .lines()
        .take(input_lines.len())
        .map(|l| l.as_bytes())
        .collect();
    let width = rows.iter().map(|r| r.len()).max().unwrap_or(0);

    for x in 0..width {
        let col: String = rows
            .iter()
            .filter_map(|r| r.get(x))
            .filter(|&&b| b != b' ')
            .map(|&b| b as char)
            .collect();
        if col.is_empty() {
            if !current_group.is_empty() {
                groups_p2.push(std::mem::take(&mut current_group));
            }
        } else {
            current_group.push(col.parse().context("could not parse column")?);
        }
    }
    if !current_group.is_empty() {
        groups_p2.push(current_group);
    }

    assert_eq!(groups_p2.len(), ops.len());
    let part2 = evaluate(&ops, groups_p2)?;

    println!("Part 2: {}", part2);
    assert_eq!(part2, 9627174150897);
    Ok(())
}
