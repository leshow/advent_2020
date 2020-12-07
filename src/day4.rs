use anyhow::{bail, Result};
use std::str::FromStr;

// part 1
pub fn compute() -> Option<usize> {
    let mut iter = include_str!("../input/day4.txt").lines();
    todo!()
}

enum PassportField {
    BirthYear,
    IssueYear,
    ExpYear,
    Height,
    HairColor,
    EyeColor,
    PassId,
    CountryId,
}

impl FromStr for PassportField {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
