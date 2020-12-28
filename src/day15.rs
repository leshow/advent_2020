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
