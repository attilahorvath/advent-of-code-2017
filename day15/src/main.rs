extern crate day15;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day15::*;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("file not found"));
    let mut lines = file.lines();

    let generator_a_previous = lines
        .next()
        .expect("error reading file")
        .expect("error reading file")
        .split_whitespace()
        .last()
        .expect("starting value not present")
        .parse()
        .expect("cannot parse starting value");

    let generator_b_previous = lines
        .next()
        .expect("error reading file")
        .expect("error reading file")
        .split_whitespace()
        .last()
        .expect("starting value not present")
        .parse()
        .expect("cannot parse starting value");

    let judge = Judge::new(generator_a_previous, generator_b_previous);

    println!("Matches found: {}", judge.count_matches(40_000_000));
}
