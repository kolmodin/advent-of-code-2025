use anyhow::{Result, anyhow, bail};
use itertools::Itertools;
use std::fs;

struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn up(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn down(&self) -> Coord {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn left(&self) -> Coord {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Coord {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn neighbors(&self) -> [Coord; 8] {
        [
            self.up(),
            self.down(),
            self.left(),
            self.right(),
            self.up().left(),
            self.up().right(),
            self.down().left(),
            self.down().right(),
        ]
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Grid {
    width: i64,
    height: i64,
    data: Vec<u8>,
}

impl Grid {
    fn from_str(s: &str) -> Result<Grid> {
        let height = s.lines().count();
        if height == 0 {
            bail!("empty grid");
        }

        let width = s
            .lines()
            .map(|ln| ln.len())
            .all_equal_value()
            .map_err(|_| anyhow!("grid lines have unequal lengths"))?;

        let data = s
            .lines()
            .flat_map(|ln| ln.as_bytes().iter())
            .cloned()
            .collect();

        Ok(Grid {
            width: width as i64,
            height: height as i64,
            data,
        })
    }

    fn iter_poss(&self) -> impl Iterator<Item = Coord> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| Coord { x, y })
    }

    fn within(&self, coord: &Coord) -> bool {
        coord.x >= 0 && coord.x < self.width && coord.y >= 0 && coord.y < self.height
    }

    fn get(&self, coord: &Coord) -> u8 {
        assert!(self.within(coord));
        self.data[coord.y as usize * self.width as usize + coord.x as usize]
    }

    fn set(&mut self, coord: &Coord, val: u8) {
        assert!(self.within(coord));
        self.data[coord.y as usize * self.width as usize + coord.x as usize] = val;
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(&Coord { x, y }) as char);
            }
            println!();
        }
        println!();
    }
}

fn liftable(grid: &Grid) -> Vec<Coord> {
    grid.iter_poss()
        .filter(|pos| grid.get(pos) == b'@')
        .filter(|pos| {
            pos.neighbors()
                .iter()
                .filter(|n| grid.within(n) && grid.get(n) == b'@')
                .count()
                < 4
        })
        .collect()
}

fn main() -> Result<()> {
    let mut grid = Grid::from_str(&fs::read_to_string("inputs/day04.txt")?)?;
    grid.print();

    let mut lifted = 0;
    loop {
        let liftable = liftable(&grid);
        if liftable.is_empty() {
            break;
        }

        if lifted == 0 {
            let part1 = liftable.len();
            println!("Part 1: {}", part1);
            assert_eq!(part1, 1516);
        }
        lifted += liftable.len();

        for p in &liftable {
            grid.set(p, b'x');
        }
    }

    let part2 = lifted;
    println!("Part 2: {}", part2);
    assert_eq!(part2, 9122);

    Ok(())
}
