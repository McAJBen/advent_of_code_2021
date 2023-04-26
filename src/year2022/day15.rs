use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
struct Sensor {
    sensor: Point,
    manhattan_distance: u32,
}

impl Sensor {
    fn from_line(line: &str) -> Self {
        let segments = line
            .strip_prefix("Sensor at ")
            .unwrap()
            .split_once(": closest beacon is at ")
            .unwrap();

        let sensor = segments
            .0
            .strip_prefix("x=")
            .unwrap()
            .split_once(", y=")
            .unwrap();

        let sensor_x: i32 = sensor.0.parse().unwrap();
        let sensor_y: i32 = sensor.1.parse().unwrap();

        let beacon = segments
            .1
            .strip_prefix("x=")
            .unwrap()
            .split_once(", y=")
            .unwrap();

        let beacon_x: i32 = beacon.0.parse().unwrap();
        let beacon_y: i32 = beacon.1.parse().unwrap();

        let manhattan_distance = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);

        Self {
            sensor: Point {
                x: sensor_x as u32,
                y: sensor_y as u32,
            },
            manhattan_distance,
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    /// returns a list of all points where self.distance(point) is manhattan_radius
    fn get_circumphrance_points(&self, manhattan_radius: u32) -> Vec<Point> {
        let mut points = Vec::new();
        for d in 0..manhattan_radius {
            points.extend([
                Point {
                    x: self.x - d + manhattan_radius,
                    y: self.y + d,
                },
                Point {
                    x: self.x + d - manhattan_radius,
                    y: self.y - d,
                },
                Point {
                    x: self.x - d,
                    y: self.y - d + manhattan_radius,
                },
                Point {
                    x: self.x + d,
                    y: self.y + d - manhattan_radius,
                },
            ])
        }
        points
    }
}

pub fn part1(input: &str) -> u64 {
    const Y: u32 = 2000000;
    let sensors: Vec<Sensor> = input.lines().map(|line| Sensor::from_line(line)).collect();

    let min_x = sensors
        .iter()
        .map(|sensor| sensor.sensor.x as i32 - sensor.manhattan_distance as i32)
        .min()
        .unwrap();

    let max_x = sensors
        .iter()
        .map(|sensor| sensor.sensor.x as i32 + sensor.manhattan_distance as i32)
        .max()
        .unwrap();

    let row_size = (max_x + 1 - min_x) as usize;

    let mut row = vec![false; row_size];
    let offset = min_x as u32;

    for sensor in sensors {
        let radius = sensor
            .manhattan_distance
            .saturating_sub(sensor.sensor.y.abs_diff(Y));
        let min_x = (sensor.sensor.x - radius - offset) as usize;
        let max_x = (sensor.sensor.x + radius - offset) as usize;
        row[min_x..max_x].fill(true);
    }

    row.into_iter()
        .fold(0, |acc, c| if c { acc + 1 } else { acc })
}

pub fn part2(input: &str) -> i64 {
    const MAX: u32 = 4000000;
    let sensors: Vec<Sensor> = input.lines().map(|line| Sensor::from_line(line)).collect();

    let beacon = sensors
        .par_iter()
        .find_map_any(|sensor| {
            sensor
                .sensor
                .get_circumphrance_points(sensor.manhattan_distance + 1)
                .into_iter()
                .find(|point| {
                    point.x <= MAX
                        && point.y <= MAX
                        && sensors
                            .iter()
                            .all(|sensor| sensor.sensor.distance(point) > sensor.manhattan_distance)
                })
        })
        .unwrap();

    beacon.x as i64 * MAX as i64 + beacon.y as i64
}
