extern crate day09;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day09::*;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("file not found"));
    let stream = file.lines().next().expect("no stream given").expect("error reading file");

    println!("Total score: {}", parse_score(&stream));
}
