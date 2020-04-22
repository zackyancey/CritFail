use std::env;
use std::process;

use roll4::DamagePart::*;
use roll4::{Damage, Rollable};

fn main() {
    let args: Vec<String> = env::args().collect();
    let d = args[1].parse::<Damage>().unwrap_or_else(|e| {
        println!("Invalid token: {}", e);
        process::exit(1)
    });
    let result = d.roll();
    println!("{}({:?})", result, result);
}
