use anyhow::{bail, Result};
use std::collections::HashSet;

// part 1
pub fn compute() -> usize {
    include_str!("../input/day6.txt")
        .split("\n\n")
        .map(|block| {
            block
                .split_whitespace()
                .map(|c| c.chars())
                .flatten()
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

pub fn part_two() -> usize {
    include_str!("../input/day6.txt")
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|c| c.chars().collect::<HashSet<_>>())
                .fold(None, |acc, set| match acc {
                    None => Some(set),
                    Some(acc) => Some(acc.intersection(&set).cloned().collect()),
                })
                .map(|num| num.len())
                .unwrap_or(0)
        })
        .sum()
}
