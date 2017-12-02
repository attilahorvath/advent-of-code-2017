extern crate day02;

use std::fs::File;
use std::io::{BufRead, BufReader};

use day02::*;

fn main() {
    let file = BufReader::new(File::open("input.txt").expect("file not found"));
    let mut rows: Vec<Vec<u32>> = Vec::new();

    for line in file.lines() {
        let line = line.expect("error reading file");

        rows.push(
            line.split_whitespace()
                .map(|i| i.parse().unwrap_or(0))
                .collect(),
        );
    }

    println!("Checksum: {}", calculate_checksum(&rows));
    println!("Second checksum: {}", calculate_second_checksum(&rows));
}
