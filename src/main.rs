use roll4::{Damage, Rollable};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let d = args[1].parse::<Damage>().unwrap_or_else(|e| {
        println!("Invalid token: {}", e);
        process::exit(1)
    });
    let result = d.roll();
    println!("{}({:?})", result, result);
}
