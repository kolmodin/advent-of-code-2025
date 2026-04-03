use anyhow::{Context, Result, anyhow};
use aoc_2025::input;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Pos3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos3 {
    fn dist2(self, other: Self) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl FromStr for Pos3 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(',')
            .map(|s| s.parse::<i64>().context("failed to parse coordinate"))
            .collect_tuple()
            .ok_or_else(|| anyhow!("expected 3 coordinates"))?;
        Ok(Pos3 {
            x: x?,
            y: y?,
            z: z?,
        })
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    num_components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            num_components: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            self.parent[i] = self.find(self.parent[i]);
            self.parent[i]
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let (root_i, root_j) = (self.find(i), self.find(j));
        if root_i != root_j {
            let (small, big) = if self.size[root_i] < self.size[root_j] {
                (root_i, root_j)
            } else {
                (root_j, root_i)
            };
            self.parent[small] = big;
            self.size[big] += self.size[small];
            self.num_components -= 1;
        }
    }
}

fn main() -> Result<()> {
    let input: Vec<Pos3> = input::read_input_day(8)?
        .lines()
        .map(Pos3::from_str)
        .collect::<Result<_>>()?;

    let count = input.len();
    let mut edges = (0..count)
        .tuple_combinations()
        .map(|(i, j)| (input[i].dist2(input[j]), i, j))
        .sorted_unstable();

    let mut uf = UnionFind::new(count);

    for (_dist, i, j) in edges.by_ref().take(1000) {
        uf.union(i, j);
    }

    let part1: usize = (0..count)
        .filter(|&i| uf.parent[i] == i)
        .map(|root| uf.size[root])
        .k_largest(3)
        .product();
    println!("Part 1: {}", part1);
    assert_eq!(part1, 175500);

    for (_dist, i, j) in edges {
        uf.union(i, j);
        if uf.num_components == 1 {
            let part2 = input[i].x * input[j].x;
            println!("Part 2: {}", part2);
            assert_eq!(part2, 6934702555);
            break;
        }
    }
    Ok(())
}
