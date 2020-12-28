use anyhow::Result;
use std::collections::HashMap;

pub fn part_one() -> Option<usize> {
    let mut stack = vec![1, 20, 11, 6, 12, 0];
    let mut j = stack.len();
    for _ in stack.len()..2020 {
        let prev = stack.last()?;
        let new_val = stack[..stack.len() - 1]
            .iter()
            .rev()
            .position(|v| v == prev)
            .map(|i| i + 1)
            .unwrap_or(0);

        stack.push(new_val);
    }

    stack.pop()
}

pub fn part_two() -> Option<usize> {
    use std::collections::{hash_map::Entry, HashMap};

    let vals = [1, 20, 11, 6, 12, 0];
    let mut map = vals
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, (None, i + 1)))
        .collect::<HashMap<_, _>>();
    let mut prev = *vals.last()?;

    for round in vals.len() + 1..=30_000_000 {
        prev = match map.entry(prev) {
            Entry::Occupied(entry) => match *entry.get() {
                (None, _) => 0,
                (Some(before), last) => last - before,
            },
            Entry::Vacant(_) => 0,
        };

        match map.entry(prev) {
            Entry::Vacant(entry) => {
                entry.insert((None, round));
            }
            Entry::Occupied(mut entry) => match *entry.get() {
                (None, last_seen) => {
                    entry.insert((Some(last_seen), round));
                }
                (Some(_), last_seen) => {
                    entry.insert((Some(last_seen), round));
                }
            },
        }
    }
    Some(prev)
}
