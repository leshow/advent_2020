use anyhow::{anyhow, bail, Result};
use std::{borrow::Borrow, collections::HashMap, convert::TryFrom, hash::Hash, str::FromStr};

// part 1
pub fn compute() -> usize {
    include_str!("../input/day4.txt")
        .split("\n\n")
        .flat_map(|block| {
            let map = block
                .split_whitespace()
                .flat_map(|entry| {
                    let mut iter = entry.split(':');
                    Some((iter.next()?, iter.next()?))
                })
                .fold(HashMap::new(), |mut map, (key, val)| {
                    map.entry(key).or_insert(val);
                    map
                });

            Passport::try_from(map)
        })
        .count()
}

impl<V> TryFrom<HashMap<&str, V>> for Passport
where
    V: ToString,
{
    type Error = anyhow::Error;

    fn try_from(map: HashMap<&str, V>) -> Result<Self, Self::Error> {
        let birth_year = map
            .get("byr")
            .map(|v| v.to_string())
            .ok_or_else(|| anyhow!("failed"))?;
        let issue_year = map
            .get("iyr")
            .map(|v| v.to_string())
            .ok_or_else(|| anyhow!("failed"))?;
        let exp_year = map
            .get("eyr")
            .map(|v| v.to_string())
            .ok_or_else(|| anyhow!("failed"))?;
        let height = map
            .get("hgt")
            .map(|v| v.to_string())
            .ok_or_else(|| anyhow!("failed"))?;
        let hair_color = map
            .get("hcl")
            .map(|v| v.to_string())
            .ok_or_else(|| anyhow!("failed"))?;
        let eye_color = map
            .get("ecl")
            .map(|v| v.to_string())
            .ok_or_else(|| anyhow!("failed"))?;
        let pass_id = map
            .get("pid")
            .map(|v| v.to_string())
            .ok_or_else(|| anyhow!("failed"))?;
        let country_id: Option<String> = map.get("cid").map(|v| v.to_string());

        Ok(Passport {
            birth_year,
            issue_year,
            exp_year,
            height,
            hair_color,
            eye_color,
            pass_id,
            country_id,
        })
    }
}

#[derive(Debug, Clone)]
struct Passport {
    birth_year: String,
    issue_year: String,
    exp_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    pass_id: String,
    country_id: Option<String>,
}

impl Passport {
    fn valid_year<S: AsRef<str>>(s: S, min: u16, max: u16) -> bool {
        s.as_ref().len() == 4
            && matches!(s.as_ref().parse::<u16>(), Ok(num) if (min..=max).contains(&num))
    }

    fn valid_issue_year(&self) -> bool {
        Passport::valid_year(&self.issue_year, 2010, 2020)
    }
    fn valid_birth_year(&self) -> bool {
        Passport::valid_year(&self.birth_year, 1920, 2002)
    }
    fn valid_exp_year(&self) -> bool {
        Passport::valid_year(&self.exp_year, 2020, 2030)
    }
    fn valid_height(&self) -> bool {
        if let Some(i) = self.height.find("cm") {
            match self.height[..i].parse::<u16>() {
                Ok(num) => num >= 150 && num <= 193,
                Err(_) => false,
            }
        } else if let Some(i) = self.height.find("in") {
            match self.height[..i].parse::<u16>() {
                Ok(num) => num >= 59 && num <= 76,
                Err(_) => false,
            }
        } else {
            false
        }
    }
    fn valid_hair(&self) -> bool {
        self.hair_color.starts_with('#') && self.hair_color[1..].chars().all(|c| c.is_digit(16))
    }
    fn valid_eye(&self) -> bool {
        matches!(
            &self.eye_color[..],
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
        )
    }
    fn valid_pass_id(&self) -> bool {
        self.pass_id.chars().filter(|c| c.is_digit(10)).count() == 9
    }
    fn valid_cid(&self) -> bool {
        true
    }

    pub fn is_valid(&self) -> bool {
        self.valid_issue_year()
            && self.valid_birth_year()
            && self.valid_exp_year()
            && self.valid_height()
            && self.valid_hair()
            && self.valid_eye()
            && self.valid_pass_id()
            && self.valid_cid()
    }
}

// part 2
pub fn part_two() -> usize {
    include_str!("../input/day4.txt")
        .split("\n\n")
        .flat_map(|block| {
            let map = block
                .split_whitespace()
                .flat_map(|entry| {
                    let mut iter = entry.split(':');
                    Some((iter.next()?, iter.next()?))
                })
                .fold(HashMap::new(), |mut map, (key, val)| {
                    map.entry(key).or_insert(val);
                    map
                });

            Passport::try_from(map)
        })
        .filter(|pass| pass.is_valid())
        .count()
}
