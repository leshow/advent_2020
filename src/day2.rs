use anyhow::Result;
use std::ops::RangeInclusive;

// part 1
pub fn compute() -> Result<usize> {
    Ok(include_str!("../input/day2.txt")
        .lines()
        .flat_map(parse_line)
        .filter(contains)
        .count())
}

fn parse_line(input: &str) -> Option<(RangeInclusive<usize>, char, &str)> {
    let mut iter = input.split(':');
    let mut first = iter.next()?.split(' ');
    let snd = &iter.next()?[1..];

    let mut range = first.next()?.split('-');
    let letter = first.next()?;

    Some((
        (range.next()?.parse::<usize>().ok()?..=range.next()?.parse::<usize>().ok()?),
        letter.parse::<char>().ok()?,
        snd,
    ))
}

fn contains((range, c, s): &(RangeInclusive<usize>, char, &str)) -> bool {
    let count = s.chars().filter(|x| c == x).count();
    range.contains(&count)
}

// part 2
pub fn part_two() -> Result<usize> {
    Ok(include_str!("../input/day2.txt")
        .lines()
        .flat_map(parse_line)
        .flat_map(contains_two)
        .filter(|b| *b)
        .count())
}

fn contains_two((range, c, s): (RangeInclusive<usize>, char, &str)) -> Option<bool> {
    Some((s.chars().nth(*range.start() - 1)? == c) ^ (s.chars().nth(*range.end() - 1)? == c))
}
