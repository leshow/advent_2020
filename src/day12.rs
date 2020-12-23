use anyhow::Result;
use std::str::FromStr;

use Heading::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Heading((Heading, isize)),
    L(isize),
    R(isize),
    F(isize),
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (&s[0..1], &s[1..]) {
            ("N", n) => Ok(Dir::Heading((Heading::North, n.parse()?))),
            ("S", n) => Ok(Dir::Heading((Heading::South, n.parse()?))),
            ("E", n) => Ok(Dir::Heading((Heading::East, n.parse()?))),
            ("W", n) => Ok(Dir::Heading((Heading::West, n.parse()?))),
            ("L", n) => Ok(Dir::L(n.parse()?)),
            ("R", n) => Ok(Dir::R(n.parse()?)),
            ("F", n) => Ok(Dir::F(n.parse()?)),
            _ => Err(anyhow::anyhow!("failed to parse direction")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Heading {
    North,
    South,
    East,
    West,
}

impl From<isize> for Heading {
    fn from(num: isize) -> Self {
        match num {
            0 => North,
            90 => East,
            180 => South,
            270 => West,
            _ => unreachable!("angle not supported"),
        }
    }
}

impl From<Heading> for isize {
    fn from(heading: Heading) -> Self {
        match heading {
            North => 0,
            East => 90,
            South => 180,
            West => 270,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pos {
    head: Heading,
    x: isize,
    y: isize,
}

impl Pos {
    fn origin() -> Self {
        Pos {
            head: Heading::East,
            x: 0,
            y: 0,
        }
    }
    fn change(&mut self, dir: Dir) -> &mut Self {
        match dir {
            Dir::Heading((head, dist)) => match head {
                East => self.x += dist,
                West => self.x -= dist,
                South => self.y -= dist,
                North => self.y += dist,
            },
            Dir::F(dist) => match self.head {
                East => self.x += dist,
                West => self.x -= dist,
                South => self.y -= dist,
                North => self.y += dist,
            },
            Dir::L(angle) => {
                self.head = ((isize::from(self.head) + 360 - angle) % 360).into();
            }
            Dir::R(angle) => {
                self.head = ((isize::from(self.head) + angle) % 360).into();
            }
        }
        self
    }
}

pub fn part_one() -> Result<isize> {
    let mut pos = Pos::origin();
    let pos = include_str!("../input/day12.txt")
        .lines()
        .flat_map(|line| line.parse::<Dir>())
        .fold(&mut pos, |acc, dist| acc.change(dist));

    Ok(pos.x.abs() + pos.y.abs())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Ship {
    pos: Pos,
    waypoint: Waypoint,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Waypoint {
    x: isize,
    y: isize,
}

impl Waypoint {
    fn swap(&mut self, angle: isize) {
        match angle {
            0 => {}
            90 => {
                let tmp = self.y;
                self.y = -self.x;
                self.x = tmp;
            }
            180 => {
                self.x = -self.x;
                self.y = -self.y;
            }
            270 => {
                let tmp = self.y;
                self.y = self.x;
                self.x = -tmp;
            }
            _ => unreachable!(),
        }
    }
}

impl Ship {
    fn origin() -> Self {
        Self {
            pos: Pos::origin(),
            waypoint: Waypoint { x: 10, y: 1 },
        }
    }
    fn change(&mut self, dir: Dir) -> &mut Self {
        match dir {
            Dir::Heading((head, dist)) => match head {
                East => self.waypoint.x += dist,
                West => self.waypoint.x -= dist,
                South => self.waypoint.y -= dist,
                North => self.waypoint.y += dist,
            },
            Dir::F(dist) => {
                self.pos.x += self.waypoint.x * dist;
                self.pos.y += self.waypoint.y * dist;
            }
            Dir::L(angle) => self.waypoint.swap(360 - angle),
            Dir::R(angle) => self.waypoint.swap(angle),
        }
        self
    }
}

pub fn part_two() -> Result<isize> {
    let mut ship = Ship::origin();
    let ship = include_str!("../input/day12.txt")
        .lines()
        .flat_map(|line| line.parse::<Dir>())
        .fold(&mut ship, |acc, dist| acc.change(dist));

    Ok(ship.pos.x.abs() + ship.pos.y.abs())
}
