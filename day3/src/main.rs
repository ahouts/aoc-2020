use simple_error::SimpleError;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Index;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum Cell {
    Empty,
    Tree,
}

struct Region {
    data: Vec<Cell>,
    rows: usize,
}

impl Region {
    fn cols(&self) -> usize {
        self.data.len() / self.rows
    }
}

impl Index<(usize, usize)> for Region {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let r = index.0;
        let c = index.1 % self.cols();
        assert!(r < self.rows);

        &self.data[r * self.cols() + c]
    }
}

struct StrWrapper<T: AsRef<str>>(T);

impl<T: AsRef<str>> TryFrom<StrWrapper<T>> for Region {
    type Error = SimpleError;

    fn try_from(value: StrWrapper<T>) -> Result<Self, Self::Error> {
        let text = value.0.as_ref();
        let cols: usize = text
            .find("\n")
            .map(Ok)
            .unwrap_or(Err(SimpleError::new("no newline in input")))?;
        let data = text
            .split("\n")
            .flat_map(str::chars)
            .map(|c| match c {
                '.' => Ok(Cell::Empty),
                '#' => Ok(Cell::Tree),
                _ => Err(SimpleError::new(format!("unknown cell type {}", c))),
            })
            .collect::<Result<Vec<_>, SimpleError>>()?;
        Ok(Region {
            rows: data.len() / cols,
            data,
        })
    }
}

fn traverse_region(region: &Region, ci: usize, ri: usize) -> usize {
    let mut r = 0;
    let mut c = 0;
    let mut t = 0;
    while r < region.rows {
        if region[(r, c)] == Cell::Tree {
            t += 1;
        }

        c += ci;
        r += ri;
    }

    t
}

fn part1(region: &Region) -> usize {
    traverse_region(region, 3, 1)
}

fn part2(region: &Region) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(ci, ri)| traverse_region(region, *ci, *ri))
        .product()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut text)?;

    let region = StrWrapper(text).try_into()?;

    println!("part 1: {}", part1(&region));
    println!("part 2: {}", part2(&region));

    Ok(())
}
