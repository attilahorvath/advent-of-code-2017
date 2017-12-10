extern crate day09;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day09::*;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("file not found"));
    let stream = file.lines().next().expect("no stream given").expect(
        "error reading file",
    );

    let stream_data = parse_stream(&stream);

    println!("Total score: {}", stream_data.score);
    println!("Garbage amount: {}", stream_data.garbage);
}
