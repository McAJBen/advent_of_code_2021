use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Probe {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl Probe {
    fn progress(&self) -> Self {
        let x = self.x + self.velocity_x;
        let y = self.y + self.velocity_y;

        let velocity_x = match self.velocity_x.cmp(&0) {
            std::cmp::Ordering::Less => self.velocity_x + 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => self.velocity_x - 1,
        };
        let velocity_y = self.velocity_y - 1;

        Probe {
            x,
            y,
            velocity_x,
            velocity_y,
        }
    }

    fn hits_target(&self, target: &TargetRange) -> bool {
        let mut probe = self.clone();

        loop {
            if probe.past_target(target) {
                return false;
            }

            if probe.in_target(target) {
                return true;
            }

            probe = probe.progress();
        }
    }

    fn in_target(&self, target: &TargetRange) -> bool {
        self.x >= target.min_x
            && self.x <= target.max_x
            && self.y >= target.min_y
            && self.y <= target.max_y
    }

    fn past_target(&self, target: &TargetRange) -> bool {
        self.x > target.max_x || self.y < target.min_y
    }
}

#[derive(Debug)]
struct TargetRange {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl TargetRange {
    fn new(input: &str) -> Self {
        // target area: x=20..30, y=-10..-5
        let min_x_regex = Regex::new(r"x=-?\d+").unwrap();
        let max_x_regex = Regex::new(r"..-?\d+").unwrap();
        let min_y_regex = Regex::new(r"y=-?\d+").unwrap();
        let max_y_regex = Regex::new(r"..-?\d+").unwrap();

        let m = min_x_regex.find(input).unwrap();
        let min_x = input[m.start() + 2..m.end()].parse::<i32>().unwrap();
        let m = max_x_regex.find_at(input, m.end()).unwrap();
        let max_x = input[m.start() + 2..m.end()].parse::<i32>().unwrap();
        let m = min_y_regex.find_at(input, m.end()).unwrap();
        let min_y = input[m.start() + 2..m.end()].parse::<i32>().unwrap();
        let m = max_y_regex.find_at(input, m.end()).unwrap();
        let max_y = input[m.start() + 2..m.end()].parse::<i32>().unwrap();

        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

fn main() {
    let input = read_to_string("puzzle_17_input").unwrap();

    let value = TargetRange::new(&input);

    let mut probes = Vec::new();

    for velocity_x in 0..1000 {
        for velocity_y in -1000..1000 {
            let probe = Probe {
                x: 0,
                y: 0,
                velocity_x,
                velocity_y,
            };

            if probe.hits_target(&value) {
                probes.push(probe);
            }
        }
    }

    let num_probes = probes.len();

    assert_eq!(2118, num_probes);

    println!("{}", num_probes);
}
