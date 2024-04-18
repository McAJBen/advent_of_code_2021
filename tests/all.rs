use advent_of_code::{
    solvers::{Solver, SolverTrait},
    utils::TestCase,
};
use paste::paste;
use std::fmt::Display;

fn run<const YEAR: u16, const DAY: u8, const PART: u8, O: Display>(solver: Solver<YEAR, DAY, PART>)
where
    Solver<YEAR, DAY, PART>: SolverTrait<O>,
{
    for test_case in TestCase::from_dir(YEAR, DAY, PART) {
        assert_eq!(
            solver.solve(test_case.input()).to_string(),
            test_case.output()
        );
    }
}

macro_rules! test_method {
    ($year_expr:expr, $day_expr:expr, $part_expr:expr) => {
        paste! {
            #[test]
            fn [<test_ $year_expr _ $day_expr _ $part_expr>]() {
                run(Solver::<$year_expr, $day_expr, $part_expr>);
            }
        }
    };
}

test_method!(2019, 1, 1);
test_method!(2019, 1, 2);
test_method!(2019, 2, 1);
test_method!(2019, 2, 2);

test_method!(2021, 1, 1);
test_method!(2021, 1, 2);
test_method!(2021, 2, 1);
test_method!(2021, 2, 2);
test_method!(2021, 3, 1);
test_method!(2021, 3, 2);
test_method!(2021, 4, 1);
test_method!(2021, 4, 2);
test_method!(2021, 5, 1);
test_method!(2021, 5, 2);
test_method!(2021, 6, 1);
test_method!(2021, 6, 2);
test_method!(2021, 7, 1);
test_method!(2021, 7, 2);
test_method!(2021, 8, 1);
test_method!(2021, 8, 2);
test_method!(2021, 9, 1);
test_method!(2021, 9, 2);
test_method!(2021, 10, 1);
test_method!(2021, 10, 2);
test_method!(2021, 11, 1);
test_method!(2021, 11, 2);
test_method!(2021, 12, 1);
test_method!(2021, 12, 2);
test_method!(2021, 13, 1);
test_method!(2021, 13, 2);
test_method!(2021, 14, 1);
test_method!(2021, 14, 2);
test_method!(2021, 15, 1);
test_method!(2021, 15, 2);
test_method!(2021, 16, 1);
test_method!(2021, 16, 2);
test_method!(2021, 17, 1);
test_method!(2021, 17, 2);
test_method!(2021, 18, 1);
test_method!(2021, 18, 2);
test_method!(2021, 19, 1);
test_method!(2021, 19, 2);
test_method!(2021, 20, 1);
test_method!(2021, 20, 2);
test_method!(2021, 21, 1);
test_method!(2021, 21, 2);
test_method!(2021, 22, 1);
test_method!(2021, 22, 2);
test_method!(2021, 23, 1);
// test_method!(2021, 23, 2);
// test_method!(2021, 24, 1);
// test_method!(2021, 24, 2);
test_method!(2021, 25, 1);

test_method!(2022, 1, 1);
test_method!(2022, 1, 2);
test_method!(2022, 2, 1);
test_method!(2022, 2, 2);
test_method!(2022, 3, 1);
test_method!(2022, 3, 2);
test_method!(2022, 4, 1);
test_method!(2022, 4, 2);
test_method!(2022, 5, 1);
test_method!(2022, 5, 2);
test_method!(2022, 6, 1);
test_method!(2022, 6, 2);
test_method!(2022, 7, 1);
test_method!(2022, 7, 2);
test_method!(2022, 8, 1);
test_method!(2022, 8, 2);
test_method!(2022, 9, 1);
test_method!(2022, 9, 2);
test_method!(2022, 10, 1);
test_method!(2022, 10, 2);
test_method!(2022, 11, 1);
test_method!(2022, 11, 2);
test_method!(2022, 12, 1);
test_method!(2022, 12, 2);
test_method!(2022, 13, 1);
test_method!(2022, 13, 2);
test_method!(2022, 14, 1);
test_method!(2022, 14, 2);
test_method!(2022, 15, 1);
test_method!(2022, 15, 2);
test_method!(2022, 16, 1);
test_method!(2022, 16, 2);
test_method!(2022, 17, 1);
// test_method!(2022, 17, 2);
test_method!(2022, 18, 1);
test_method!(2022, 18, 2);
// test_method!(2022, 19, 1);
// test_method!(2022, 19, 2);
// test_method!(2022, 20, 1);
// test_method!(2022, 20, 2);
// test_method!(2022, 21, 1);
// test_method!(2022, 21, 2);
// test_method!(2022, 22, 1);
// test_method!(2022, 22, 2);
// test_method!(2022, 23, 1);
// test_method!(2022, 23, 2);
// test_method!(2022, 24, 1);
// test_method!(2022, 24, 2);
// test_method!(2022, 25, 1);

test_method!(2023, 1, 1);
test_method!(2023, 1, 2);
