mod mods;
use mods::mains;
use std::env;

fn get_fn_number(args: &mut env::Args) -> Result<usize, String> {
    args.next()
        .map_or(Err(String::from("Specify function number to run")), |s| {
            s.parse::<usize>()
                .map_err(|e| format!("Unable to parse integer argument: {e}"))
        })
        .map_or_else(Err, |i| {
            if i > 100 {
                Err(String::from("The number must be between 0 and 99."))
            } else {
                Ok(i)
            }
        })
}

fn main() {
    let mut args = env::args();
    args.next();
    let i = match get_fn_number(&mut args) {
        Ok(i) => i,
        Err(str) => {
            println!("{str}");
            return;
        }
    };

    (mains[i])(args)
}
