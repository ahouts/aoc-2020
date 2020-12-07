use once_cell::sync::Lazy;
use regex::Regex;
use simple_error::SimpleError;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

static RULE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(no|\d+) (other|[a-z ]+) bags?,?").unwrap());

static RULES_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([a-z ]+) bags contain ([a-z, \d]+).$").unwrap());

static SHINY_GOLD: &str = "shiny gold";

struct Rule {
    descr: String,
    count: u8,
}

fn parse_rules() -> Result<HashMap<String, Vec<Rule>>, Box<dyn Error>> {
    BufReader::new(File::open("input.txt")?)
        .lines()
        .map::<Result<(String, Vec<Rule>), Box<dyn Error>>, _>(|l| {
            let l = l?;
            let captures = match RULES_REGEX.captures(l.as_str()) {
                Some(c) => c,
                None => Err(SimpleError::new(format!("rule did not match: {}", l)))?,
            };

            let target = match captures.get(1).unwrap().as_str() {
                s => String::from(s),
            };

            let matches = captures.get(2).unwrap().as_str();
            let captures: Vec<_> = RULE_REGEX
                .captures_iter(matches)
                .map::<Result<Rule, Box<dyn Error>>, _>(|capture| {
                    let count = match capture.get(1).unwrap().as_str() {
                        "no" => {
                            return Ok(Rule {
                                count: 0,
                                descr: String::new(),
                            })
                        }
                        e => e
                            .trim()
                            .parse::<u8>()
                            .map_err(|e| SimpleError::new(format!("number not match: {}", e)))?,
                    };
                    let descr = capture.get(2).unwrap().as_str().to_string();

                    Ok(Rule { descr, count })
                })
                .filter(|r| match r {
                    Ok(r) => r.count != 0,
                    Err(_) => true,
                })
                .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

            Ok((target, captures))
        })
        .collect()
}

fn part1(rules: &HashMap<String, Vec<Rule>>) -> usize {
    let references = rules
        .iter()
        .flat_map(|(src, targets)| {
            targets
                .iter()
                .map(move |target| (target.descr.clone(), src.clone()))
        })
        .fold::<HashMap<String, HashSet<String>>, _>(HashMap::new(), |mut acc, (target, dest)| {
            (*acc.entry(target).or_default()).insert(dest);
            acc
        });
    let mut can_contain_gold_bag = HashSet::new();
    let mut left_to_check = references
        .get(SHINY_GOLD)
        .map(|s| s.clone())
        .unwrap_or_else(HashSet::new);

    while !left_to_check.is_empty() {
        let mut new_left_to_check = HashSet::new();
        for to_check in left_to_check {
            if can_contain_gold_bag.contains(&to_check) {
                continue;
            }
            if let Some(refs) = references.get(&to_check) {
                for r in refs {
                    new_left_to_check.insert(r.clone());
                }
            }
            can_contain_gold_bag.insert(to_check);
        }
        left_to_check = new_left_to_check;
    }

    can_contain_gold_bag.len()
}

fn part2(rules: &HashMap<String, Vec<Rule>>) -> usize {
    let mut bag_contains = HashMap::<String, usize>::new();
    for (rule, _) in rules.iter().filter(|(_, l)| l.is_empty()) {
        bag_contains.insert(rule.clone(), 0);
    }

    while !bag_contains.contains_key(SHINY_GOLD) {
        for (rule, contains) in rules {
            if bag_contains.contains_key(rule) {
                continue;
            }
            let maybe_bag_count = contains
                .iter()
                .map(|r| {
                    bag_contains
                        .get(&r.descr)
                        .map(|containing| (*containing + 1) * r.count as usize)
                })
                .fold(Some(0), |acc, v| match (acc, v) {
                    (Some(a), Some(b)) => Some(a + b),
                    _ => None,
                });
            if let Some(count) = maybe_bag_count {
                bag_contains.insert(rule.clone(), count);
            }
        }
    }

    *bag_contains.get(SHINY_GOLD).unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let rules = parse_rules()?;

    println!("part 1: {}", part1(&rules));
    println!("part 2: {}", part2(&rules));

    Ok(())
}
