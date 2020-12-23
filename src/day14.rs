use anyhow::Result;
use std::collections::HashMap;

pub fn part_one() -> Result<u64> {
    let mut or_mask: u64 = 0;
    let mut and_mask: u64 = 0;
    let mut mem = HashMap::new();
    for line in include_str!("../input/day14.txt").lines() {
        if line.starts_with("mask") {
            or_mask = 0;
            and_mask = 0;
            for (i, &byte) in line[7..].as_bytes().iter().enumerate() {
                match byte {
                    b'1' => or_mask |= 1 << (35 - i),
                    b'0' => and_mask |= 1 << (35 - i),
                    _ => {}
                }
            }
            and_mask = !and_mask;
        } else {
            let addr = line[4..].split(']').next().unwrap().parse::<u64>()?;
            let val = line.rsplit('=').next().unwrap()[1..].parse::<u64>()?;
            *mem.entry(addr).or_default() = val & and_mask | or_mask;
        }
    }
    Ok(mem.values().sum())
}
