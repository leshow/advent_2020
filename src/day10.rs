use std::collections::BTreeMap;

use anyhow::Result;

pub fn compute() -> Option<u16> {
    let mut input = include_str!("../input/day10.txt")
        .lines()
        .flat_map(|x| x.parse::<u16>())
        .collect::<Vec<_>>();
    input.sort_unstable();

    let (one, three) =
        input
            .windows(2)
            .fold((1, 1), |(one, three), nums| match nums[1] - nums[0] {
                1 => (one + 1, three),
                3 => (one, three + 1),
                _ => (one, three),
            });
    Some(one * three)
}

pub fn part_two_naive() -> Option<usize> {
    use std::collections::BTreeMap;
    use std::collections::HashSet;
    let mut input = include_str!("../input/day10.txt")
        .lines()
        .flat_map(|x| x.parse::<i32>())
        .collect::<HashSet<_>>();

    // first impl did not use memoization and would just run indefinitely-- though correct
    fn count(arr: &HashSet<i32>, t: i32, mem: &mut BTreeMap<i32, usize>) -> usize {
        if let Some(x) = mem.get(&t) {
            return *x;
        }
        if t == 0 {
            mem.insert(t, 1);
            return 1;
        }
        if t <= 0 || !arr.contains(&t) {
            mem.insert(t, 0);
            return 0;
        }
        let ret = count(arr, t - 1, mem) + count(arr, t - 2, mem) + count(arr, t - 3, mem);
        mem.insert(t, ret);
        ret
    }
    Some(count(&input, *input.iter().max()?, &mut BTreeMap::new()))
}

pub fn part_two_faster() -> u64 {
    use std::collections::BTreeMap;
    use std::collections::HashSet;
    let mut input = include_str!("../input/day10.txt")
        .lines()
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<_>>();
    input.push(0);
    input.sort_unstable();

    let mut edges = BTreeMap::new();
    for edge in &input {
        edges.entry(edge).or_insert_with(Vec::new).extend(
            (1..4)
                .map(|n| edge + n)
                .flat_map(|b| input.binary_search(&b)),
        );
    }
    // now count the paths
    let mut count = BTreeMap::new();
    for v in input.iter().rev() {
        let adj = &edges[&v];
        if adj.is_empty() {
            count.insert(v, 1);
        } else {
            count.insert(v, adj.iter().map(|n| count[n]).sum());
        }
    }

    count[&0]
}
