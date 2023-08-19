use std::env;

mod mods;
// Add more module imports for each file you created

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a file number as an argument (e.g., 01, 02, ...)");
        return;
    }

    let mains = mods::MODS;

    match args[1].parse::<usize>() {
        Ok(num) => {
            if num <= 99 {
                let func: fn(Vec<String>) = mains[num];
                func(args[1..].to_vec());
            } else {
                println!("The number must be between 0 and 99.")
            }
        }
        Err(_) => {
            println!("Invalid integer argument provided.")
        }
    };
}
