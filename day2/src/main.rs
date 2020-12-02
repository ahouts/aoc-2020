use itertools::Itertools;
use simple_error::SimpleError;
use std::error::Error;
use std::io::{BufRead, BufReader};

struct Rule {
    start: u8,
    end: u8,
    character: u8,
}

struct Password(Vec<u8>);

struct Statement {
    rule: Rule,
    password: Password,
}

trait StatementExt {
    fn parse(&self) -> Result<Statement, Box<dyn Error>>;
}

impl<T: AsRef<str>> StatementExt for T {
    fn parse(&self) -> Result<Statement, Box<dyn Error>> {
        let s = self.as_ref().as_bytes();

        fn split_at(s: &[u8], c: u8) -> Result<(&[u8], &[u8]), Box<dyn Error>> {
            let i = s
                .iter()
                .enumerate()
                .filter(|(_, x)| **x == c)
                .map(|(i, _)| i)
                .next()
                .map(Ok)
                .unwrap_or_else(|| {
                    Err(SimpleError::new(format!(
                        "malformed rule, missing '{}'",
                        c as char
                    )))
                })?;
            let (a, b) = s.split_at(i);
            Ok((a, &b[1..]))
        }

        let (rule, password) = split_at(s, b':')?;
        let (range, character) = split_at(rule, b' ')?;
        let (start, end) = split_at(range, b'-')?;

        Ok(Statement {
            rule: Rule {
                start: std::str::from_utf8(start)?.parse()?,
                end: std::str::from_utf8(end)?.parse()?,
                character: character[0],
            },
            password: Password(password.to_vec()),
        })
    }
}

fn sum_by_rule_application<F>(f: F) -> Result<usize, Box<dyn Error>>
where
    F: Fn(Statement) -> usize,
{
    Ok(BufReader::new(std::fs::File::open("input.txt")?)
        .lines()
        .map::<Result<Statement, Box<dyn Error>>, _>(|r| Ok(r?.parse()?))
        .map::<Result<usize, Box<dyn Error>>, _>(|r| Ok(f(r?)))
        .fold_results(0, std::ops::Add::add)?)
}

fn part1() -> Result<usize, Box<dyn Error>> {
    sum_by_rule_application(|statement| {
        let char_count = statement
            .password
            .0
            .iter()
            .filter(|c| **c == statement.rule.character)
            .count();
        if statement.rule.start as usize <= char_count && char_count <= statement.rule.end as usize
        {
            1
        } else {
            0
        }
    })
}

fn part2() -> Result<usize, Box<dyn Error>> {
    sum_by_rule_application(|statement| {
        let c1 = statement.password.0[statement.rule.start as usize];
        let c2 = statement.password.0[statement.rule.end as usize];
        if (c1 == statement.rule.character) ^ (c2 == statement.rule.character) {
            1
        } else {
            0
        }
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("part 1: {}", part1()?);
    println!("part 2: {}", part2()?);
    Ok(())
}
