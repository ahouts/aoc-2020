use num_bigint::BigInt;
use num_traits::Zero;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

fn get_data() -> Result<Vec<BigInt>, Box<dyn Error>> {
    BufReader::new(File::open("input.txt")?)
        .lines()
        .map(|result| Ok(result?.parse()?))
        .collect()
}

fn part1(data: &[BigInt]) -> BigInt {
    let mut window: HashMap<BigInt, usize> =
        data[..25]
            .iter()
            .cloned()
            .map(|n| (n, 1))
            .fold(HashMap::new(), |mut acc, (n, c)| {
                *acc.entry(n).or_insert(0) += c;
                acc
            });
    for (to_check, to_remove) in data[25..].iter().zip(data.iter()) {
        let mut found = false;
        for x in window.keys() {
            if window.contains_key(&(to_check - x)) {
                found = true;
                break;
            }
        }

        if !found {
            return to_check.clone();
        }

        let remove_count = *window.get(to_remove).unwrap();
        if remove_count == 1 {
            window.remove(to_remove);
        } else {
            window.insert(to_remove.clone(), remove_count - 1);
        }

        *window.entry(to_check.clone()).or_default() += 1;
    }

    panic!()
}

fn part2(data: &[BigInt], invalid_number: BigInt) -> BigInt {
    for i in 0..data.len() {
        let mut j = i;
        let mut sum = BigInt::zero();
        while sum < invalid_number {
            j += 1;
            sum = data[i..j].iter().fold(BigInt::zero(), BigInt::add);
            if sum == invalid_number {
                return data[i..j].iter().min().unwrap() + data[i..j].iter().max().unwrap();
            }
        }
    }

    panic!()
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_data()?;

    let invalid_number = part1(data.as_slice());
    println!("part 1: {}", invalid_number);
    println!("part 2: {}", part2(data.as_slice(), invalid_number));

    Ok(())
}
