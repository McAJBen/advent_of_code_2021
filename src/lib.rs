use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    iter::Peekable,
};

pub fn read_lines(file: &File) -> Vec<String> {
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

pub struct ZipWithNext<Iter, Item>
where
    Iter: Iterator<Item = Item>,
    Item: Clone + Copy,
{
    iter: Peekable<Iter>,
}

impl<Iter, Item> Iterator for ZipWithNext<Iter, Item>
where
    Iter: Iterator<Item = Item>,
    Item: Clone + Copy,
{
    type Item = (Item, Item);

    fn next(&mut self) -> Option<Self::Item> {
        let left = self.iter.next();
        let right = self.iter.peek();

        match (left, right) {
            (Some(left), Some(right)) => Some((left, *right)),
            _ => None,
        }
    }
}

pub trait ZipWithNextExt: Iterator {
    fn zip_with_next(self) -> ZipWithNext<Self, Self::Item>
    where
        Self::Item: Clone + Copy,
        Self: Sized,
    {
        ZipWithNext {
            iter: self.peekable(),
        }
    }
}

impl<I: Iterator> ZipWithNextExt for I {}

pub struct ZipWithNextN<Iter, Item>
where
    Iter: Iterator<Item = Item>,
    Item: Clone + Copy,
{
    n: usize,
    past: Option<VecDeque<Item>>,
    iter: Iter,
}

impl<Iter, Item> Iterator for ZipWithNextN<Iter, Item>
where
    Iter: Iterator<Item = Item>,
    Item: Clone + Copy,
{
    type Item = Vec<Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.past {
            Some(p) => {
                let next = self.iter.next()?;
                p.pop_front();
                p.push_back(next);

                let past_vec = p.iter().cloned().collect::<Vec<_>>();

                Some(past_vec)
            }
            None => {
                let mut p = VecDeque::with_capacity(self.n);
                for _ in 0..self.n {
                    p.push_back(self.iter.next().unwrap());
                }

                let past_vec = p.iter().cloned().collect::<Vec<_>>();

                self.past = Some(p);

                Some(past_vec)
            }
        }
    }
}

pub trait ZipWithNextNExt: Iterator {
    fn zip_with_next_n(self, n: usize) -> ZipWithNextN<Self, Self::Item>
    where
        Self::Item: Clone + Copy,
        Self: Sized,
    {
        ZipWithNextN {
            n,
            past: None,
            iter: self,
        }
    }
}

impl<I: Iterator> ZipWithNextNExt for I {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl Direction {
    fn from_str(string: &str) -> Direction {
        match string {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Unknown direction: {}", string),
        }
    }
}

pub struct DirectionCommand {
    pub direction: Direction,
    pub amount: i32,
}

impl DirectionCommand {
    pub fn new(cmd: String) -> Self {
        let (left, right) = cmd.split_once(' ').unwrap();
        Self {
            direction: Direction::from_str(left),
            amount: right.parse::<i32>().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
