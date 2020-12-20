use simple_error::SimpleError;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use varisat::{ExtendFormula, Solver, Var};

#[derive(Debug)]
struct Rule {
    name: String,
    constraints: Vec<RangeInclusive<u32>>,
}

struct StrWrapper<T: AsRef<str>>(T);

impl<T: AsRef<str>> TryFrom<StrWrapper<T>> for Rule {
    type Error = Box<dyn Error>;

    fn try_from(value: StrWrapper<T>) -> Result<Self, Self::Error> {
        let mut parts = value.0.as_ref().split(':').fuse();
        match (parts.next(), parts.next(), parts.next()) {
            (Some(name), Some(rules), None) => Ok(Rule {
                name: name.to_string(),
                constraints: rules
                    .split("or")
                    .map(str::trim)
                    .map(|r| {
                        let mut rule_parts = r.split('-').fuse();
                        match (rule_parts.next(), rule_parts.next(), rule_parts.next()) {
                            (Some(l), Some(u), None) => {
                                let l = l.parse()?;
                                let u = u.parse()?;
                                Ok(l..=u)
                            }
                            _ => Err(SimpleError::new("expected lower and upper bound").into()),
                        }
                    })
                    .collect::<Result<Vec<_>, Box<dyn Error>>>()?,
            }),
            _ => Err(SimpleError::new("invalid rule").into()),
        }
    }
}

#[derive(Debug, Clone)]
struct Ticket(Vec<u32>);

impl<T: AsRef<str>> TryFrom<StrWrapper<T>> for Ticket {
    type Error = ParseIntError;

    fn try_from(value: StrWrapper<T>) -> Result<Self, Self::Error> {
        Ok(Ticket(
            value
                .0
                .as_ref()
                .split(',')
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        ))
    }
}

fn parse() -> Result<(Vec<Rule>, Vec<Ticket>), Box<dyn Error>> {
    let mut on_rules = true;
    let mut rules = Vec::new();
    let mut tickets = Vec::new();

    for res in BufReader::new(File::open("input.txt")?).lines() {
        let line = res?;
        if line.is_empty() {
            continue;
        }
        match on_rules {
            true => {
                if line.as_str() == "your ticket:" {
                    on_rules = false;
                    continue;
                }
                rules.push(StrWrapper(line).try_into()?);
            }
            false => {
                if line.as_str() == "nearby tickets:" {
                    continue;
                }
                tickets.push(StrWrapper(line).try_into()?);
            }
        }
    }

    Ok((rules, tickets))
}

fn part1(rules: &[Rule], tickets: &[Ticket]) -> u32 {
    tickets
        .iter()
        .flat_map(|t| t.0.iter())
        .copied()
        .filter(|n| {
            rules
                .iter()
                .flat_map(|r| r.constraints.iter())
                .all(|c| !c.contains(n))
        })
        .sum()
}

fn part2(rules: &[Rule], tickets: &[Ticket]) -> u64 {
    let valid_tickets: Vec<Ticket> = tickets
        .iter()
        .filter(|t| {
            !t.0.iter().copied().any(|n| {
                rules
                    .iter()
                    .flat_map(|r| r.constraints.iter())
                    .all(|c| !c.contains(&n))
            })
        })
        .cloned()
        .collect();

    let mut solver = Solver::new();
    let n_fields = valid_tickets[0].0.len();
    let mut vars = Vec::new();

    for rule in rules.iter() {
        vars.push(Vec::new());
        let vars_for_rule = vars.last_mut().unwrap();
        for i in 0..n_fields {
            let is_rule_on_field = solver.new_var();
            vars_for_rule.push(is_rule_on_field);
            let is_valid = valid_tickets
                .iter()
                .all(|t| rule.constraints.iter().any(|r| r.contains(&t.0[i])));
            if !is_valid {
                solver.add_clause(&[is_rule_on_field.negative()]);
            }
        }
    }

    for vars_for_rule in vars.iter() {
        solver.add_clause(
            vars_for_rule
                .iter()
                .copied()
                .map(Var::positive)
                .collect::<Vec<_>>()
                .as_slice(),
        );
        for i in 0..(vars_for_rule.len() - 1) {
            for j in (i + 1)..vars_for_rule.len() {
                solver.add_clause(&[vars_for_rule[i].negative(), vars_for_rule[j].negative()])
            }
        }
    }

    for a in 0..n_fields {
        solver.add_clause(
            vars.iter()
                .map(|vars_for_rule| vars_for_rule[a])
                .map(Var::positive)
                .collect::<Vec<_>>()
                .as_slice(),
        );
        for i in 0..(n_fields - 1) {
            for j in (i + 1)..n_fields {
                solver.add_clause(&[vars[a][i].negative(), vars[a][j].negative()])
            }
        }
    }

    assert!(solver.solve().unwrap());
    let results = solver.model().unwrap();
    let filled_positions = results
        .into_iter()
        .filter(|l| l.is_positive())
        .map(|l| l.var())
        .collect::<HashSet<_>>();

    vars.iter()
        .enumerate()
        .map(|(i, vars_for_rule)| (vars_for_rule, &rules[i]))
        .filter(|(_, rule)| rule.name.starts_with("departure"))
        .map(|(vars_for_rule, _)| {
            vars_for_rule
                .iter()
                .enumerate()
                .filter(|(_, v)| filled_positions.contains(*v))
                .map(|(i, _)| i)
                .next()
                .unwrap()
        })
        .map(|pos| valid_tickets[0].0[pos] as u64)
        .product()
}

fn main() -> Result<(), Box<dyn Error>> {
    let (rules, tickets) = parse()?;

    println!("part 1: {}", part1(rules.as_slice(), tickets.as_slice()));
    println!("part 2: {}", part2(rules.as_slice(), tickets.as_slice()));

    Ok(())
}
