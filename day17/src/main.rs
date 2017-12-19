extern crate day17;

use std::fs::File;
use std::io::prelude::*;

use day17::*;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect(
        "error reading file",
    );

    let mut spinlock = Spinlock::new(contents.trim().parse().expect("invalid value"));

    println!("Value after latest: {}", spinlock.value_after_latest(2017));

    println!(
        "Value after zero: {}",
        spinlock.value_after_zero(50_000_000)
    );
}
