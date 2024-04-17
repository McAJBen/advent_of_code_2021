#[cfg(test)]
use crate::{solvers::*, utils::read_input};
use paste::paste;

macro_rules! test_method {
    ($year_expr:expr, $day_expr:expr, $part_expr:expr, $expected_output:expr) => {
        paste! {
            #[test]
            fn [<test_ $year_expr _ $day_expr _ $part_expr>]() {
                let input = read_input($year_expr, $day_expr);
                assert_eq!(Solver::<$year_expr, $day_expr, $part_expr>.solve(&input), $expected_output);
            }
        }
    };
}

test_method!(2019, 1, 1, 3403509);
test_method!(2019, 1, 2, 5102369);
test_method!(2019, 2, 1, 3765464);
test_method!(2019, 2, 2, 7610);

test_method!(2021, 1, 1, 1583);
test_method!(2021, 1, 2, 1627);
test_method!(2021, 2, 1, 1989014);
test_method!(2021, 2, 2, 2006917119);
test_method!(2021, 3, 1, 3959450);
test_method!(2021, 3, 2, 7440311);
test_method!(2021, 4, 1, 16674);
test_method!(2021, 4, 2, 7075);
test_method!(2021, 5, 1, 4826);
test_method!(2021, 5, 2, 16793);
test_method!(2021, 6, 1, 389726);
test_method!(2021, 6, 2, 1743335992042);
test_method!(2021, 7, 1, 354129);
test_method!(2021, 7, 2, 98905973);
test_method!(2021, 8, 1, 449);
test_method!(2021, 8, 2, 968175);
test_method!(2021, 9, 1, 594);
test_method!(2021, 9, 2, 858494);
test_method!(2021, 10, 1, 392097);
test_method!(2021, 10, 2, 4263222782);
test_method!(2021, 11, 1, 1691);
test_method!(2021, 11, 2, 216);
test_method!(2021, 12, 1, 4011);
test_method!(2021, 12, 2, 108035);
test_method!(2021, 13, 1, 671);
test_method!(
    2021,
    13,
    2,
    "
###   ##  ###  #  #  ##  ###  #  # #   
#  # #  # #  # #  # #  # #  # # #  #   
#  # #    #  # #### #  # #  # ##   #   
###  #    ###  #  # #### ###  # #  #   
#    #  # #    #  # #  # # #  # #  #   
#     ##  #    #  # #  # #  # #  # ####
"
    .trim_start()
);
test_method!(2021, 14, 1, 2223);
test_method!(2021, 14, 2, 2566282754493);
test_method!(2021, 15, 1, 769);
test_method!(2021, 15, 2, 2963);
test_method!(2021, 16, 1, 967);
test_method!(2021, 16, 2, 12883091136209);
test_method!(2021, 17, 1, 5565);
test_method!(2021, 17, 2, 2118);
test_method!(2021, 18, 1, 4008);
test_method!(2021, 18, 2, 4667);
test_method!(2021, 19, 1, 362);
test_method!(2021, 19, 2, 12204);
test_method!(2021, 20, 1, 5597);
test_method!(2021, 20, 2, 18723);
test_method!(2021, 21, 1, 916083);
test_method!(2021, 21, 2, 49982165861983);
test_method!(2021, 22, 1, 644257);
test_method!(2021, 22, 2, 1235484513229032);
test_method!(2021, 23, 1, 19046);
// test_method!(2021, 23, 2, 47484);
// test_method!(2021, 24, 1, 29989297949519);
// test_method!(2021, 24, 2, 19518121316118);
test_method!(2021, 25, 1, 557);

test_method!(2022, 1, 1, 66186);
test_method!(2022, 1, 2, 196804);
test_method!(2022, 2, 1, 12740);
test_method!(2022, 2, 2, 11980);
test_method!(2022, 3, 1, 8176);
test_method!(2022, 3, 2, 2689);
test_method!(2022, 4, 1, 644);
test_method!(2022, 4, 2, 926);
test_method!(2022, 5, 1, "CFFHVVHNC");
test_method!(2022, 5, 2, "FSZWBPTBG");
test_method!(2022, 6, 1, 1235);
test_method!(2022, 6, 2, 3051);
test_method!(2022, 7, 1, 1367870);
test_method!(2022, 7, 2, 549173);
test_method!(2022, 8, 1, 1851);
test_method!(2022, 8, 2, 574080);
test_method!(2022, 9, 1, 5883);
test_method!(2022, 9, 2, 2367);
test_method!(2022, 10, 1, 14320);
test_method!(
    2022,
    10,
    2,
    "
###...##..###..###..#..#..##..###....##.
#..#.#..#.#..#.#..#.#.#..#..#.#..#....#.
#..#.#....#..#.###..##...#..#.#..#....#.
###..#....###..#..#.#.#..####.###.....#.
#....#..#.#....#..#.#.#..#..#.#....#..#.
#.....##..#....###..#..#.#..#.#.....##..
"
    .trim_start()
);
test_method!(2022, 11, 1, 56350);
test_method!(2022, 11, 2, 13954061248);
test_method!(2022, 12, 1, 380);
test_method!(2022, 12, 2, 375);
test_method!(2022, 13, 1, 6395);
test_method!(2022, 13, 2, 24921);
test_method!(2022, 14, 1, 698);
test_method!(2022, 14, 2, 28594);
test_method!(2022, 15, 1, 5335787);
test_method!(2022, 15, 2, 13673971349056);
test_method!(2022, 16, 1, 1474);
test_method!(2022, 16, 2, 2100);
test_method!(2022, 17, 1, 3168);
// test_method!(2022, 17, 2, 1554117647070);
test_method!(2022, 18, 1, 3500);
test_method!(2022, 18, 2, 2048);
// test_method!(2022, 19, 1, 0);
// test_method!(2022, 19, 2, 0);
// test_method!(2022, 20, 1, 0);
// test_method!(2022, 20, 2, 0);
// test_method!(2022, 21, 1, 0);
// test_method!(2022, 21, 2, 0);
// test_method!(2022, 22, 1, 0);
// test_method!(2022, 22, 2, 0);
// test_method!(2022, 23, 1, 0);
// test_method!(2022, 23, 2, 0);
// test_method!(2022, 24, 1, 0);
// test_method!(2022, 24, 2, 0);
// test_method!(2022, 25, 1, 0);

test_method!(2023, 1, 1, 55386);
test_method!(2023, 1, 2, 54824);
