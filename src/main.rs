use std::env;
use std::process;

mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Provide a day to run when invoking the command. ex: cargo run 1");
        process::exit(1);
    }

    match args[1].as_ref() {
        "1" => day01::run(),
        _ => eprintln!("Invalid day name given"),
    }
}
