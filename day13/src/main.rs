use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse() -> Result<(u64, Vec<(u64, u64)>), Box<dyn Error>> {
    let mut lines = BufReader::new(File::open("input.txt")?).lines();
    let start: u64 = lines.next().unwrap()?.parse()?;
    let busses = lines
        .next()
        .unwrap()?
        .split(',')
        .enumerate()
        .filter(|(_, b)| *b != "x")
        .map(|(i, b)| Ok((i as u64, b.parse()?)))
        .collect::<Result<Vec<(u64, u64)>, std::num::ParseIntError>>()?;
    Ok((start, busses))
}

fn part1(start: u64, busses: &[u64]) -> u64 {
    let bus = busses
        .iter()
        .copied()
        .min_by_key(|b| *b - start % *b)
        .unwrap();
    let mut departure_time = 0;
    while departure_time < start {
        departure_time += bus;
    }
    bus * (departure_time - start)
}

fn part2(busses: &[(u64, u64)]) -> u64 {
    let mut current_ts = 0;
    let z = busses.first().unwrap().1;
    loop {
        if busses
            .iter()
            .copied()
            .all(|(i, b)| ((b - current_ts % b) % b) == i % b)
        {
            return current_ts;
        }
        current_ts += z;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (start, busses) = parse()?;

    println!(
        "part 1: {}",
        part1(
            start,
            busses
                .iter()
                .copied()
                .map(|(_, b)| b)
                .collect::<Vec<_>>()
                .as_slice()
        )
    );

    println!("part 2: {}", part2(busses.as_slice()));

    Ok(())
}
