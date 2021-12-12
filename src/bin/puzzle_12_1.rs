use std::{collections::HashMap, fs::read_to_string};

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
}

fn main() {
    let input = read_to_string("puzzle_12_input").unwrap();

    let graph = Graph::new(&input);

    let paths = graph.get_all_paths();

    let total = paths.len();

    assert_eq!(4011, total);

    println!("{}", total);
}
