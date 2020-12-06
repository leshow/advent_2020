use anyhow::{bail, Result};

pub fn compute() -> Result<usize> {
    let iter = include_str!("../input/day1.txt")
        .lines()
        .flat_map(|a| a.parse::<usize>().ok());

    for (i, a) in iter.clone().enumerate() {
        for b in iter.clone().skip(i) {
            if a + b == 2020 {
                return Ok(a * b);
            }
        }
    }

    bail!("failed to find matching result")
}

pub fn part_two() -> Result<usize> {
    let iter = include_str!("../input/day1.txt")
        .lines()
        .flat_map(|a| a.parse::<usize>().ok());

    for (i, a) in iter.clone().enumerate() {
        for (j, b) in iter.clone().enumerate().skip(i) {
            for c in iter.clone().skip(i + j) {
                if a + b + c == 2020 {
                    return Ok(a * b * c);
                }
            }
        }
    }

    bail!("failed to find matching result")
}
