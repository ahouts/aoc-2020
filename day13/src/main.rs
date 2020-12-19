use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};

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

const THREADS: usize = 11;
const CHECK_FREQ: usize = 100000000;

fn part2(mut busses: Vec<(u64, u64)>) -> u64 {
    let ans = Arc::new(Mutex::new(None));
    busses.sort_by_key(|(_, b)| *b);
    busses.reverse();
    let (zi, z) = busses.first().copied().unwrap();
    (0..THREADS).into_par_iter().for_each({
        let ans = ans.clone();
        move |i| {
            let mut current_ts = (z - zi) + z * i as u64;
            let mut iter_count = 0;
            loop {
                if busses
                    .iter()
                    .copied()
                    .all(|(i, b)| ((b - current_ts % b) % b) == i % b)
                {
                    let mut g = ans.lock().unwrap();
                    match *g {
                        None => *g = Some(current_ts),
                        Some(n) if current_ts < n => *g = Some(current_ts),
                        _ => (),
                    }
                    return;
                }
                current_ts += z * THREADS as u64;
                if current_ts > (u64::MAX >> 1) {
                    panic!();
                }
                iter_count += 1;
                if iter_count >= CHECK_FREQ {
                    let g = ans.lock().unwrap();
                    if let Some(n) = *g {
                        if current_ts > n {
                            return;
                        }
                    }
                    iter_count = 0;
                }
            }
        }
    });
    let g = ans.lock().unwrap();
    g.unwrap()
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

    println!("part 2: {}", part2(busses));

    Ok(())
}
