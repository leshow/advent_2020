use anyhow::{bail, Result};

// part 1
pub fn compute() -> Option<usize> {
    let mut iter = include_str!("../input/day3.txt")
        .lines()
        .map(|line| line.as_bytes());
    let size = iter.next()?.iter().count();
    let mut count = 0;
    let mut pos = 3;
    for line in iter {
        if *line.get(pos)? == b'#' {
            count += 1;
        }
        pos = (pos + 3) % size;
    }
    Some(count)
}

// part 2
pub fn part_two() -> Option<usize> {
    let mut iter = include_str!("../input/day3.txt")
        .lines()
        .map(|line| line.as_bytes());
    Some(
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .flat_map(|(a, b)| compute_block(iter.clone(), *a, *b))
            .product(),
    )
}

fn compute_block<'a>(
    mut lines: impl Iterator<Item = &'a [u8]>,
    a: usize,
    b: usize,
) -> Option<usize> {
    let mut count = 0;
    let mut pos = 0;
    for line in lines.skip(b).step_by(b) {
        pos = (pos + a) % line.len();
        if *line.get(pos)? == b'#' {
            count += 1;
        }
    }
    Some(count)
}
