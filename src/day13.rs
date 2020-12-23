use anyhow::Result;

pub fn part_one() -> Option<usize> {
    let mut lines = include_str!("../input/day13.txt").lines();
    let timestamp = lines.next()?.parse::<usize>().ok()?;
    let bus_lines = lines
        .next()?
        .split(',')
        .flat_map(|s| s.parse::<usize>())
        .collect::<Vec<_>>();
    let earliest_time = bus_lines
        .iter()
        .map(|&id| (id, id - (timestamp % id)))
        .min_by_key(|&(_, t)| t)?;

    Some(earliest_time.0 * earliest_time.1)
}
