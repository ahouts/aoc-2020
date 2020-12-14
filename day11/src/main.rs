use crate::Spot::{Floor, Occupied, Unoccupied};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Spot {
    Floor,
    Unoccupied,
    Occupied,
}

impl From<char> for Spot {
    fn from(c: char) -> Self {
        match c {
            'L' => Unoccupied,
            '#' => Occupied,
            '.' => Floor,
            _ => panic!(),
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq)]
struct WaitingArea {
    spots: Vec<Spot>,
    cols: usize,
}

impl WaitingArea {
    fn step(&self, target: &mut WaitingArea) {
        target.cols = self.cols;
        if target.spots.len() != self.spots.len() {
            target.spots = vec![Floor; self.spots.len()];
        }
        for row in 0..(self.rows() as isize) {
            for col in 0..(self.cols as isize) {
                let spot = self.get(row, col);
                let new = match spot {
                    Some(Unoccupied)
                        if !self.get_adjacent(row, col).iter().any(|s| *s == Occupied) =>
                    {
                        Occupied
                    }
                    Some(Occupied)
                        if self
                            .get_adjacent(row, col)
                            .iter()
                            .filter(|s| **s == Occupied)
                            .count()
                            >= 4 =>
                    {
                        Unoccupied
                    }
                    Some(s) => s,
                    None => unreachable!(),
                };
                target.set(row, col, new);
            }
        }
    }

    fn step2(&self, target: &mut WaitingArea) {
        target.cols = self.cols;
        if target.spots.len() != self.spots.len() {
            target.spots = vec![Floor; self.spots.len()];
        }
        for row in 0..(self.rows() as isize) {
            for col in 0..(self.cols as isize) {
                let spot = self.get(row, col);
                let visible = self.get_visible(row, col);
                let new = match spot {
                    Some(Unoccupied) if !visible.iter().any(|s| *s == Occupied) => Occupied,
                    Some(Occupied) if visible.iter().filter(|s| **s == Occupied).count() >= 5 => {
                        Unoccupied
                    }
                    Some(s) => s,
                    None => unreachable!(),
                };
                target.set(row, col, new);
            }
        }
    }

    fn get_adjacent(&self, row: isize, col: isize) -> Vec<Spot> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|(r, c)| self.get(row + r, col + c))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
    }

    fn get_visible(&self, row: isize, col: isize) -> Vec<Spot> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .copied()
        .map(|(r, c)| {
            let mut s = 1;
            while let Some(v) = self.get(row + (r * s), col + (c * s)) {
                if v == Occupied || v == Unoccupied {
                    return Some(v);
                }
                s += 1;
            }
            None
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
    }

    fn get(&self, row: isize, col: isize) -> Option<Spot> {
        if row < 0 || col < 0 || col as usize >= self.cols {
            None
        } else {
            let idx = row as usize * self.cols + col as usize;
            if idx >= self.spots.len() {
                None
            } else {
                Some(self.spots[idx])
            }
        }
    }

    fn set(&mut self, row: isize, col: isize, val: Spot) {
        let idx = row as usize * self.cols + col as usize;
        self.spots[idx] = val;
    }

    fn rows(&self) -> usize {
        self.spots.len() / self.cols
    }
}

impl<T: AsRef<str>> From<T> for WaitingArea {
    fn from(text: T) -> Self {
        let text = text.as_ref();
        let mut spots = Vec::new();
        let mut width = 0;
        for row in text.split("\n") {
            if row.is_empty() {
                continue;
            }
            row.chars().map(Spot::from).for_each(|s| spots.push(s));
            width = row.len();
        }
        WaitingArea { cols: width, spots }
    }
}

fn part1(waiting_area: &mut WaitingArea) -> usize {
    let mut scratch = WaitingArea::default();
    while *waiting_area != scratch {
        waiting_area.step(&mut scratch);
        std::mem::swap(waiting_area, &mut scratch);
    }
    waiting_area
        .spots
        .iter()
        .copied()
        .filter(|s| *s == Occupied)
        .count()
}

fn part2(waiting_area: &mut WaitingArea) -> usize {
    let mut scratch = WaitingArea::default();
    while *waiting_area != scratch {
        waiting_area.step2(&mut scratch);
        std::mem::swap(waiting_area, &mut scratch);
    }
    waiting_area
        .spots
        .iter()
        .copied()
        .filter(|s| *s == Occupied)
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut text)?;

    let waiting_area: WaitingArea = text.into();
    println!("part 1: {}", part1(&mut waiting_area.clone()));
    println!("part 2: {}", part2(&mut waiting_area.clone()));

    Ok(())
}
