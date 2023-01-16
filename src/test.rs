#[cfg(test)]
use crate::utils::read_input;
use paste::paste;

macro_rules! test_method {
    ($year:expr, $year_i:ident, $day:expr, $day_i:ident, $part_i:ident, $answer:expr) => {
        paste! {
            #[test]
            fn [<$year_i _ $day_i _ $part_i>]() {
                let input = read_input($year, $day);
                assert_eq!(crate::$year_i::$day_i::$part_i(&input), $answer);
            }
        }
    };
}

test_method!(2019, year2019, 1, day01, part1, 3403509);
test_method!(2019, year2019, 1, day01, part2, 5102369);
test_method!(2019, year2019, 2, day02, part1, 3765464);
test_method!(2019, year2019, 2, day02, part2, 7610);

test_method!(2021, year2021, 1, day01, part1, 1583);
test_method!(2021, year2021, 1, day01, part2, 1627);
test_method!(2021, year2021, 2, day02, part1, 1989014);
test_method!(2021, year2021, 2, day02, part2, 2006917119);
test_method!(2021, year2021, 3, day03, part1, 3959450);
test_method!(2021, year2021, 3, day03, part2, 7440311);
test_method!(2021, year2021, 4, day04, part1, 16674);
test_method!(2021, year2021, 4, day04, part2, 7075);
test_method!(2021, year2021, 5, day05, part1, 4826);
test_method!(2021, year2021, 5, day05, part2, 16793);
test_method!(2021, year2021, 6, day06, part1, 389726);
test_method!(2021, year2021, 6, day06, part2, 1743335992042);
test_method!(2021, year2021, 7, day07, part1, 354129);
test_method!(2021, year2021, 7, day07, part2, 98905973);
test_method!(2021, year2021, 8, day08, part1, 449);
test_method!(2021, year2021, 8, day08, part2, 968175);
test_method!(2021, year2021, 9, day09, part1, 594);
test_method!(2021, year2021, 9, day09, part2, 858494);
test_method!(2021, year2021, 10, day10, part1, 392097);
test_method!(2021, year2021, 10, day10, part2, 4263222782);
test_method!(2021, year2021, 11, day11, part1, 1691);
test_method!(2021, year2021, 11, day11, part2, 216);
test_method!(2021, year2021, 12, day12, part1, 4011);
test_method!(2021, year2021, 12, day12, part2, 108035);
test_method!(2021, year2021, 13, day13, part1, 671);
test_method!(2021, year2021, 13, day13, part2, "###   ##  ###  #  #  ##  ###  #  # #   \n#  # #  # #  # #  # #  # #  # # #  #   \n#  # #    #  # #### #  # #  # ##   #   \n###  #    ###  #  # #### ###  # #  #   \n#    #  # #    #  # #  # # #  # #  #   \n#     ##  #    #  # #  # #  # #  # ####\n");
test_method!(2021, year2021, 14, day14, part1, 2223);
test_method!(2021, year2021, 14, day14, part2, 2566282754493);
test_method!(2021, year2021, 15, day15, part1, 769);
test_method!(2021, year2021, 15, day15, part2, 2963);
test_method!(2021, year2021, 16, day16, part1, 967);
test_method!(2021, year2021, 16, day16, part2, 12883091136209);
test_method!(2021, year2021, 17, day17, part1, 5565);
test_method!(2021, year2021, 17, day17, part2, 2118);
test_method!(2021, year2021, 18, day18, part1, 4008);
test_method!(2021, year2021, 18, day18, part2, 4667);
test_method!(2021, year2021, 19, day19, part1, 362);
test_method!(2021, year2021, 19, day19, part2, 12204);
test_method!(2021, year2021, 20, day20, part1, 5597);
test_method!(2021, year2021, 20, day20, part2, 18723);
test_method!(2021, year2021, 21, day21, part1, 916083);
test_method!(2021, year2021, 21, day21, part2, 49982165861983);
test_method!(2021, year2021, 22, day22, part1, 644257);
test_method!(2021, year2021, 22, day22, part2, 1235484513229032);
// test_method!(2021, year2021, 23, day23, part1, 0);
// test_method!(2021, year2021, 23, day23, part2, 0);
// test_method!(2021, year2021, 24, day24, part1, 0);
// test_method!(2021, year2021, 24, day24, part2, 0);
test_method!(2021, year2021, 25, day25, part1, 557);

test_method!(2022, year2022, 1, day01, part1, 66186);
test_method!(2022, year2022, 1, day01, part2, 196804);
test_method!(2022, year2022, 2, day02, part1, 12740);
test_method!(2022, year2022, 2, day02, part2, 11980);
test_method!(2022, year2022, 3, day03, part1, 8176);
test_method!(2022, year2022, 3, day03, part2, 2689);
test_method!(2022, year2022, 4, day04, part1, 644);
test_method!(2022, year2022, 4, day04, part2, 926);
test_method!(2022, year2022, 5, day05, part1, "CFFHVVHNC");
test_method!(2022, year2022, 5, day05, part2, "FSZWBPTBG");
test_method!(2022, year2022, 6, day06, part1, 1235);
test_method!(2022, year2022, 6, day06, part2, 3051);
test_method!(2022, year2022, 7, day07, part1, 1367870);
test_method!(2022, year2022, 7, day07, part2, 549173);
test_method!(2022, year2022, 8, day08, part1, 1851);
test_method!(2022, year2022, 8, day08, part2, 574080);
test_method!(2022, year2022, 9, day09, part1, 5883);
test_method!(2022, year2022, 9, day09, part2, 2367);
test_method!(2022, year2022, 10, day10, part1, 14320);
test_method!(2022, year2022, 10, day10, part2, "###...##..###..###..#..#..##..###....##.\n#..#.#..#.#..#.#..#.#.#..#..#.#..#....#.\n#..#.#....#..#.###..##...#..#.#..#....#.\n###..#....###..#..#.#.#..####.###.....#.\n#....#..#.#....#..#.#.#..#..#.#....#..#.\n#.....##..#....###..#..#.#..#.#.....##..\n");
test_method!(2022, year2022, 11, day11, part1, 56350);
test_method!(2022, year2022, 11, day11, part2, 13954061248);
test_method!(2022, year2022, 12, day12, part1, 380);
test_method!(2022, year2022, 12, day12, part2, 375);
test_method!(2022, year2022, 13, day13, part1, 6395);
test_method!(2022, year2022, 13, day13, part2, 24921);
test_method!(2022, year2022, 14, day14, part1, 698);
test_method!(2022, year2022, 14, day14, part2, 28594);
// test_method!(2022, year2022, 15, day15, part1, 0);
// test_method!(2022, year2022, 15, day15, part2, 0);
// test_method!(2022, year2022, 16, day16, part1, 0);
// test_method!(2022, year2022, 16, day16, part2, 0);
// test_method!(2022, year2022, 17, day17, part1, 0);
// test_method!(2022, year2022, 17, day17, part2, 0);
// test_method!(2022, year2022, 18, day18, part1, 0);
// test_method!(2022, year2022, 18, day18, part2, 0);
// test_method!(2022, year2022, 19, day19, part1, 0);
// test_method!(2022, year2022, 19, day19, part2, 0);
// test_method!(2022, year2022, 20, day20, part1, 0);
// test_method!(2022, year2022, 20, day20, part2, 0);
// test_method!(2022, year2022, 21, day21, part1, 0);
// test_method!(2022, year2022, 21, day21, part2, 0);
// test_method!(2022, year2022, 22, day22, part1, 0);
// test_method!(2022, year2022, 22, day22, part2, 0);
// test_method!(2022, year2022, 23, day23, part1, 0);
// test_method!(2022, year2022, 23, day23, part2, 0);
// test_method!(2022, year2022, 24, day24, part1, 0);
// test_method!(2022, year2022, 24, day24, part2, 0);
// test_method!(2022, year2022, 25, day25, part1, 0);
