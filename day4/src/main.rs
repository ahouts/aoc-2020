use simple_error::SimpleError;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

fn parse_passports(text: &str) -> Result<Vec<HashMap<String, String>>, SimpleError> {
    text.split("\n\n")
        .map(|p| {
            p.split_ascii_whitespace()
                .map(|e| {
                    let mut d = e.split(":");
                    let (a, b) = match (d.next(), d.next(), d.next()) {
                        (Some(a), Some(b), None) => (a, b),
                        _ => return Err(SimpleError::new("unexpected number of fields")),
                    };
                    Ok((String::from(a), String::from(b)))
                })
                .collect::<Result<HashMap<String, String>, SimpleError>>()
        })
        .collect()
}

fn part1(passports: &[HashMap<String, String>]) -> usize {
    passports
        .iter()
        .map(|p| {
            [
                p.contains_key("byr"),
                p.contains_key("iyr"),
                p.contains_key("eyr"),
                p.contains_key("hgt"),
                p.contains_key("hcl"),
                p.contains_key("ecl"),
                p.contains_key("pid"),
            ]
        })
        .filter(|r| r.iter().all(|a| *a))
        .count()
}

fn part2(passports: &[HashMap<String, String>]) -> usize {
    let valid_eye_colors: HashSet<_> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .into_iter()
        .collect();
    passports
        .iter()
        .map(|p| {
            [
                p.get("byr")
                    .map(|byr| (1920..=2002).contains(&byr.parse::<u16>().unwrap_or(0)))
                    .unwrap_or(false),
                p.get("iyr")
                    .map(|iyr| (2010..=2020).contains(&iyr.parse::<u16>().unwrap_or(0)))
                    .unwrap_or(false),
                p.get("eyr")
                    .map(|eyr| (2020..=2030).contains(&eyr.parse::<u16>().unwrap_or(0)))
                    .unwrap_or(false),
                p.get("hgt")
                    .map(|hgt| {
                        if hgt.contains("cm") {
                            (150..=193).contains(
                                &hgt.split("cm")
                                    .next()
                                    .unwrap_or("")
                                    .parse::<u8>()
                                    .unwrap_or(0),
                            )
                        } else if hgt.contains("in") {
                            (59..=76).contains(
                                &hgt.split("in")
                                    .next()
                                    .unwrap_or("")
                                    .parse::<u8>()
                                    .unwrap_or(0),
                            )
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false),
                p.get("hcl")
                    .map(|hcl| {
                        if hcl.chars().next() != Some('#') {
                            false
                        } else {
                            hcl.chars().skip(1).count() == 6
                                && hcl
                                    .chars()
                                    .skip(1)
                                    .all(|c| ('0' <= c && c <= '9') || ('a' <= c && c <= 'f'))
                        }
                    })
                    .unwrap_or(false),
                p.get("ecl")
                    .map(|ecl| valid_eye_colors.contains(ecl.as_str()))
                    .unwrap_or(false),
                p.get("pid")
                    .map(|pid| pid.len() == 9 && pid.chars().all(|c| '0' <= c && c <= '9'))
                    .unwrap_or(false),
            ]
        })
        .filter(|r| r.iter().all(|a| *a))
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    BufReader::new(File::open("input.txt")?).read_to_string(&mut text)?;

    let passports = parse_passports(text.as_str())?;

    println!("part 1: {}", part1(passports.as_slice()));
    println!("part 2: {}", part2(passports.as_slice()));

    Ok(())
}
