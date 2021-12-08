use std::{
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
    Item: Clone,
{
    iter: Peekable<Iter>,
}

impl<Iter, Item> ZipWithNext<Iter, Item>
where
    Iter: Iterator<Item = Item>,
    Item: Clone,
{
    pub fn new(iter: Iter) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }
}

impl<Iter, Item> Iterator for ZipWithNext<Iter, Item>
where
    Iter: Iterator<Item = Item>,
    Item: Clone,
{
    type Item = (Item, Item);

    fn next(&mut self) -> Option<Self::Item> {
        let left = self.iter.next();
        let right = self.iter.peek();

        match (left, right) {
            (Some(left), Some(right)) => Some((left, right.clone())),
            _ => None,
        }
    }
}

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
