use crate::solvers::{Solver, SolverTrait};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
    Start,
    End,
    SmallCave(char),
    BigCave(char),
}

impl Node {
    fn from_str(s: &str) -> Node {
        if s == "start" {
            Node::Start
        } else if s == "end" {
            Node::End
        } else if s.to_lowercase() == s {
            Node::SmallCave(s.chars().next().unwrap())
        } else {
            Node::BigCave(s.chars().next().unwrap())
        }
    }

    fn can_revisit(&self) -> bool {
        matches!(self, Node::BigCave(_))
    }

    fn is_small(&self) -> bool {
        matches!(self, Node::SmallCave(_))
    }
}

struct Graph {
    nodes: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let mut nodes = HashMap::new();
        for line in input.lines() {
            let (node1, node2) = line.split_once('-').unwrap();

            let node1 = Node::from_str(node1);
            let node2 = Node::from_str(node2);

            nodes.entry(node1).or_insert_with(Vec::new).push(node2);
            nodes.entry(node2).or_insert_with(Vec::new).push(node1);
        }

        Self { nodes }
    }

    fn get_all_paths(&self) -> Vec<Vec<Node>> {
        let mut found_paths = Vec::new();
        let mut searching_paths = vec![vec![Node::Start]];

        while let Some(searching_path) = searching_paths.pop() {
            let last_node = searching_path.last().unwrap();

            if last_node == &Node::End {
                found_paths.push(searching_path);
            } else {
                let routes = self.nodes.get(last_node).unwrap();
                for route in routes {
                    if route.can_revisit() || !searching_path.contains(route) {
                        let mut new_path = searching_path.clone();
                        new_path.push(*route);
                        searching_paths.push(new_path);
                    }
                }
            }
        }

        found_paths
    }

    fn get_all_paths2(&self) -> Vec<Vec<Node>> {
        let mut found_paths = Vec::new();
        let mut searching_paths = vec![vec![Node::Start]];

        while let Some(searching_path) = searching_paths.pop() {
            let last_node = searching_path.last().unwrap();

            if last_node == &Node::End {
                found_paths.push(searching_path);
            } else {
                let routes = self.nodes.get(last_node).unwrap();
                for route in routes {
                    let num_times_visited =
                        searching_path.iter().filter(|node| node == &route).count();

                    let has_visited_twice = self.nodes.keys().any(|node| {
                        node.is_small() && searching_path.iter().filter(|&n| n == node).count() >= 2
                    });

                    let can_go = match route {
                        Node::BigCave(_) => true,
                        Node::SmallCave(_) => num_times_visited == 0 || !has_visited_twice,
                        Node::Start => false,
                        Node::End => true,
                    };

                    if can_go {
                        let mut new_path = searching_path.clone();
                        new_path.push(*route);
                        searching_paths.push(new_path);
                    }
                }
            }
        }

        found_paths
    }
}

impl SolverTrait for Solver<2021, 12, 1> {
    fn solve(&self, input: &str) -> impl ToString {
        let graph = Graph::new(input);

        let paths = graph.get_all_paths();

        paths.len()
    }
}

impl SolverTrait for Solver<2021, 12, 2> {
    fn solve(&self, input: &str) -> impl ToString {
        let graph = Graph::new(input);

        let paths = graph.get_all_paths2();

        paths.len()
    }
}
