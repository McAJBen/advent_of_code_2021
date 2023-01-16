use crate::utils::ZipWithNextExt;

pub fn part1(input: &str) -> u32 {
    let points: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let max_x = points.iter().flatten().map(|(x, _)| *x).max().unwrap();
    let max_y = points.iter().flatten().map(|(_, y)| *y).max().unwrap();

    let mut board = vec![vec![false; max_x + 1]; max_y + 1];

    for points in points {
        for ((a_x, a_y), (b_x, b_y)) in points.into_iter().zip_with_next() {
            if a_x == b_x {
                if a_y < b_y {
                    for y in a_y..=b_y {
                        board[y][a_x] = true;
                    }
                } else {
                    for y in b_y..=a_y {
                        board[y][a_x] = true;
                    }
                }
            } else if a_y == b_y {
                if a_x < b_x {
                    for x in a_x..=b_x {
                        board[a_y][x] = true;
                    }
                } else {
                    for x in b_x..=a_x {
                        board[a_y][x] = true;
                    }
                }
            }
        }
    }

    for i in 0.. {
        let mut x = 500;
        let mut y = 0;
        loop {
            if y >= max_y {
                return i;
            } else if !board[y + 1][x] {
                y += 1;
            } else if !board[y + 1][x - 1] {
                y += 1;
                x -= 1;
            } else if !board[y + 1][x + 1] {
                y += 1;
                x += 1;
            } else {
                break;
            }
        }

        board[y][x] = true;
    }

    panic!();
}

pub fn part2(input: &str) -> u32 {
    let mut points: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let max_x = points.iter().flatten().map(|(x, _)| *x).max().unwrap();
    let max_y = points.iter().flatten().map(|(_, y)| *y).max().unwrap();

    points.push(vec![(0, max_y + 2), (max_x * 2, max_y + 2)]);

    let max_x = points.iter().flatten().map(|(x, _)| *x).max().unwrap();
    let max_y = points.iter().flatten().map(|(_, y)| *y).max().unwrap();

    let mut board = vec![vec![false; max_x + 1]; max_y + 1];

    for points in points {
        for ((a_x, a_y), (b_x, b_y)) in points.into_iter().zip_with_next() {
            if a_x == b_x {
                if a_y < b_y {
                    for y in a_y..=b_y {
                        board[y][a_x] = true;
                    }
                } else {
                    for y in b_y..=a_y {
                        board[y][a_x] = true;
                    }
                }
            } else if a_y == b_y {
                if a_x < b_x {
                    for x in a_x..=b_x {
                        board[a_y][x] = true;
                    }
                } else {
                    for x in b_x..=a_x {
                        board[a_y][x] = true;
                    }
                }
            }
        }
    }

    for i in 0.. {
        let mut x = 500;
        let mut y = 0;
        loop {
            if y >= max_y {
                return i;
            } else if !board[y + 1][x] {
                y += 1;
            } else if !board[y + 1][x - 1] {
                y += 1;
                x -= 1;
            } else if !board[y + 1][x + 1] {
                y += 1;
                x += 1;
            } else {
                break;
            }
        }

        if x == 500 && y == 0 {
            return i + 1;
        }

        board[y][x] = true;
    }

    panic!();
}
