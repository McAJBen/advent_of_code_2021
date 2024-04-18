use std::{
    collections::VecDeque,
    fs::{read_dir, read_to_string},
    iter::Peekable,
    str::FromStr,
};

#[derive(Debug)]
pub struct TestCase {
    name: String,
    input: String,
    output: String,
}

impl TestCase {
    pub fn from_dir(year: u16, day: u8, part: u8) -> Vec<Self> {
        read_dir(format!("data/{year:04}/{day:02}"))
            .unwrap()
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .map(|path| TestCase {
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                input: read_to_string(path.join("input.txt")).unwrap(),
                output: read_to_string(path.join(format!("output{part}.txt"))).unwrap(),
            })
            .collect()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn output(&self) -> &str {
        &self.output
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (x, y) = input.split_once(',').unwrap();

        Ok(Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}
