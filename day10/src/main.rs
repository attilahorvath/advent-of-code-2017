extern crate day10;

use std::fs::File;
use std::io::prelude::*;

use day10::*;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect(
        "error reading file",
    );

    let lengths = contents
        .split(',')
        .map(|i| i.trim().parse().unwrap_or(0))
        .collect::<Vec<_>>();

    let hash = hash(256, &lengths);
    println!("Product of first two elements: {}", hash[0] * hash[1]);
}
