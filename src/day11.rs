use anyhow::Result;
use std::{convert::TryFrom, ops::Deref};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Seat {
    Floor = b'.',
    Occupied = b'#',
    Empty = b'L',
}

impl From<u8> for Seat {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Seat::Floor,
            b'#' => Seat::Occupied,
            b'L' => Seat::Empty,
            _ => unreachable!("seat type not allowed"),
        }
    }
}

#[rustfmt::skip]
const DIRS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1), 
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1)
];

pub fn compute() -> Option<usize> {
    let mut board = include_str!("../input/day11.txt")
        .lines()
        .map(|line| line.bytes().map(Seat::from).collect())
        .collect::<Vec<Vec<Seat>>>();

    Sim { board }.run(Strategy::Adj)
}

pub fn part_two() -> Option<usize> {
    let mut board = include_str!("../input/day11.txt")
        .lines()
        .map(|line| line.bytes().map(Seat::from).collect())
        .collect::<Vec<Vec<Seat>>>();

    Sim { board }.run(Strategy::FurthestAdj)
}

struct Sim {
    board: Vec<Vec<Seat>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Strategy {
    Adj,
    FurthestAdj,
}

impl Sim {
    fn new(board: Vec<Vec<Seat>>) -> Self {
        Self { board }
    }

    fn run(&mut self, strategy: Strategy) -> Option<usize> {
        while self.step(strategy)? {
            // println!("{:#?}", self.board);
        }
        Some(self.count_occupied())
    }

    fn step(&mut self, strategy: Strategy) -> Option<bool> {
        let rows = self.board.len();
        let cols = self.board.first()?.len();
        let mut new_board = self.board.clone();
        let mut changed = false;

        for y in 0..rows {
            for x in 0..cols {
                match strategy {
                    Strategy::Adj => {
                        let occupied = self
                            .get_adj(x, y)
                            .filter(|&seat| seat == Seat::Occupied)
                            .count();
                        match new_board[y][x] {
                            Seat::Empty if occupied == 0 => {
                                new_board[y][x] = Seat::Occupied;
                                changed = true;
                            }
                            Seat::Occupied if occupied >= 4 => {
                                new_board[y][x] = Seat::Empty;
                                changed = true;
                            }
                            _ => {}
                        }
                    }
                    Strategy::FurthestAdj => {
                        let occupied = self
                            .get_furthest_adj(x, y)
                            .filter(|&seat| seat == Seat::Occupied)
                            .count();
                        match new_board[y][x] {
                            Seat::Empty if occupied == 0 => {
                                new_board[y][x] = Seat::Occupied;
                                changed = true;
                            }
                            Seat::Occupied if occupied >= 5 => {
                                new_board[y][x] = Seat::Empty;
                                changed = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        if changed {
            self.board = new_board;
        }

        Some(changed)
    }

    fn count_occupied(&self) -> usize {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .copied()
                    .filter(|&seat| seat == Seat::Occupied)
                    .count()
            })
            .sum()
    }

    fn get_furthest_adj(&self, x: usize, y: usize) -> impl Iterator<Item = Seat> + '_ {
        DIRS.iter().filter_map(move |(dx, dy)| {
            (1..)
                .map(|distance| {
                    Some(
                        *self
                            .board
                            .get(((y as isize) + dy * distance) as usize)?
                            .get(((x as isize) + dx * distance) as usize)?,
                    )
                })
                .find(|&place| place != Some(Seat::Floor))
                .flatten()
        })
    }

    fn get_adj(&self, x: usize, y: usize) -> impl Iterator<Item = Seat> + '_ {
        DIRS.iter().copied().flat_map(move |(dx, dy)| {
            Some(
                *self
                    .board
                    .get((y as isize + dy) as usize)?
                    .get((x as isize + dx) as usize)?,
            )
        })
    }
}
