use std::str::FromStr;

use anyhow::{Context, Result};

use aoc_2025::input;

#[derive(Debug)]
struct Machine {
    lights: u16,
    buttons: Vec<Button>,
    joltage: Vec<u16>,
}

#[derive(Debug)]
struct Button {
    triggers: Vec<u16>,
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let lights_str = words.next().context("missing lights string")?;
        let joltage_str = words.next_back().context("missing jolting string")?;

        let lights = lights_str
            .trim_matches(&['[', ']'])
            .bytes()
            .enumerate()
            .fold(
                0u16,
                |acc, (i, b)| if b == b'#' { acc | (1 << i) } else { acc },
            );

        let buttons = words
            .map(|word| {
                Ok(Button {
                    triggers: word
                        .trim_matches(&['(', ')'])
                        .split(',')
                        .map(|s| s.parse::<u16>().context("could not parse button"))
                        .collect::<Result<Vec<u16>>>()?,
                })
            })
            .collect::<Result<Vec<Button>>>()?;

        let joltage = joltage_str
            .trim_matches(&['{', '}'])
            .split(',')
            .map(|s| s.parse::<u16>().context("could not parse joltage"))
            .collect::<Result<Vec<u16>>>()?;

        Ok(Machine {
            lights,
            buttons,
            joltage,
        })
    }
}

fn min_pushed_buttons(machine: &Machine) -> Result<u16> {
    let buttons = machine
        .buttons
        .iter()
        .map(|b| b.triggers.iter().fold(0u16, |acc, n| acc | (1 << n)));

    let mut combinations = vec![0u16];
    for button in buttons {
        let len = combinations.len();
        for i in 0..len {
            combinations.push(combinations[i] ^ button);
        }
    }

    combinations
        .into_iter()
        .enumerate()
        .filter(|&(_, lights)| lights == machine.lights)
        .map(|(i, _)| i.count_ones() as u16)
        .min()
        .context("no solution found")
}

fn main() -> Result<()> {
    let input = input::read_input_day(10)?
        .lines()
        .map(Machine::from_str)
        .collect::<Result<Vec<Machine>>>()?;

    let part1: u16 = input.iter().map(min_pushed_buttons).sum::<Result<u16>>()?;

    println!("Part 1: {}", part1);
    assert_eq!(part1, 571);

    Ok(())
}
