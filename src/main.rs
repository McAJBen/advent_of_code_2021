use clap::Parser;
use std::time::Instant;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    year: Option<u16>,
    #[clap(short, long)]
    day: Option<u8>,
    #[clap(short, long)]
    part: Option<u8>,
}

fn main() {
    let args = Args::parse();

    let Args { year, day, part } = args;

    macro_rules! matches_args {
        ($year:expr, $year_i:ident, $day:expr, $day_i:ident, $part:expr, $part_i:ident) => {
            if year.unwrap_or($year) == $year
                && day.unwrap_or($day) == $day
                && part.unwrap_or($part) == $part
            {
                let start = Instant::now();
                advent_of_code::$year_i::$day_i::$part_i();
                let duration = Instant::now() - start;
                println!("year{} day{:02} part{}: {:?}", $year, $day, $part, duration);
            }
        };
    }

    matches_args!(2019, year2019, 1, day01, 1, part1);
    matches_args!(2019, year2019, 1, day01, 2, part2);
    matches_args!(2019, year2019, 2, day02, 1, part1);
    matches_args!(2019, year2019, 2, day02, 2, part2);

    matches_args!(2021, year2021, 1, day01, 1, part1);
    matches_args!(2021, year2021, 1, day01, 2, part2);
    matches_args!(2021, year2021, 2, day02, 1, part1);
    matches_args!(2021, year2021, 2, day02, 2, part2);
    matches_args!(2021, year2021, 3, day03, 1, part1);
    matches_args!(2021, year2021, 3, day03, 2, part2);
    matches_args!(2021, year2021, 4, day04, 1, part1);
    matches_args!(2021, year2021, 4, day04, 2, part2);
    matches_args!(2021, year2021, 5, day05, 1, part1);
    matches_args!(2021, year2021, 5, day05, 2, part2);
    matches_args!(2021, year2021, 6, day06, 1, part1);
    matches_args!(2021, year2021, 6, day06, 2, part2);
    matches_args!(2021, year2021, 7, day07, 1, part1);
    matches_args!(2021, year2021, 7, day07, 2, part2);
    matches_args!(2021, year2021, 8, day08, 1, part1);
    matches_args!(2021, year2021, 8, day08, 2, part2);
    matches_args!(2021, year2021, 9, day09, 1, part1);
    matches_args!(2021, year2021, 9, day09, 2, part2);
    matches_args!(2021, year2021, 10, day10, 1, part1);
    matches_args!(2021, year2021, 10, day10, 2, part2);
    matches_args!(2021, year2021, 11, day11, 1, part1);
    matches_args!(2021, year2021, 11, day11, 2, part2);
    matches_args!(2021, year2021, 12, day12, 1, part1);
    matches_args!(2021, year2021, 12, day12, 2, part2);
    matches_args!(2021, year2021, 13, day13, 1, part1);
    matches_args!(2021, year2021, 13, day13, 2, part2);
    matches_args!(2021, year2021, 14, day14, 1, part1);
    matches_args!(2021, year2021, 14, day14, 2, part2);
    matches_args!(2021, year2021, 15, day15, 1, part1);
    matches_args!(2021, year2021, 15, day15, 2, part2);
    matches_args!(2021, year2021, 16, day16, 1, part1);
    matches_args!(2021, year2021, 16, day16, 2, part2);
    matches_args!(2021, year2021, 17, day17, 1, part1);
    matches_args!(2021, year2021, 17, day17, 2, part2);
    matches_args!(2021, year2021, 18, day18, 1, part1);
    matches_args!(2021, year2021, 18, day18, 2, part2);
    matches_args!(2021, year2021, 19, day19, 1, part1);
    matches_args!(2021, year2021, 19, day19, 2, part2);
    matches_args!(2021, year2021, 20, day20, 1, part1);
    matches_args!(2021, year2021, 20, day20, 2, part2);
    matches_args!(2021, year2021, 21, day21, 1, part1);
    matches_args!(2021, year2021, 21, day21, 2, part2);
    matches_args!(2021, year2021, 22, day22, 1, part1);
    matches_args!(2021, year2021, 22, day22, 2, part2);
    // matches_args!(2021, year2021, 23, day23, 1, part1);
    // matches_args!(2021, year2021, 23, day23, 2, part2);
    // matches_args!(2021, year2021, 24, day24, 1, part1);
    // matches_args!(2021, year2021, 24, day24, 2, part2);
    matches_args!(2021, year2021, 25, day25, 1, part1);

    matches_args!(2022, year2022, 1, day01, 1, part1);
    matches_args!(2022, year2022, 1, day01, 2, part2);
    matches_args!(2022, year2022, 2, day02, 1, part1);
    matches_args!(2022, year2022, 2, day02, 2, part2);
    matches_args!(2022, year2022, 3, day03, 1, part1);
    matches_args!(2022, year2022, 3, day03, 2, part2);
    matches_args!(2022, year2022, 4, day04, 1, part1);
    matches_args!(2022, year2022, 4, day04, 2, part2);
    matches_args!(2022, year2022, 5, day05, 1, part1);
    matches_args!(2022, year2022, 5, day05, 2, part2);

    println!("{:?}", args);
}
