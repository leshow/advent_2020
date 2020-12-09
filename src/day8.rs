use anyhow::{anyhow, bail, Result};
use std::str::FromStr;

#[derive(Debug)]
pub enum Opcode {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Opcode {
    fn nop_jmp_swap(&mut self) {
        match *self {
            Opcode::Jmp(val) => *self = Opcode::Nop(val),
            Opcode::Nop(val) => *self = Opcode::Jmp(val),
            _ => (),
        }
    }
}

impl FromStr for Opcode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Opcode::*;
        let mut iter = s.split(' ');
        let op = iter
            .next()
            .ok_or_else(|| anyhow!("failed to parse opcode"))?;
        let val = iter
            .next()
            .ok_or_else(|| anyhow!("failed to parse val"))?
            .parse::<i32>()?;

        Ok(match op {
            "acc" => Acc(val),
            "jmp" => Jmp(val),
            "nop" => Nop(val),
            _ => return Err(anyhow!("failed to find opcode")),
        })
    }
}

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    executed: bool,
}

pub fn compute() -> i32 {
    let mut asm = include_str!("../input/day8.txt")
        .lines()
        .flat_map(|line| line.parse::<Opcode>())
        .map(|op| Instruction {
            opcode: op,
            executed: false,
        })
        .collect::<Vec<_>>();
    run(asm)
}

fn run(mut asm: Vec<Instruction>) -> i32 {
    let mut ip = 0_i32;
    let mut acc = 0;
    while let Some(op) = asm.get_mut(ip as usize) {
        if op.executed {
            break;
        }
        match op.opcode {
            Opcode::Acc(val) => {
                acc += val;
                ip += 1;
            }
            Opcode::Jmp(val) => {
                ip += val;
            }
            Opcode::Nop(_) => {
                ip += 1;
            }
        }
        op.executed = true;
    }

    acc
}

pub fn part_two() -> i32 {
    let mut asm = include_str!("../input/day8.txt")
        .lines()
        .flat_map(|line| line.parse::<Opcode>())
        .map(|op| Instruction {
            opcode: op,
            executed: false,
        })
        .collect::<Vec<_>>();

    let mut ip = 0_i32;
    let mut acc = 0_i32;
    while let Some(op) = asm.get_mut(ip as usize) {
        // swap and try
        op.opcode.nop_jmp_swap();
        if let Some(acc) = terminate(&mut asm) {
            return acc;
        }
        // swap back
        asm[ip as usize].opcode.nop_jmp_swap();
        ip += 1;
    }
    acc
}

fn terminate(asm: &mut Vec<Instruction>) -> Option<i32> {
    let mut ip = 0_i32;
    let mut acc = 0;
    while let Some(op) = asm.get_mut(ip as usize) {
        if op.executed {
            asm.iter_mut().for_each(|val| val.executed = false);
            return None;
        }
        match op.opcode {
            Opcode::Acc(val) => {
                acc += val;
                ip += 1;
            }
            Opcode::Jmp(val) => {
                ip += val;
            }
            Opcode::Nop(_) => {
                ip += 1;
            }
        }
        op.executed = true;
    }
    // no more instructions
    asm.iter_mut().for_each(|val| val.executed = false);
    Some(acc)
}
