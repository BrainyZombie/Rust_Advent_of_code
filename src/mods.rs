mod aoc_2022;
mod aoc_helpers;
use aoc_2022::day1_1;

type MainFn<T> = fn(Box<T>);

fn noop<T>(_: Box<T>)
where
    T: Iterator<Item = String> + ?Sized,
{
}
pub const MAINS: [MainFn<dyn Iterator<Item = String>>; 100] = {
    let mut arr: [MainFn<dyn Iterator<Item = String>>; 100] = [noop; 100];
    arr[0] = day1_1::main;
    arr
};
