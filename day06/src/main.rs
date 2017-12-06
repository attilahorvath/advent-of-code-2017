extern crate day06;

use std::fs::File;
use std::io::prelude::*;

use day06::*;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect(
        "error reading file",
    );

    let mut blocks = contents
        .split_whitespace()
        .map(|i| i.parse().unwrap_or(0))
        .collect::<Vec<_>>();

    println!("Max steps: {}", max_steps(&mut blocks));
}
