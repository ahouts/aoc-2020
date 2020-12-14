use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse() -> Result<Vec<i64>, Box<dyn Error>> {
    BufReader::new(File::open("input.txt")?)
        .lines()
        .map(|r| Ok(r?.parse()?))
        .collect()
}

fn part1(nums: &[i64]) -> i64 {
    let (one_diff, three_diff) = std::iter::once(0)
        .chain(nums.iter().copied())
        .zip(
            nums.iter()
                .copied()
                .chain(std::iter::once(nums.last().copied().unwrap() + 3)),
        )
        .map(|(p, n)| {
            (
                if n - p == 1 { 1 } else { 0 },
                if n - p == 3 { 1 } else { 0 },
            )
        })
        .fold((0, 0), |(o1, t1), (o2, t2)| (o1 + o2, t1 + t2));
    one_diff * three_diff
}

fn part2(nums: &[i64]) -> i64 {
    fn calculate_configurations(c: &HashMap<i64, i64>, n: i64) -> i64 {
        *c.get(&(n - 1)).unwrap_or(&0)
            + *c.get(&(n - 2)).unwrap_or(&0)
            + *c.get(&(n - 3)).unwrap_or(&0)
    }

    let mut c = HashMap::new();
    c.insert(0, 1);
    for n in nums.iter().copied() {
        c.insert(n, calculate_configurations(&c, n));
    }
    calculate_configurations(&c, *nums.last().unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut nums = parse()?;
    nums.sort_unstable();
    println!("part 1: {}", part1(nums.as_slice()));
    println!("part 2: {}", part2(nums.as_slice()));
    Ok(())
}
