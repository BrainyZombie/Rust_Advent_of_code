mod aoc_2022;
mod aoc_helpers;
use aoc_2022::day1_1;
use aoc_2022::day1_2;
use aoc_2022::day2_1;
use aoc_2022::day2_2;
use aoc_2022::day3_1;
use aoc_2022::day3_2;
use aoc_2022::day4_1;
use aoc_2022::day4_2;
use aoc_2022::day5_1;
use aoc_2022::day5_2;
use aoc_2022::day6_1;
use aoc_2022::day6_2;
use aoc_2022::day7_1;
use aoc_2022::day7_2;
use aoc_2022::day8_1;
use aoc_2022::day8_2;

type MainFn<T> = fn(Box<T>);

fn noop<T>(_: Box<T>)
where
    T: Iterator<Item = String> + ?Sized,
{
}
pub const MAINS: [MainFn<dyn Iterator<Item = String>>; 100] = {
    let mut arr: [MainFn<dyn Iterator<Item = String>>; 100] = [noop; 100];
    arr[0] = day1_1::main;
    arr[1] = day1_2::main;
    arr[2] = day2_1::main;
    arr[3] = day2_2::main;
    arr[4] = day3_1::main;
    arr[5] = day3_2::main;
    arr[6] = day4_1::main;
    arr[7] = day4_2::main;
    arr[8] = day5_1::main;
    arr[9] = day5_2::main;
    arr[10] = day6_1::main;
    arr[11] = day6_2::main;
    arr[12] = day7_1::main;
    arr[13] = day7_2::main;
    arr[14] = day8_1::main;
    arr[15] = day8_2::main;
    arr
};
