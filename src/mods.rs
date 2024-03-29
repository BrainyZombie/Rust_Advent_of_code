mod aoc_2022;
mod aoc_helpers;
use aoc_2022::day01_1;
use aoc_2022::day01_2;
use aoc_2022::day02_1;
use aoc_2022::day02_2;
use aoc_2022::day03_1;
use aoc_2022::day03_2;
use aoc_2022::day04_1;
use aoc_2022::day04_2;
use aoc_2022::day05_1;
use aoc_2022::day05_2;
use aoc_2022::day06_1;
use aoc_2022::day06_2;
use aoc_2022::day07_1;
use aoc_2022::day07_2;
use aoc_2022::day08_1;
use aoc_2022::day08_2;
use aoc_2022::day09_1;
use aoc_2022::day09_2;
use aoc_2022::day10_1;
use aoc_2022::day10_2;
use aoc_2022::day11_1;
use aoc_2022::day11_2;
use aoc_2022::day12_1;
use aoc_2022::day12_2;
use aoc_2022::day13_1;
use aoc_2022::day13_2;
use aoc_2022::day14_1;
use aoc_2022::day14_2;
use aoc_2022::day15_1;
use aoc_2022::day15_2;
use aoc_2022::day16_1;
use aoc_2022::day16_2;
use aoc_2022::day17_1;
use aoc_2022::day17_2;

type MainFn<T> = fn(Box<T>);

fn noop<T>(_: Box<T>)
where
    T: Iterator<Item = String> + ?Sized,
{
}
pub const MAINS: [MainFn<dyn Iterator<Item = String>>; 100] = {
    let mut arr: [MainFn<dyn Iterator<Item = String>>; 100] = [noop; 100];
    arr[00] = day01_1::main;
    arr[01] = day01_2::main;
    arr[02] = day02_1::main;
    arr[03] = day02_2::main;
    arr[04] = day03_1::main;
    arr[05] = day03_2::main;
    arr[06] = day04_1::main;
    arr[07] = day04_2::main;
    arr[08] = day05_1::main;
    arr[09] = day05_2::main;
    arr[10] = day06_1::main;
    arr[11] = day06_2::main;
    arr[12] = day07_1::main;
    arr[13] = day07_2::main;
    arr[14] = day08_1::main;
    arr[15] = day08_2::main;
    arr[16] = day09_1::main;
    arr[17] = day09_2::main;
    arr[18] = day10_1::main;
    arr[19] = day10_2::main;
    arr[20] = day11_1::main;
    arr[21] = day11_2::main;
    arr[22] = day12_1::main;
    arr[23] = day12_2::main;
    arr[24] = day13_1::main;
    arr[25] = day13_2::main;
    arr[26] = day14_1::main;
    arr[27] = day14_2::main;
    arr[28] = day15_1::main;
    arr[29] = day15_2::main;
    arr[30] = day16_1::main;
    arr[31] = day16_2::main;
    arr[32] = day17_1::main;
    arr[33] = day17_2::main;
    arr
};
