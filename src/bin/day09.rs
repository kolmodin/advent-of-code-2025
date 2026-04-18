use anyhow::{Context, Result, anyhow};
use aoc_2025::input;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::str::FromStr;

const NEIGHBORS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos2 {
    x: i64,
    y: i64,
}

impl Pos2 {
    fn area(self, other: &Self) -> i64 {
        let width = (self.x - other.x).abs() + 1;
        let height = (self.y - other.y).abs() + 1;
        width * height
    }

    fn neighbors(&self) -> [Pos2; 8] {
        NEIGHBORS.map(|(dx, dy)| Pos2 {
            x: self.x + dx,
            y: self.y + dy,
        })
    }
}

impl FromStr for Pos2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split(',')
            .map(|s| s.parse::<i64>().context("failed to parse coordinate"))
            .collect_tuple()
            .ok_or_else(|| anyhow!("expected 2 coordinates"))?;

        Ok(Pos2 { x: x?, y: y? })
    }
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    top_left: Pos2,
    bottom_right: Pos2,
}

impl Rect {
    fn new(a: &Pos2, b: &Pos2) -> Self {
        Rect {
            top_left: Pos2 {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            },
            bottom_right: Pos2 {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
            },
        }
    }

    fn contains(&self, pos: &Pos2) -> bool {
        self.top_left.x <= pos.x
            && pos.x <= self.bottom_right.x
            && self.top_left.y <= pos.y
            && pos.y <= self.bottom_right.y
    }

    fn left(&self) -> VLine {
        VLine {
            x: self.top_left.x,
            y_min: self.top_left.y,
            y_max: self.bottom_right.y,
        }
    }

    fn right(&self) -> VLine {
        VLine {
            x: self.bottom_right.x,
            y_min: self.top_left.y,
            y_max: self.bottom_right.y,
        }
    }

    fn top(&self) -> HLine {
        HLine {
            y: self.top_left.y,
            x_min: self.top_left.x,
            x_max: self.bottom_right.x,
        }
    }

    fn bottom(&self) -> HLine {
        HLine {
            y: self.bottom_right.y,
            x_min: self.top_left.x,
            x_max: self.bottom_right.x,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct HLine {
    y: i64,
    x_min: i64,
    x_max: i64,
}

#[derive(Debug, Clone, Copy)]
struct VLine {
    x: i64,
    y_min: i64,
    y_max: i64,
}

impl VLine {
    fn intersection(&self, hline: &HLine) -> Option<Pos2> {
        if hline.x_min <= self.x
            && self.x <= hline.x_max
            && self.y_min <= hline.y
            && hline.y <= self.y_max
        {
            Some(Pos2 {
                x: self.x,
                y: hline.y,
            })
        } else {
            None
        }
    }
}

fn pos_is_red_or_green(pos: &Pos2, vlines: &[VLine], hlines: &[HLine]) -> bool {
    // If the point is on a horizontal line, it's "green"
    if hlines
        .iter()
        .take_while(|h| h.y <= pos.y)
        .any(|h| h.y == pos.y && pos.x >= h.x_min && pos.x <= h.x_max)
    {
        return true;
    }

    let mut intersections = 0;
    for v in vlines {
        if v.x > pos.x {
            break;
        }
        if v.x <= pos.x && pos.y >= v.y_min && pos.y <= v.y_max {
            if v.x == pos.x {
                return true; // On a vertical line
            }
            // Ray casting: count intersections with segments to the left.
            // Using strictly greater than v.y_min ensures we don't double-count vertices.
            if pos.y > v.y_min {
                intersections += 1;
            }
        }
    }
    intersections % 2 == 1
}

fn is_rect_only_red_and_green(
    rect: &Rect,
    _all_corners: &HashSet<Pos2>,
    hlines: &[HLine],
    vlines: &[VLine],
) -> bool {
    let mut poi: Vec<Pos2> = vec![rect.top_left, rect.bottom_right];

    // The input is kind, this check is not needed.
    // poi.extend(all_corners.iter().filter(|c| rect.contains(c)));

    // add all points where a line crosses the rect to poi
    for h in hlines {
        poi.extend(rect.left().intersection(h));
        poi.extend(rect.right().intersection(h));
    }

    for v in vlines {
        poi.extend(v.intersection(&rect.top()));
        poi.extend(v.intersection(&rect.bottom()));
    }

    poi.into_iter()
        .flat_map(|p| p.neighbors())
        .filter(|p| rect.contains(p))
        .unique()
        .all(|p| pos_is_red_or_green(&p, vlines, hlines))
}

fn main() -> Result<()> {
    let mut input: Vec<Pos2> = input::read_input_day(9)?
        .lines()
        .map(Pos2::from_str)
        .collect::<Result<_>>()?;

    let part1 = input
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.area(b))
        .max()
        .context("no input")?;

    println!("Part 1: {}", part1);
    assert_eq!(part1, 4750092396);

    let mut horizontal = vec![];
    let mut vertical = vec![];
    input.push(input[0]);

    for &[a, b] in input.array_windows::<2>() {
        if a.x == b.x {
            vertical.push(VLine {
                x: a.x,
                y_min: a.y.min(b.y),
                y_max: a.y.max(b.y),
            });
        } else {
            horizontal.push(HLine {
                y: a.y,
                x_min: a.x.min(b.x),
                x_max: a.x.max(b.x),
            });
        }
    }
    horizontal.sort_unstable_by_key(|h| h.y);
    vertical.sort_unstable_by_key(|v| v.x);

    let all_corners: HashSet<Pos2> = FromIterator::from_iter(input);

    // input has extra pos
    let part2 = all_corners
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (Rect::new(a, b), a.area(b)))
        .sorted_unstable_by_key(|&(_, area)| Reverse(area))
        .find(|(r, _)| is_rect_only_red_and_green(r, &all_corners, &horizontal, &vertical))
        .context("no rect")?;

    println!("Part 2: {}", part2.1);
    assert_eq!(part2.1, 1468516555);

    Ok(())
}
