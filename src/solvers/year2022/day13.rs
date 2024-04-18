use crate::solvers::{Solver, SolverTrait};
use std::{cmp::Ordering, collections::VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Integer(u32),
    List(Vec<Node>),
}

#[derive(Debug)]
enum CodePoint {
    OpenBracket,
    CloseBracket,
    Number(u32),
}

impl Node {
    fn from_str(input: &str) -> Self {
        let mut deque = VecDeque::<CodePoint>::new();

        let mut digits = String::new();
        for c in input.chars() {
            match c {
                '[' => {
                    deque.push_back(CodePoint::OpenBracket);
                }
                ']' => {
                    if !digits.is_empty() {
                        let num = digits.parse().unwrap();
                        digits.clear();
                        deque.push_back(CodePoint::Number(num));
                    }
                    deque.push_back(CodePoint::CloseBracket);
                }
                ',' => {
                    if !digits.is_empty() {
                        let num = digits.parse().unwrap();
                        digits.clear();
                        deque.push_back(CodePoint::Number(num));
                    }
                }
                digit => {
                    digits.push(digit);
                }
            }
        }

        Self::from_deque(&mut deque)
    }

    fn from_deque(input: &mut VecDeque<CodePoint>) -> Self {
        match input.pop_front().unwrap() {
            CodePoint::OpenBracket => {
                let mut vec = Vec::new();
                loop {
                    match input[0] {
                        CodePoint::OpenBracket => vec.push(Self::from_deque(input)),
                        CodePoint::CloseBracket => {
                            input.pop_front();
                            return Node::List(vec);
                        }
                        CodePoint::Number(num) => {
                            vec.push(Node::Integer(num));
                            input.pop_front();
                        }
                    }
                }
            }
            CodePoint::Number(num) => Node::Integer(num),
            CodePoint::CloseBracket => panic!(),
        }
    }

    /// true for right order
    /// false for right order
    fn compare(&self, other: &Node) -> Option<bool> {
        match self {
            Node::Integer(self_num) => match other {
                Node::Integer(other_num) => match self_num.cmp(other_num) {
                    Ordering::Less => Some(true),
                    Ordering::Equal => None,
                    Ordering::Greater => Some(false),
                },
                Node::List(other_list) => Node::List(vec![Node::Integer(*self_num)])
                    .compare(&Node::List(other_list.clone())),
            },
            Node::List(self_list) => match other {
                Node::Integer(other_num) => Node::List(self_list.clone())
                    .compare(&Node::List(vec![Node::Integer(*other_num)])),
                Node::List(other_list) => {
                    let mut i = 0;
                    loop {
                        match (self_list.get(i), other_list.get(i)) {
                            (Some(self_i), Some(other_i)) => {
                                if let Some(x) = self_i.compare(other_i) {
                                    return Some(x);
                                }
                            }
                            (None, Some(_)) => {
                                return Some(true);
                            }
                            (Some(_), None) => {
                                return Some(false);
                            }
                            (None, None) => return None,
                        }
                        i += 1;
                    }
                }
            },
        }
    }
}

impl SolverTrait for Solver<2022, 13, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        input
            .split("\n\n")
            .enumerate()
            .filter_map(|(index, lines)| {
                let (left, right) = lines.split_once('\n').unwrap();
                let left = Node::from_str(left);
                let right = Node::from_str(right);
                if left.compare(&right) == Some(true) {
                    Some(index + 1)
                } else {
                    None
                }
            })
            .sum::<usize>()
    }
}

impl SolverTrait for Solver<2022, 13, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        let mut packets: Vec<Node> = input
            .lines()
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    Some(Node::from_str(line))
                }
            })
            .chain([Node::from_str("[[2]]"), Node::from_str("[[6]]")])
            .collect();

        packets.sort_unstable_by(|a, b| match a.compare(b) {
            Some(is_in_order) => {
                if is_in_order {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            None => Ordering::Equal,
        });

        let pos1 = packets
            .iter()
            .position(|node| node == &Node::List(vec![Node::List(vec![Node::Integer(2)])]))
            .unwrap()
            + 1;

        let pos2 = packets
            .iter()
            .position(|node| node == &Node::List(vec![Node::List(vec![Node::Integer(6)])]))
            .unwrap()
            + 1;

        pos1 * pos2
    }
}
