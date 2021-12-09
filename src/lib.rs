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

pub trait ZipWithNextExt: Iterator {
    fn zip_with_next(self) -> ZipWithNext<Self, Self::Item>
    where
        Self::Item: Clone,
        Self: Sized,
    {
        ZipWithNext {
            iter: self.peekable(),
        }
    }
}

impl<I: Iterator> ZipWithNextExt for I {}

// enum LazyInit<U, T> {
//     Uninit(U),
//     Init(T),
// }

// pub struct ZipWithNextN<Iter, Item>
// where
//     Iter: Iterator<Item = Item>,
//     Item: Clone,
// {
//     past: LazyInit<usize, VecDeque<Item>>,
//     iter: Iter,
// }

// impl<Iter, Item> Iterator for ZipWithNextN<Iter, Item>
// where
//     Iter: Iterator<Item = Item>,
//     Item: Clone,
// {
//     type Item = Vec<Item>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let past = match self.past {
//             LazyInit::Uninit(n) => {
//                 let mut past = VecDeque::with_capacity(n);
//                 for _ in 0..n {
//                     past.push_back(self.iter.next()?);
//                 }
//                 LazyInit::Init(past)
//             }
//             LazyInit::Init(ref mut past) => {
//                 let next = self.iter.next();

//                 past.pop_front();
//                 past.push_back(next);

//                 past
//             }
//         };

//         let LazyInit::Init(past) = &mut self.past;

//         let next = self.iter.next();

//         match next {
//             Some(next) => {
//                 past.pop_front();
//                 past.push_back(next);

//                 Some(past.iter().map(|item| item.clone()).collect::<Vec<_>>())
//             }
//             None => None,
//         }
//     }
// }

// pub trait ZipWithNextNExt: Iterator {
//     fn zip_with_next(self, n: usize) -> ZipWithNextN<Self, Self::Item>
//     where
//         Self::Item: Clone,
//         Self: Sized,
//     {
//         ZipWithNextN {
//             past: LazyInit::Uninit(n),
//             iter: self,
//         }
//     }
// }

// impl<I: Iterator> ZipWithNextNExt for I {}

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
