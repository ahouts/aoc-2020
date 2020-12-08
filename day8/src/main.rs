use crate::Instruction::{Acc, Jmp, Nop};
use simple_error::SimpleError;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;

type Arch = i32;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(Arch),
    Jmp(Arch),
    Nop(Arch),
}

#[derive(Debug, Default, Clone)]
struct Memory {
    ram: Vec<Instruction>,
}

impl Index<Arch> for Memory {
    type Output = Instruction;

    fn index(&self, index: i32) -> &Self::Output {
        &self.ram[index as usize]
    }
}

struct Wrapper<T>(T);

impl<R: BufRead> TryFrom<Wrapper<R>> for Memory {
    type Error = Box<dyn Error>;

    fn try_from(reader: Wrapper<R>) -> Result<Self, Self::Error> {
        Ok(Memory {
            ram: reader
                .0
                .lines()
                .map::<Result<Instruction, Box<dyn Error>>, _>(|r| {
                    let l = r?;
                    let i = l.find(" ").map(Ok).unwrap_or_else(|| {
                        Err(SimpleError::new(format!(
                            "error parsing instruction: {}",
                            l
                        )))
                    })?;
                    let v = l[(i + 1)..].parse()?;
                    match &l[..i] {
                        "acc" => Ok(Acc(v)),
                        "jmp" => Ok(Jmp(v)),
                        "nop" => Ok(Nop(v)),
                        s => Err(SimpleError::new(format!("unrecognized instruction: {}", s)))?,
                    }
                })
                .collect::<Result<Vec<Instruction>, Box<dyn Error>>>()?,
        })
    }
}

#[derive(Debug, Default)]
struct Cpu {
    pc: Arch,
    acc: Arch,
}

impl Cpu {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Acc(v) => self.acc += v,
            Jmp(o) => self.pc += o - 1,
            Nop(_) => (),
        }
        self.pc += 1;
    }
}

#[derive(Debug, Default)]
struct Console {
    cpu: Cpu,
    mem: Memory,
}

impl Console {
    fn step(&mut self) -> bool {
        let i = self.mem[self.cpu.pc];
        self.cpu.execute(i);
        self.cpu.pc as usize == self.mem.ram.len()
    }

    fn load(&mut self, mem: Memory) {
        self.mem = mem;
    }

    fn reset(&mut self) {
        *self = Console::default();
    }
}

fn part1(console: &mut Console) -> Arch {
    let mut visited = HashSet::new();
    while !visited.contains(&console.cpu.pc) {
        visited.insert(console.cpu.pc);
        console.step();
    }

    console.cpu.acc
}

fn part2(console: &mut Console, mem: Memory) -> Arch {
    let mut idx = 0;
    loop {
        let mut mem = mem.clone();
        let mut found = false;
        for i in mem.ram[idx..].iter_mut() {
            idx += 1;
            match *i {
                Acc(_) => (),
                Jmp(v) => {
                    *i = Nop(v);
                    found = true;
                    break;
                }
                Nop(v) => {
                    *i = Jmp(v);
                    found = true;
                    break;
                }
            }
        }
        if !found {
            panic!();
        }

        console.reset();
        console.load(mem);

        let mut visited = HashSet::new();
        while !visited.contains(&console.cpu.pc) {
            visited.insert(console.cpu.pc);
            if console.step() {
                return console.cpu.acc;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut console = Console::default();

    let mem: Memory = Wrapper(BufReader::new(File::open("input.txt")?)).try_into()?;
    console.load(mem.clone());

    println!("part 1: {}", part1(&mut console));
    println!("part 2: {}", part2(&mut console, mem));

    Ok(())
}
