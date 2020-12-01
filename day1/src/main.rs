use simple_error::SimpleError;
use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader};

fn get_input() -> Result<Vec<u32>, Box<dyn Error>> {
    BufReader::new(std::fs::File::open("input.txt")?)
        .lines()
        .map(|l| Ok(l?.parse::<u32>()?))
        .collect::<Result<Vec<_>, Box<dyn Error>>>()
        .into()
}

fn part1() -> Result<u32, Box<dyn Error>> {
    let numbers = get_input()?;
    let mut c = HashSet::new();
    for n in numbers {
        if n > 2020 {
            continue;
        }
        if c.contains(&n) {
            return Ok(n * (2020 - n));
        }
        c.insert(2020 - n);
    }

    Err(SimpleError::new("unable to find answer to part 1").into())
}

fn part2() -> Result<u32, Box<dyn Error>> {
    let mut numbers = get_input()?;
    numbers.sort();

    let relevant_numbers = numbers
        .into_iter()
        .take_while(|n| *n < 2020)
        .collect::<Vec<_>>();

    fn le_slice(numbers: &[u32], le: u32) -> &[u32] {
        match numbers.binary_search(&le) {
            Ok(i) => {
                let n_eq = numbers[i..]
                    .iter()
                    .copied()
                    .take_while(|n| *n == le)
                    .count();
                &numbers[..=(i + n_eq - 1)]
            }
            Err(i) => &numbers[..i],
        }
    }

    for n1 in relevant_numbers.iter().copied() {
        let relevant_slice = le_slice(relevant_numbers.as_slice(), 2020 - n1);
        for n2 in relevant_slice.into_iter().copied() {
            let n3 = 2020 - n1 - n2;
            if relevant_numbers.binary_search(&n3).is_ok() {
                return Ok(n1 * n2 * n3);
            }
        }
    }

    Err(SimpleError::new("unable to find answer to part 2").into())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("part 1: {}", part1()?);
    println!("part 2: {}", part2()?);
    Ok(())
}
