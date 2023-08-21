mod aoc_2022;
mod aoc_helpers;
use aoc_2022::day1_1;
use aoc_2022::day1_2;
use aoc_2022::day2_1;
use aoc_2022::day2_2;

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
    arr
};
