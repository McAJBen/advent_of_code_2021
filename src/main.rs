use advent_of_code::{
    solvers::{Solver, SolverTrait},
    utils::TestCase,
};
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
    #[clap(short, long)]
    test: Option<String>,
}

fn run<const YEAR: u16, const DAY: u8, const PART: u8>(args: &Args, solver: Solver<YEAR, DAY, PART>)
where
    Solver<YEAR, DAY, PART>: SolverTrait,
{
    if args.year.unwrap_or(solver.year()) == solver.year()
        && args.day.unwrap_or(solver.day()) == solver.day()
        && args.part.unwrap_or(solver.part()) == solver.part()
    {
        for test_case in TestCase::from_dir(solver.year(), solver.day(), solver.part()) {
            if args
                .test
                .as_ref()
                .map_or(true, |test| test.eq(test_case.name()))
            {
                println!(
                    "year {:04} day {:02} part: {} test: {}",
                    solver.year(),
                    solver.day(),
                    solver.part(),
                    test_case.name(),
                );
                let start = Instant::now();
                let result = solver.solve(test_case.input());
                let duration = Instant::now() - start;
                println!("  duration: {:?}\n    {}", duration, result.to_string());
                assert_eq!(test_case.output(), result.to_string());
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    run(&args, Solver::<2019, 1, 1>);
    run(&args, Solver::<2019, 1, 2>);
    run(&args, Solver::<2019, 2, 1>);
    run(&args, Solver::<2019, 2, 2>);

    run(&args, Solver::<2021, 1, 1>);
    run(&args, Solver::<2021, 1, 2>);
    run(&args, Solver::<2021, 2, 1>);
    run(&args, Solver::<2021, 2, 2>);
    run(&args, Solver::<2021, 3, 1>);
    run(&args, Solver::<2021, 3, 2>);
    run(&args, Solver::<2021, 4, 1>);
    run(&args, Solver::<2021, 4, 2>);
    run(&args, Solver::<2021, 5, 1>);
    run(&args, Solver::<2021, 5, 2>);
    run(&args, Solver::<2021, 6, 1>);
    run(&args, Solver::<2021, 6, 2>);
    run(&args, Solver::<2021, 7, 1>);
    run(&args, Solver::<2021, 7, 2>);
    run(&args, Solver::<2021, 8, 1>);
    run(&args, Solver::<2021, 8, 2>);
    run(&args, Solver::<2021, 9, 1>);
    run(&args, Solver::<2021, 9, 2>);
    run(&args, Solver::<2021, 10, 1>);
    run(&args, Solver::<2021, 10, 2>);
    run(&args, Solver::<2021, 11, 1>);
    run(&args, Solver::<2021, 11, 2>);
    run(&args, Solver::<2021, 12, 1>);
    run(&args, Solver::<2021, 12, 2>);
    run(&args, Solver::<2021, 13, 1>);
    run(&args, Solver::<2021, 13, 2>);
    run(&args, Solver::<2021, 14, 1>);
    run(&args, Solver::<2021, 14, 2>);
    run(&args, Solver::<2021, 15, 1>);
    run(&args, Solver::<2021, 15, 2>);
    run(&args, Solver::<2021, 16, 1>);
    run(&args, Solver::<2021, 16, 2>);
    run(&args, Solver::<2021, 17, 1>);
    run(&args, Solver::<2021, 17, 2>);
    run(&args, Solver::<2021, 18, 1>);
    run(&args, Solver::<2021, 18, 2>);
    run(&args, Solver::<2021, 19, 1>);
    run(&args, Solver::<2021, 19, 2>);
    run(&args, Solver::<2021, 20, 1>);
    run(&args, Solver::<2021, 20, 2>);
    run(&args, Solver::<2021, 21, 1>);
    run(&args, Solver::<2021, 21, 2>);
    run(&args, Solver::<2021, 22, 1>);
    run(&args, Solver::<2021, 22, 2>);
    run(&args, Solver::<2021, 23, 1>);
    // run(&args, Solver::<2021, 23, 2>);
    // run(&args, Solver::<2021, 24, 1>);
    // run(&args, Solver::<2021, 24, 2>);
    run(&args, Solver::<2021, 25, 1>);

    run(&args, Solver::<2022, 1, 1>);
    run(&args, Solver::<2022, 1, 2>);
    run(&args, Solver::<2022, 2, 1>);
    run(&args, Solver::<2022, 2, 2>);
    run(&args, Solver::<2022, 3, 1>);
    run(&args, Solver::<2022, 3, 2>);
    run(&args, Solver::<2022, 4, 1>);
    run(&args, Solver::<2022, 4, 2>);
    run(&args, Solver::<2022, 5, 1>);
    run(&args, Solver::<2022, 5, 2>);
    run(&args, Solver::<2022, 6, 1>);
    run(&args, Solver::<2022, 6, 2>);
    run(&args, Solver::<2022, 7, 1>);
    run(&args, Solver::<2022, 7, 2>);
    run(&args, Solver::<2022, 8, 1>);
    run(&args, Solver::<2022, 8, 2>);
    run(&args, Solver::<2022, 9, 1>);
    run(&args, Solver::<2022, 9, 2>);
    run(&args, Solver::<2022, 10, 1>);
    run(&args, Solver::<2022, 10, 2>);
    run(&args, Solver::<2022, 11, 1>);
    run(&args, Solver::<2022, 11, 2>);
    run(&args, Solver::<2022, 12, 1>);
    run(&args, Solver::<2022, 12, 2>);
    run(&args, Solver::<2022, 13, 1>);
    run(&args, Solver::<2022, 13, 2>);
    run(&args, Solver::<2022, 14, 1>);
    run(&args, Solver::<2022, 14, 2>);
    run(&args, Solver::<2022, 15, 1>);
    run(&args, Solver::<2022, 15, 2>);
    run(&args, Solver::<2022, 16, 1>);
    run(&args, Solver::<2022, 16, 2>);
    run(&args, Solver::<2022, 17, 1>);
    run(&args, Solver::<2022, 17, 2>);
    run(&args, Solver::<2022, 18, 1>);
    run(&args, Solver::<2022, 18, 2>);
    run(&args, Solver::<2022, 19, 1>);
    run(&args, Solver::<2022, 19, 2>);
    run(&args, Solver::<2022, 20, 1>);
    run(&args, Solver::<2022, 20, 2>);
    run(&args, Solver::<2022, 21, 1>);
    run(&args, Solver::<2022, 21, 2>);
    run(&args, Solver::<2022, 22, 1>);
    run(&args, Solver::<2022, 22, 2>);
    run(&args, Solver::<2022, 23, 1>);
    run(&args, Solver::<2022, 23, 2>);
    run(&args, Solver::<2022, 24, 1>);
    run(&args, Solver::<2022, 24, 2>);
    run(&args, Solver::<2022, 25, 1>);

    run(&args, Solver::<2023, 1, 1>);
    run(&args, Solver::<2023, 1, 2>);
    run(&args, Solver::<2023, 2, 1>);
    run(&args, Solver::<2023, 2, 2>);
}
