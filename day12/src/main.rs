use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum CardinalDirection {
    East,
    North,
    West,
    South,
}

impl CardinalDirection {
    fn rot(self, direction: RelativeDirection) -> Self {
        match self {
            CardinalDirection::North => match direction {
                RelativeDirection::Left => CardinalDirection::West,
                RelativeDirection::Right => CardinalDirection::East,
            },
            CardinalDirection::South => match direction {
                RelativeDirection::Left => CardinalDirection::East,
                RelativeDirection::Right => CardinalDirection::West,
            },
            CardinalDirection::West => match direction {
                RelativeDirection::Left => CardinalDirection::South,
                RelativeDirection::Right => CardinalDirection::North,
            },
            CardinalDirection::East => match direction {
                RelativeDirection::Left => CardinalDirection::North,
                RelativeDirection::Right => CardinalDirection::South,
            },
        }
    }
}

#[derive(Debug)]
struct Ship {
    heading: CardinalDirection,
    x: i64,
    y: i64,
}

impl Ship {
    fn step(&mut self, i: Instruction) {
        match i {
            Instruction::Move { direction, amount } => self.move_direction(direction, amount),
            Instruction::Forward { amount } => self.move_direction(self.heading, amount),
            Instruction::Turn { direction, deg } => {
                for _ in 0..(deg / 90) {
                    self.heading = self.heading.rot(direction);
                }
            }
        }
    }

    fn move_direction(&mut self, direction: CardinalDirection, amount: i64) {
        match direction {
            CardinalDirection::North => self.y += amount,
            CardinalDirection::South => self.y -= amount,
            CardinalDirection::East => self.x += amount,
            CardinalDirection::West => self.x -= amount,
        }
    }

    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            heading: CardinalDirection::East,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum RelativeDirection {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Move {
        direction: CardinalDirection,
        amount: i64,
    },
    Turn {
        direction: RelativeDirection,
        deg: u16,
    },
    Forward {
        amount: i64,
    },
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let amount: i64 = s[1..].parse().unwrap();
        match s.chars().next().unwrap() {
            'N' => Instruction::Move {
                direction: CardinalDirection::North,
                amount,
            },
            'S' => Instruction::Move {
                direction: CardinalDirection::South,
                amount,
            },
            'E' => Instruction::Move {
                direction: CardinalDirection::East,
                amount,
            },
            'W' => Instruction::Move {
                direction: CardinalDirection::West,
                amount,
            },
            'L' => Instruction::Turn {
                direction: RelativeDirection::Left,
                deg: (amount % 360) as u16,
            },
            'R' => Instruction::Turn {
                direction: RelativeDirection::Right,
                deg: (amount % 360) as u16,
            },
            'F' => Instruction::Forward { amount },
            _ => panic!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Waypoint {
    x: i64,
    y: i64,
}

impl Waypoint {
    fn step(&mut self, ship: &mut Ship, i: Instruction) {
        match i {
            Instruction::Move { direction, amount } => match direction {
                CardinalDirection::North => self.y += amount,
                CardinalDirection::South => self.y -= amount,
                CardinalDirection::East => self.x += amount,
                CardinalDirection::West => self.x -= amount,
            },
            Instruction::Turn { direction, deg } => match direction {
                RelativeDirection::Left => {
                    for _ in 0..(deg / 90) {
                        let x = self.x;
                        self.x = -self.y;
                        self.y = x;
                    }
                }
                RelativeDirection::Right => {
                    for _ in 0..(deg / 90) {
                        let x = self.x;
                        self.x = self.y;
                        self.y = -x;
                    }
                }
            },
            Instruction::Forward { amount } => {
                ship.x += self.x * amount;
                ship.y += self.y * amount;
            }
        }
    }
}

impl Default for Waypoint {
    fn default() -> Self {
        Waypoint { x: 10, y: 1 }
    }
}

fn parse() -> Result<Vec<Instruction>, Box<dyn Error>> {
    BufReader::new(File::open("input.txt")?)
        .lines()
        .map(|r| Ok(r?.as_str().into()))
        .collect::<Result<Vec<Instruction>, Box<dyn Error>>>()
}

fn part1(ins: &[Instruction]) -> i64 {
    let mut ship = Ship::default();
    for i in ins {
        ship.step(*i);
    }
    ship.manhattan_distance()
}

fn part2(ins: &[Instruction]) -> i64 {
    let mut ship = Ship::default();
    let mut waypoint = Waypoint::default();
    for i in ins {
        waypoint.step(&mut ship, *i);
    }
    ship.manhattan_distance()
}

fn main() -> Result<(), Box<dyn Error>> {
    let ins = parse()?;

    println!("part 1: {}", part1(ins.as_slice()));
    println!("part 2: {}", part2(ins.as_slice()));

    Ok(())
}
