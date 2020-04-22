use critfail::{RollExp, Rollable};
use std::env;
use std::process;
mod gui;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let d = args[1].parse::<RollExp>().unwrap_or_else(|e| {
            println!("Invalid token: {}", e);
            process::exit(1)
        });
        let result = d.roll();
        println!("{:?}\n{}", result, result);
    } else {
        gui::run();
    }
}
