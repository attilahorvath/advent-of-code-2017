extern crate day05;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day05::*;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("file not found"));
    let mut offsets = file.lines()
        .map(|i| i.expect("error reading file").parse().unwrap_or(0))
        .collect::<Vec<_>>();

    println!("Number of steps: {}", number_of_steps(&mut offsets));
}
