mod lib;
use crate::mods::aoc_helpers::file_io::file_io;

pub fn main<T: Iterator<Item = String>>(args: T) {
    let args = Box::from(
        vec![
            String::from("assets/2022_2_input.txt"),
            String::from("assets/2022_2_2_output.txt"),
        ]
        .into_iter()
        .chain(args),
    );

    let res = file_io(args, lib::run);
    match res {
        Ok(res) => println!("Result is {res}"),
        Err(res) => println!("Err is {res}"),
    };
}
