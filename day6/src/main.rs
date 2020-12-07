use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

fn get_input() -> Result<String, std::io::Error> {
    let mut text = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut text)?;
    Ok(text)
}

fn get_forms(text: &str) -> Vec<HashSet<u8>> {
    text.split("\n\n")
        .map(|p| {
            p.split("\n")
                .flat_map(|s| s.as_bytes().iter())
                .cloned()
                .collect()
        })
        .collect()
}

fn get_intersecting_forms(text: &str) -> Vec<HashSet<u8>> {
    text.split("\n\n")
        .map(|p| {
            p.split("\n")
                .filter(|l| !l.is_empty())
                .map(|s| s.as_bytes().iter().cloned().collect::<HashSet<u8>>())
                .fold1(|h1, h2| h1.intersection(&h2).cloned().collect())
                .unwrap_or_else(HashSet::new)
        })
        .collect()
}

fn part1(forms: &[HashSet<u8>]) -> usize {
    forms.iter().map(HashSet::len).sum()
}

fn part2(forms: &[HashSet<u8>]) -> usize {
    forms.iter().map(HashSet::len).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = get_input()?;
    let forms = get_forms(text.as_str());
    println!("part 1: {}", part1(forms.as_slice()));

    let intersecting_forms = get_intersecting_forms(text.as_str());
    println!("part 2: {}", part2(intersecting_forms.as_slice()));

    Ok(())
}
