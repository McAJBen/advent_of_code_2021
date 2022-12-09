use crate::utils::read_input;

pub fn part1(input: &str) -> u64 {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let height = grid.len();
    let width = grid.len();

    let mut visibility_grid = vec![vec![false; width]; height];

    for x in 0..width {
        visibility_grid[0][x] = true;
        let mut h = grid[0][x];
        for y in 1..height {
            if grid[y][x] > h {
                visibility_grid[y][x] = true;
                h = grid[y][x];
            }
        }
        visibility_grid[height - 1][x] = true;
        let mut h = grid[height - 1][x];
        for y in (0..(height - 1)).rev() {
            if grid[y][x] > h {
                visibility_grid[y][x] = true;
                h = grid[y][x];
            }
        }
    }
    for y in 0..height {
        visibility_grid[y][0] = true;
        let mut h = grid[y][0];
        for x in 1..width {
            if grid[y][x] > h {
                visibility_grid[y][x] = true;
                h = grid[y][x];
            }
        }
        visibility_grid[y][width - 1] = true;
        let mut h = grid[y][width - 1];
        for x in (0..(width - 1)).rev() {
            if grid[y][x] > h {
                visibility_grid[y][x] = true;
                h = grid[y][x];
            }
        }
    }

    dbg!(&visibility_grid);

    visibility_grid.iter().flatten().filter(|v| **v).count() as u64
}

#[test]
fn test_part1() {
    let input = read_input(2022, 8);
    assert_eq!(part1(&input), 1851);
}

pub fn part2(input: &str) -> u64 {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let height = grid.len();
    let width = grid.len();

    let mut max = 0;

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let current = grid[y][x];
            let left = (0..x)
                .rev()
                .position(|x| grid[y][x] >= current)
                .map(|i| i + 1)
                .unwrap_or(x);
            let right = (x + 1..width)
                .position(|x| grid[y][x] >= current)
                .map(|i| i + 1)
                .unwrap_or(width - x - 1);
            let up = (0..y)
                .rev()
                .position(|y| grid[y][x] >= current)
                .map(|i| i + 1)
                .unwrap_or(y);
            let down = (y + 1..height)
                .position(|y| grid[y][x] >= current)
                .map(|i| i + 1)
                .unwrap_or(height - y - 1);

            let score = left * right * up * down;

            max = max.max(score as u64);
        }
    }

    max
}

#[test]
fn test_part2() {
    let input = read_input(2022, 8);
    assert_eq!(part2(&input), 574080);
}
