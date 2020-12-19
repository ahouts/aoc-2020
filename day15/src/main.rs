use std::collections::HashMap;

const INPUT: &str = "6,19,0,5,7,13,1";

fn parse() -> Vec<u64> {
    INPUT
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn nth_spoken(start: &[u64], n: u64) -> u64 {
    let mut memory = HashMap::new();
    let mut spoken_before: Option<u64> = None;
    for (t, n) in start.iter().copied().enumerate() {
        spoken_before = memory.get(&n).copied();
        memory.insert(n, t as u64 + 1);
    }

    let mut spoken = start.last().copied().unwrap();
    for t in (start.len() as u64 + 1)..=n {
        spoken = if let Some(prev) = spoken_before {
            t - prev - 1
        } else {
            0
        };
        spoken_before = memory.get(&spoken).copied();
        memory.insert(spoken, t);
    }

    spoken
}

fn part1(start: &[u64]) -> u64 {
    nth_spoken(start, 2020)
}

fn part2(start: &[u64]) -> u64 {
    nth_spoken(start, 30000000)
}

fn main() {
    let start = parse();
    println!("part 1: {}", part1(start.as_slice()));
    println!("part 2: {}", part2(start.as_slice()));
}
