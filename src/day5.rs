use anyhow::Result;
use std::cmp;

pub fn compute() -> Option<u16> {
    include_str!("../input/day5.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'B' | 'R' => 1,
                    _ => 0,
                })
                .fold(0, |acc, bit| acc * 2 + bit)
        })
        .max()
}

pub fn part_two() -> u16 {
    let (num, min, max) = include_str!("../input/day5.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'B' | 'R' => 1,
                    _ => 0,
                })
                .fold(0, |acc, bit| acc * 2 + bit)
        })
        .fold((0, u16::MAX, u16::MIN), |(acc, min, max), num| {
            (acc ^ num, cmp::min(min, num), cmp::max(max, num))
        });
    (min..=max).fold(num, |acc, a| acc ^ a)
}
