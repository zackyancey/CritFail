use roll4::DamagePart::*;
use roll4::{Damage, Rollable};

fn main() {
    let d: Damage = vec![Dice(4, 8), Modifier(-3), Dice(2, -4)];
    let result = d.roll();
    println!("{}({:?})", result, result);
}
