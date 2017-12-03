extern crate day03;

use day03::*;

const INPUT: u32 = 361527;

fn main() {
    println!("Distance: {}", distance(INPUT));
    println!("Greater value: {}", value_greater_than(INPUT));
}
