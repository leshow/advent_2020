use anyhow::Result;
pub fn compute() -> Option<u32> {
    // could speed this up with a mashmap but whatever..
    let input = include_str!("../input/day9.txt")
        .lines()
        .flat_map(|x| x.parse::<u32>())
        .collect::<Vec<_>>();
    let len = 25;
    input
        .windows(len + 1)
        .enumerate()
        .find(|(i, nums)| {
            let n = nums[len];
            !&nums[0..len]
                .iter()
                .any(|&v| n.checked_sub(v).map(|r| nums.contains(&r)).unwrap_or(false))
        })
        .map(|(_, n)| n[len])
}

// O(n^2)
pub fn part_two() -> Option<u64> {
    let input = include_str!("../input/day9.txt")
        .lines()
        .flat_map(|x| x.parse::<u64>())
        .collect::<Vec<_>>();

    const MAGIC: u64 = 85_848_519;
    // brute force the window size
    for i in 0..input.len() {
        for j in 1..=(input.len() - i) {
            if MAGIC == input[i..i + j].iter().sum() {
                let min = input[i..i + j].iter().min()?;
                let max = input[i..i + j].iter().max()?;
                return Some(min + max);
            }
        }
    }
    None
}

// O(n) use a sliding window instead
pub fn part_two_fast() -> Option<u64> {
    let input = include_str!("../input/day9.txt")
        .lines()
        .flat_map(|x| x.parse::<u64>())
        .collect::<Vec<_>>();

    const MAGIC: u64 = 85_848_519;
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;
    while sum != MAGIC {
        if sum < MAGIC {
            sum += input[j];
            j += 1;
        } else {
            sum -= input[i];
            i += 1;
        }
    }
    let min = input[i..j].iter().min()?;
    let max = input[i..j].iter().max()?;

    Some(min + max)
}
